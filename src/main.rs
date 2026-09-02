mod build_info;
mod core;
mod diagnostics;
mod render;
mod runtime;
mod scene;
mod systems;
mod theme;
mod ui;
mod weather;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fn run(cmd: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let status = std::process::Command::new(cmd).args(args).status()?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("{cmd} {} exited with status {status}", args.join(" ")).into())
        }
    }

    fn print_runtime_identity() -> Result<(), Box<dyn std::error::Error>> {
        println!("==============================");
        println!("YAM RUNTIME IDENTITY");
        println!("SOURCE: {}", env!("CARGO_MANIFEST_DIR"));
        println!(
            "BUILD: yam {}, build {} ({})",
            build_info::VERSION,
            build_info::BUILD_TIME,
            build_info::build_hash()
        );
        println!("BIN PATH: {:?}", std::env::current_exe()?);
        println!("==============================");
        Ok(())
    }

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version") {
        println!("yam-rust {}", build_info::VERSION);
        return Ok(());
    }
    if args.iter().any(|a| a == "--identity") {
        print_runtime_identity()?;
        return Ok(());
    }
    if args.iter().any(|a| a == "--check-updates") {
        return run("cargo", &["outdated"]);
    }
    if args.iter().any(|a| a == "--update") {
        return run("bash", &["scripts/update.sh"]);
    }
    if let Some(flag_pos) = args.iter().position(|a| a == "--compile-hero") {
        // The argument selects a *registered* source, by stem or path. It
        // cannot just override `source_path`: geometry and `absent_color` come
        // from the descriptor, so overriding the path alone would compile one
        // asset against another's drop reference and file it under the wrong
        // package name.
        let options = match args.get(flag_pos + 1).filter(|a| !a.starts_with("--")) {
            Some(requested) => match render::hero_source::from_stem_or_path(requested) {
                Some(source) => render::hero_compiler::CompileOptions::for_source(&source),
                None => {
                    let known: Vec<&str> =
                        render::hero_source::ALL.iter().map(|s| s.stem).collect();
                    return Err(format!(
                        "unknown hero source {requested:?}; registered sources are: {}",
                        known.join(", ")
                    )
                    .into());
                }
            },
            None => render::hero_compiler::CompileOptions::default(),
        };
        return render::hero_compiler::compile(&options)
            .map(|_| ())
            .map_err(|err| err.into());
    }
    let initial_world_kind = if args.iter().any(|a| a == "--sandbox") {
        crate::core::world::WorldKind::Sandbox
    } else {
        crate::core::world::WorldKind::MainScene
    };
    // Saved dev positions are preserved by default. They are reseeded only on
    // an explicit `--hard-reset`, or automatically when the saved file was
    // written by a different version - see `saved_state_predates_this_version`.
    let hard_reset = args.iter().any(|a| a == "--hard-reset");
    runtime::run(initial_world_kind, hard_reset)
}
