use kana_tui::{
    app::App,
    config::Config,
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, path::PathBuf};

#[tokio::main]
async fn main() -> kana_tui::AppResult<()> {
    let config = get_or_create_default_config();
    let mut app = App::new(config);

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            _ => {}
        }
    }

    tui.exit()?;
    Ok(())
}

fn get_or_create_default_config() -> Config {
    let config_path = get_config_path();
    Config::parse_from_path(&config_path).unwrap_or_else(|_| {
        let default_config = Config::default();
        default_config
            .save(&config_path)
            .expect("Could not save config");
        default_config
    })
}

fn get_config_path() -> PathBuf {
    let config_folder = dirs::config_dir()
        .expect("Config folder for your OS not found")
        .join(env!("CARGO_PKG_NAME"));
    if !config_folder.exists() {
        std::fs::create_dir_all(&config_folder).expect("Could not create config folder");
    }

    config_folder.join("config.toml")
}
