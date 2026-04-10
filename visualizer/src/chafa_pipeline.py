from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path

from PIL import Image, ImageEnhance


class ChafaPipeline:
    def __init__(self, repo_root: Path, config: dict) -> None:
        self.repo_root = repo_root
        self.config = config["chafa"]

    def load_frames(self) -> list[list[str]]:
        source_path = self._ensure_source_gif()
        raw_dir = self.repo_root / self.config["cache_dir_raw"]
        chafa_dir = self.repo_root / self.config["cache_dir_chafa"]
        raw_dir.mkdir(parents=True, exist_ok=True)
        chafa_dir.mkdir(parents=True, exist_ok=True)

        signature = self._signature_for(source_path)
        manifest_path = chafa_dir / "manifest.json"
        if self._is_cache_valid(manifest_path, signature):
            return self._read_cached_frames(chafa_dir, manifest_path)

        raw_paths = self._extract_frames(source_path, raw_dir)
        rendered_paths = self._render_frames(raw_paths, chafa_dir)
        manifest = {
            "signature": signature,
            "frames": [path.name for path in rendered_paths],
            "frame_count": len(rendered_paths),
        }
        manifest_path.write_text(json.dumps(manifest, indent=2), encoding="utf-8")
        return [path.read_text(encoding="utf-8").splitlines() for path in rendered_paths]

    def _ensure_source_gif(self) -> Path:
        source_value = Path(self.config["source_gif"])
        gif_path = source_value if source_value.is_absolute() else self.repo_root / source_value
        if gif_path.exists():
            return gif_path

        fallback_value = Path(self.config["fallback_image"])
        fallback_path = (
            fallback_value if fallback_value.is_absolute() else self.repo_root / fallback_value
        )
        if not fallback_path.exists():
            raise FileNotFoundError(
                f"Missing source GIF at {gif_path} and fallback image at {fallback_path}"
            )

        if not gif_path.is_absolute():
            gif_path.parent.mkdir(parents=True, exist_ok=True)
        else:
            gif_path = self.repo_root / "assets/source.gif"
            gif_path.parent.mkdir(parents=True, exist_ok=True)
        self._build_fallback_gif(fallback_path, gif_path)
        return gif_path

    def _build_fallback_gif(self, source_image: Path, gif_path: Path) -> None:
        frame_count = self.config["frame_count"]
        base = Image.open(source_image).convert("RGBA")
        frames = []
        for index in range(frame_count):
            phase = index / max(1, frame_count - 1)
            x_shift = int((phase - 0.5) * 16)
            y_shift = int((0.5 - abs(phase - 0.5)) * 8)
            canvas = Image.new("RGBA", base.size, (0, 0, 0, 0))
            canvas.alpha_composite(base, (x_shift, y_shift))
            enhancer = ImageEnhance.Brightness(canvas)
            frame = enhancer.enhance(0.94 + (0.08 * (1.0 - abs(phase - 0.5) * 2.0)))
            frames.append(frame.convert("P", palette=Image.Palette.ADAPTIVE))
        frames[0].save(
            gif_path,
            save_all=True,
            append_images=frames[1:],
            duration=90,
            loop=0,
            disposal=2,
            transparency=0,
        )

    def _extract_frames(self, source_path: Path, output_dir: Path) -> list[Path]:
        for old_file in output_dir.glob("frame_*.png"):
            old_file.unlink()

        with Image.open(source_path) as image:
            total_frames = getattr(image, "n_frames", 1)
            requested = self.config["frame_count"]
            indexes = sorted({int(i * total_frames / requested) for i in range(requested)})
            rendered: list[Path] = []
            for output_index, frame_index in enumerate(indexes):
                image.seek(frame_index)
                frame = image.convert("RGBA")
                path = output_dir / f"frame_{output_index:02d}.png"
                frame.save(path)
                rendered.append(path)
        return rendered

    def _render_frames(self, raw_paths: list[Path], output_dir: Path) -> list[Path]:
        for old_file in output_dir.glob("frame_*.ansi"):
            old_file.unlink()

        rendered_paths: list[Path] = []
        for index, raw_path in enumerate(raw_paths):
            target_path = output_dir / f"frame_{index:02d}.ansi"
            cmd = self._build_chafa_command(raw_path)
            result = subprocess.run(
                cmd,
                check=True,
                capture_output=True,
                text=True,
            )
            target_path.write_text(result.stdout.rstrip("\n") + "\n", encoding="utf-8")
            rendered_paths.append(target_path)
        return rendered_paths

    def _build_chafa_command(self, raw_path: Path) -> list[str]:
        cmd = [
            "chafa",
            "--format=symbols",
            f"--size={self.config['width']}x{self.config['height']}",
            f"--align={self.config['align']}",
            f"--symbols={self.config['symbols']}",
            f"--fill={self.config['fill']}",
            f"--colors={self.config['colors']}",
            f"--color-space={self.config['color_space']}",
            f"--color-extractor={self.config['color_extractor']}",
            f"--bg={self.config['bg']}",
            f"--threshold={self.config['threshold']}",
            f"--preprocess={self.config['preprocess']}",
            f"--dither={self.config['dither']}",
            "--animate=off",
        ]
        if self.config["fg_only"]:
            cmd.append("--fg-only")
        cmd.append(str(raw_path))
        return cmd

    def _signature_for(self, source_path: Path) -> str:
        config_text = json.dumps(self.config, sort_keys=True)
        digest = hashlib.sha256()
        digest.update(source_path.read_bytes())
        digest.update(config_text.encode("utf-8"))
        return digest.hexdigest()

    def _is_cache_valid(self, manifest_path: Path, signature: str) -> bool:
        if not manifest_path.exists():
            return False
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        return manifest.get("signature") == signature

    def _read_cached_frames(self, output_dir: Path, manifest_path: Path) -> list[list[str]]:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        frames = []
        for file_name in manifest["frames"]:
            frames.append((output_dir / file_name).read_text(encoding="utf-8").splitlines())
        return frames
