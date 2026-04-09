use analysis::parse::{Announcements, ParseError};
use log::{error, info};
use std::process;

fn main() {
    env_logger::init();
    info!("Placeholder для экспериментов с cli");
    let parsing_demo =
        r#"[UserBackets{"user_id":"Bob","backets":[Backet{"asset_id":"milk","count":3,},],},]"#;
    let announcements = match analysis::parse::parse::<Announcements>(parsing_demo) {
        Ok((_, announcements)) => announcements,
        Err(ParseError::InvalidFormat) => {
            error!("Не удалось распарсить демо-строку");
            process::exit(1);
        }
    };
    info!("demo-parsed: {:?}", announcements);

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        error!("Укажите путь к файлу с логами");
        error!("Использование: {} <log_file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    info!(
        "Trying opening file '{}' from directory '{}'",
        filename,
        match std::env::current_dir() {
            Ok(dir) => dir.to_string_lossy().to_string(),
            Err(e) => {
                error!("Не удалось получить текущую директорию: {}", e);
                process::exit(1);
            }
        }
    );

    let file = match std::fs::File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            error!("Не удалось открыть файл '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let logs = analysis::read_log(file, analysis::READ_MODE_ALL, vec![]);
    info!("got logs ({} entries):", logs.len());
    logs.iter().for_each(|parsed| info!("{:?}", parsed));
}
