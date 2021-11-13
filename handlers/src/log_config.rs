use chrono::Utc;

pub fn init() {
    let date_log = Utc::now().format("%Y-%m-%d");
    let path_log = format!("log/{}.log", date_log);
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] => {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // LEVEL LOG
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(path_log).unwrap())
        .apply()
        .unwrap();
}
