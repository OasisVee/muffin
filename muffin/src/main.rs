use app::driver::App;
use app::config;
mod app;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let arg0 = args.next().unwrap();

    let mut presets_path = "~/.config/muffin/presets.kdl".to_string();
    let mut launch_preset = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--presets" | "-p" => {
                presets_path = args.next().ok_or(format!("{arg} expects a path"))?;
            }
            "--help" | "-h" => {
                eprintln!(
                    r"
Usage: {arg0} [OPTIONS]

OPTIONS:
    -p, --presets <FILE>    Path to KDL file with session presets
    -l, --launch <PRESET>   Launch a preset
    -h, --help              Print help
                        ",
                );
                std::process::exit(1);
            }
            "--launch" | "-l" => {
                launch_preset = Some(args.next().ok_or(format!("{arg} expects a preset name"))?);
            }
            x => {
                eprintln!("Unknown flag or value '{x}'. Run '{arg0} --help' for usage.",);
                std::process::exit(1);
            }
        }
    }

    let presets_path = shellexpand::full(&presets_path)
        .expect("Failed to expand environment variables in path")
        .to_string();

    let presets_str: String = std::fs::read(&presets_path)
        .expect("Error reading file.")
        .try_into()
        .expect("Error parsing file into a string.");
    let presets = parser::parse_config(&presets_str)?;

    if let Some(preset_name) = launch_preset {
        if let Some(preset) = presets.get(&preset_name) {
            tmux::spawn_preset(preset)?;
            if std::env::var("TMUX").is_ok() {
                tmux::switch_session(&preset.name)?;
            } else {
                tmux::attach_session(&preset.name)?;
            }
            return Ok(());
        } else {
            return Err(format!("Preset '{preset_name}' not found."));
        }
    }

    let sessions = match tmux::list_sessions() {
        Ok(sessions) => sessions,
        Err(_) => {
            let config = config::load_config().unwrap_or_default();
            if let Some(default_preset_name) = config.default_preset {
                if let Some(preset) = presets.get(&default_preset_name) {
                    tmux::spawn_preset(preset)?;
                } else {
                    tmux::create_session("")?;
                }
            } else {
                tmux::create_session("")?;
            }
            tmux::list_sessions()?
        }
    };

    let mut app = App::new(sessions, presets, presets_path.to_string());

    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal).await;

    ratatui::restore();
    app_result
}
