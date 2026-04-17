from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
from pathlib import Path

from PIL import Image, ImageEnhance, ImageFilter


class ChafaPipeline:
    PREPROCESS_SIGNATURE = {
        "brightness": 1.00,
        "gamma": 1.02,
        "contrast": 1.02,
        "color": 1.06,
        "blur": 0.25,
        "unsharp_radius": 1.0,
        "unsharp_percent": 95,
        "unsharp_threshold": 2,
    }

    def __init__(self, repo_root: Path, config: dict) -> None:
        self.repo_root = repo_root
        self.config = config["chafa"]

    def _resolve_path(self, value: str) -> Path:
        path = Path(value)
        return path if path.is_absolute() else self.repo_root / path

    def _select_source(self) -> Path:
        source = self._resolve_path(self.config["source_gif"])
        if source.exists():
            return source

        fallback = self._resolve_path(self.config["fallback_image"])
        if fallback.exists():
            return fallback

        raise FileNotFoundError(
            "Neither source GIF nor fallback image exists: "
            f"{source} / {fallback}"
        )

    def _cache_dirs(self) -> tuple[Path, Path]:
        raw_dir = self._resolve_path(self.config["cache_dir_raw"])
        chafa_dir = self._resolve_path(self.config["cache_dir_chafa"])
        raw_dir.mkdir(parents=True, exist_ok=True)
        chafa_dir.mkdir(parents=True, exist_ok=True)
        return raw_dir, chafa_dir

    def _manifest_path(self, chafa_dir: Path) -> Path:
        return chafa_dir / "manifest.json"

    def _cache_signature(self, source: Path) -> str:
        payload = {
            "source": str(source.resolve()),
            "source_mtime_ns": source.stat().st_mtime_ns,
            "source_size": source.stat().st_size,
            "frame_count": self.config["frame_count"],
            "width": self.config["width"],
            "height": self.config["height"],
            "align": self.config["align"],
            "symbols": self.config.get("symbols"),
            "fill": self.config.get("fill"),
            "colors": self.config.get("colors"),
            "color_space": self.config.get("color_space"),
            "color_extractor": self.config.get("color_extractor"),
            "fg_only": self.config.get("fg_only"),
            "bg": self.config.get("bg"),
            "threshold": self.config.get("threshold"),
            "dither": self.config.get("dither"),
            "dither_grain": self.config.get("dither_grain"),
            "dither_intensity": self.config.get("dither_intensity"),
            "optimize": self.config.get("optimize"),
            "preprocess": self.config.get("preprocess"),
            "preprocess_signature": self.PREPROCESS_SIGNATURE,
        }
        digest = hashlib.sha256(
            json.dumps(payload, sort_keys=True, separators=(",", ":")).encode("utf-8")
        ).hexdigest()
        return digest

    def _load_manifest(self, manifest_path: Path) -> dict | None:
        if not manifest_path.exists():
            return None

        try:
            return json.loads(manifest_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            return None

    def _write_manifest(self, manifest_path: Path, payload: dict) -> None:
        manifest_path.write_text(
            json.dumps(payload, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )

    # -------------------- GAMMA --------------------
    def _apply_gamma(self, frame: Image.Image, gamma: float) -> Image.Image:
        if gamma == 1.0:
            return frame

        inv_gamma = 1.0 / gamma
        lut = [
            max(0, min(255, int(round((i / 255.0) ** inv_gamma * 255.0))))
            for i in range(256)
        ]

        if frame.mode == "RGBA":
            rgb = frame.convert("RGB").point(lut * 3)
            alpha = frame.getchannel("A")
            rgb.putalpha(alpha)
            return rgb

        if frame.mode == "RGB":
            return frame.point(lut * 3)

        return frame.point(lut)

    # -------------------- PREPROCESS --------------------
    def _preprocess_frame(self, frame: Image.Image) -> Image.Image:
        sig = self.PREPROCESS_SIGNATURE

        # GLOBAL tone shaping
        frame = ImageEnhance.Brightness(frame).enhance(sig["brightness"])
        frame = self._apply_gamma(frame, gamma=sig["gamma"])
        frame = ImageEnhance.Contrast(frame).enhance(sig["contrast"])
        frame = ImageEnhance.Color(frame).enhance(sig["color"])

        frame = frame.convert("RGB")
        pixels = frame.load()
        w, h = frame.size

        # PASS 1 — stronger dark red separation (quantization-safe)
        for y in range(h):
            for x in range(w):
                r, g, b = pixels[x, y]

                lum = 0.2126 * r + 0.7152 * g + 0.0722 * b

                if (
                    r > g * 1.10 and
                    r > b * 1.10 and
                    55 < lum < 125
                ):
                    # preserve separation without collapsing too many tones into one red slab
                    r = min(255, int(r * 1.10))
                    g = int(g * 0.90)
                    b = int(b * 0.88)

                    # small lift only for the darkest warm reds
                    if lum < 80:
                        r = min(255, int(r * 1.05))
                        g = int(g * 0.98)

                    pixels[x, y] = (r, g, b)

        # PASS 2 — boost dark/midtone separation for braille density
        for y in range(h):
            for x in range(w):
                r, g, b = pixels[x, y]

                lum = 0.2126 * r + 0.7152 * g + 0.0722 * b

                # lift darker regions slightly so they survive fg-only quantization
                if 25 < lum < 110:
                    boost = 1.08
                    r = min(255, int(r * boost))
                    g = min(255, int(g * boost))
                    b = min(255, int(b * boost))

                    # add slight contrast skew (prevents gray collapse)
                    r = min(255, int(r * 1.03))
                    b = min(255, int(b * 0.98))

                    pixels[x, y] = (r, g, b)

        # FINAL smoothing
        frame = frame.filter(ImageFilter.GaussianBlur(radius=sig["blur"]))
        frame = frame.filter(
            ImageFilter.UnsharpMask(
                radius=sig["unsharp_radius"],
                percent=sig["unsharp_percent"],
                threshold=sig["unsharp_threshold"],
            )
        )

        return frame

    # -------------------- FRAME EXTRACTION --------------------
    def _extract_frames(self, source_path: Path, output_dir: Path) -> list[Path]:
        for f in output_dir.glob("frame_*.png"):
            f.unlink()

        with Image.open(source_path) as img:
            total = getattr(img, "n_frames", 1)
            requested = self.config["frame_count"]

            paths = []
            for i in range(requested):
                idx = int(i * total / requested)
                img.seek(idx)
                frame = img.convert("RGBA")

                path = output_dir / f"frame_{i:02d}.png"
                frame.save(path)
                paths.append(path)

        return paths

    # -------------------- CHAFA --------------------
    def _build_chafa_command(self, path: Path) -> list[str]:
        cmd = [
            "chafa",
            "--format=symbols",
            f"--size={self.config['width']}x{self.config['height']}",
            f"--align={self.config['align']}",
        ]

        symbols = self.config.get("symbols")
        if symbols and symbols != "none":
            cmd.append(f"--symbols={symbols}")

        fill = self.config.get("fill")
        if fill and fill != "none":
            cmd.append(f"--fill={fill}")

        colors = self.config.get("colors")
        if colors:
            cmd.append(f"--colors={colors}")

        color_space = self.config.get("color_space")
        if color_space:
            cmd.append(f"--color-space={color_space}")

        color_extractor = self.config.get("color_extractor")
        if color_extractor:
            cmd.append(f"--color-extractor={color_extractor}")

        dither = self.config.get("dither")
        if dither:
            cmd.append(f"--dither={dither}")

        dither_grain = self.config.get("dither_grain")
        if dither_grain:
            cmd.append(f"--dither-grain={dither_grain}")

        dither_intensity = self.config.get("dither_intensity")
        if dither_intensity is not None:
            cmd.append(f"--dither-intensity={dither_intensity}")

        optimize = self.config.get("optimize")
        if optimize is not None:
            cmd.append(f"--optimize={optimize}")

        preprocess = self.config.get("preprocess")
        if preprocess is not None:
            cmd.append(f"--preprocess={preprocess}")

        threshold = self.config.get("threshold")
        if threshold is not None:
            cmd.append(f"--threshold={threshold}")

        bg = self.config.get("bg")
        if bg:
            cmd.append(f"--bg={bg}")

        if self.config.get("fg_only", True):
            cmd.append("--fg-only")

        cmd.append(str(path))
        return cmd

    # -------------------- RENDER --------------------
    def _render_frames(self, raw_paths: list[Path], out_dir: Path) -> list[Path]:
        out_dir.mkdir(parents=True, exist_ok=True)

        rendered = []
        temp_dir = out_dir / "_pre"
        if temp_dir.exists():
            shutil.rmtree(temp_dir)
        temp_dir.mkdir(parents=True, exist_ok=True)

        try:
            for i, raw in enumerate(raw_paths):
                with Image.open(raw) as img:
                    processed = self._preprocess_frame(img)

                tmp = temp_dir / f"frame_{i:02d}.png"
                processed.save(tmp)

                cmd = self._build_chafa_command(tmp)
                result = subprocess.run(cmd, capture_output=True, text=True, check=True)

                out = out_dir / f"frame_{i:02d}.ansi"
                out.write_text(result.stdout.rstrip("\n") + "\n", encoding="utf-8")
                rendered.append(out)
        finally:
            shutil.rmtree(temp_dir, ignore_errors=True)

        return rendered

    # -------------------- PUBLIC --------------------
    def load_frames(self) -> list[list[str]]:
        source = self._select_source()
        raw_dir, chafa_dir = self._cache_dirs()
        manifest_path = self._manifest_path(chafa_dir)
        expected_signature = self._cache_signature(source)
        requested = int(self.config["frame_count"])

        manifest = self._load_manifest(manifest_path)
        if manifest and manifest.get("signature") == expected_signature:
            rendered = sorted(chafa_dir.glob("frame_*.ansi"))
            if len(rendered) == requested:
                return [p.read_text(encoding="utf-8").splitlines() for p in rendered]

        raw = self._extract_frames(source, raw_dir)
        rendered = self._render_frames(raw, chafa_dir)

        self._write_manifest(
            manifest_path,
            {
                "signature": expected_signature,
                "source": str(source.resolve()),
                "frame_count": requested,
            },
        )

        return [p.read_text().splitlines() for p in rendered]
