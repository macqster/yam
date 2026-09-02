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

use std::ffi::OsString;

use crate::ui::state::BootStartPolicy;

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
    runtime::run(runtime_options(&args, std::env::var_os(AUTO_START_ENV)))
}

/// Environment fallback for `--auto-start`, for launchers that find it easier
/// to set a variable than to edit an argument vector.
const AUTO_START_ENV: &str = "YAM_AUTO_START";

/// Builds the runtime options from an argument vector and the auto-start
/// environment variable.
///
/// Takes both as parameters rather than reading the environment itself so the
/// precedence is testable without mutating the process environment, which is
/// racy under the parallel test harness.
///
/// Note this is only reached for an ordinary run: `--version`, `--identity`,
/// `--check-updates`, `--update` and `--compile-hero` are handled earlier in
/// `main` and return before this point, so those modes are unaffected by the
/// flags below.
fn runtime_options(args: &[String], auto_start_env: Option<OsString>) -> runtime::RuntimeOptions {
    let initial_world_kind = if args.iter().any(|a| a == "--sandbox") {
        crate::core::world::WorldKind::Sandbox
    } else {
        crate::core::world::WorldKind::MainScene
    };

    // Saved dev positions are preserved by default. They are reseeded only on
    // an explicit `--hard-reset`, or automatically when the saved file was
    // written by a different version - see `saved_state_predates_this_version`.
    let hard_reset = args.iter().any(|a| a == "--hard-reset");

    // The flag is authoritative and the variable is the fallback, so a launcher
    // that exports YAM_AUTO_START cannot be overridden by accident, and an
    // explicit flag never has to fight the environment. Only "1", "true", "yes"
    // and "on" enable it: an empty or unset variable, and anything else
    // (including "0" and "false"), leaves the default manual behavior alone.
    let auto_start = args.iter().any(|a| a == "--auto-start")
        || auto_start_env
            .and_then(|value| value.into_string().ok())
            .is_some_and(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            });

    runtime::RuntimeOptions {
        initial_world_kind,
        hard_reset,
        boot_start_policy: if auto_start {
            BootStartPolicy::Automatic
        } else {
            BootStartPolicy::Manual
        },
    }
}

/// CLI-parsing tests.
///
/// Deliberately a separate module from the `tests` above rather than merged
/// into it: that module is slated for removal on another branch, and keeping
/// these apart lets both changes land in either order without a conflict.
#[cfg(test)]
mod cli_tests {
    use super::{runtime_options, AUTO_START_ENV};
    use crate::core::world::WorldKind;
    use crate::ui::state::BootStartPolicy;
    use std::ffi::OsString;

    fn args(list: &[&str]) -> Vec<String> {
        // argv[0] is the binary path; the parser must not treat it as a flag.
        std::iter::once("yam-rust".to_string())
            .chain(list.iter().map(|a| a.to_string()))
            .collect()
    }

    #[test]
    fn manual_is_the_default() {
        let opts = runtime_options(&args(&[]), None);
        assert_eq!(opts.boot_start_policy, BootStartPolicy::Manual);
        assert_eq!(opts.initial_world_kind, WorldKind::MainScene);
        assert!(!opts.hard_reset);
    }

    #[test]
    fn auto_start_flag_selects_automatic() {
        let opts = runtime_options(&args(&["--auto-start"]), None);
        assert_eq!(opts.boot_start_policy, BootStartPolicy::Automatic);
    }

    #[test]
    fn auto_start_env_selects_automatic_when_the_flag_is_absent() {
        for value in ["1", "true", "yes", "on", "TRUE", " on "] {
            let opts = runtime_options(&args(&[]), Some(OsString::from(value)));
            assert_eq!(
                opts.boot_start_policy,
                BootStartPolicy::Automatic,
                "{value:?} should enable auto-start"
            );
        }
    }

    #[test]
    fn other_env_values_leave_manual_alone() {
        for value in ["", "0", "false", "no", "off", "maybe"] {
            let opts = runtime_options(&args(&[]), Some(OsString::from(value)));
            assert_eq!(
                opts.boot_start_policy,
                BootStartPolicy::Manual,
                "{value:?} should not enable auto-start"
            );
        }
    }

    #[test]
    fn the_flag_wins_over_a_disabling_env_value() {
        let opts = runtime_options(&args(&["--auto-start"]), Some(OsString::from("0")));
        assert_eq!(opts.boot_start_policy, BootStartPolicy::Automatic);
    }

    #[test]
    fn auto_start_composes_with_sandbox_and_hard_reset() {
        // The flag is orthogonal: it must not disturb world selection or reset.
        let opts = runtime_options(&args(&["--sandbox", "--auto-start", "--hard-reset"]), None);
        assert_eq!(opts.boot_start_policy, BootStartPolicy::Automatic);
        assert_eq!(opts.initial_world_kind, WorldKind::Sandbox);
        assert!(opts.hard_reset);
    }

    #[test]
    fn auto_start_does_not_imply_hard_reset_or_sandbox() {
        let opts = runtime_options(&args(&["--auto-start"]), None);
        assert!(
            !opts.hard_reset,
            "auto-start must not reseed saved positions"
        );
        assert_eq!(opts.initial_world_kind, WorldKind::MainScene);
    }

    #[test]
    fn a_similar_looking_argument_does_not_enable_auto_start() {
        // Matching is exact, not a prefix or substring test.
        for arg in ["--auto", "--auto-start=1", "auto-start", "--no-auto-start"] {
            let opts = runtime_options(&args(&[arg]), None);
            assert_eq!(
                opts.boot_start_policy,
                BootStartPolicy::Manual,
                "{arg:?} should not enable auto-start"
            );
        }
    }

    #[test]
    fn the_env_variable_is_namespaced() {
        assert_eq!(AUTO_START_ENV, "YAM_AUTO_START");
    }
}
