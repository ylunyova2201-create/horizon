//! Horizon: native Rust client for Minecraft Java Edition 26.3 gameplay.
#![deny(unsafe_op_in_unsafe_fn)]
mod game;

use horizon_client::{crash, logging};

/// The mod API level this game serves, kept in the program for the launcher
/// to read without running it (`horizon_mods::api_level::MARK`). `main`
/// reads it, so the linker keeps it.
static MOD_API_LEVEL: [u8; 27] = horizon_mods::api_level::MARK;

#[cfg(not(target_os = "android"))]
fn main() {
    std::hint::black_box(&MOD_API_LEVEL);
    horizon_client::telemetry::mark_start();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let game_args = args.iter().take_while(|a| *a != "--dedicated");
    let help = game_args.clone().any(|a| a == "--help" || a == "-h");
    // The dedicated server logs in its own folder, as vanilla's does; the game in its home.
    let (home, side, thread) = if args.iter().any(|a| a == "--dedicated") {
        (std::path::PathBuf::from("."), "server", "Server thread")
    } else {
        (logging::game_home(), "client", "Render thread")
    };
    if !help {
        logging::init(&home, thread);
        // The log's first line names the build, so a bug report that carries the log says which one it was.
        log::info!("Horizon {}", crash::version());
        crash::install(home.clone(), side);
    }
    if let Err(error) = game::run() {
        log::error!("Horizon: {error}");
        if !help {
            crash::report_error(&home, side, &*error);
        }
        // Started from Explorer, the console window closes with the game:
        // keep the message on screen until it is read.
        #[cfg(windows)]
        if std::io::IsTerminal::is_terminal(&std::io::stdin()) {
            eprintln!("Press Enter to close.");
            let _ = std::io::stdin().read_line(&mut String::new());
        }
        std::process::exit(1);
    }
    logging::flush();
}


/// NativeActivity entry point used by cargo-apk on Android.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    // Android does not promise a useful process working directory. Keep all
    // user data (resources, saves, options and touch layout) in this app's
    // private files directory, which survives app restarts and updates.
    let home = app.internal_data_path().unwrap_or_else(logging::game_home);
    if let Err(error) = std::fs::create_dir_all(&home)
        .and_then(|_| std::env::set_current_dir(&home))
    {
        eprintln!(
            "Horizon: could not prepare private app data directory {}: {error}",
            home.display()
        );
        return;
    }
    // Other parts of the client resolve logs and crash reports through this
    // variable. Set it before starting Horizon's worker threads.
    unsafe {
        std::env::set_var("HORIZON_HOME", &home);
    }

    horizon_client::telemetry::mark_start();
    logging::init(&home, "Render thread");
    log::info!("Horizon {} (Android)", crash::version());
    crash::install(home.clone(), "client");
    if let Err(error) = game::run_android(app) {
        log::error!("Horizon Android: {error}");
        crash::report_error(&home, "client", &*error);
        logging::flush();
        std::process::exit(1);
    }
    logging::flush();
}
