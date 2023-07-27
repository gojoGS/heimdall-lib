use std::time::SystemTime;

pub fn setup_fern() -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = base_config.format(|out, message, record| {
        out.finish(format_args!(
            "[{} {} {}] {}",
            humantime::format_rfc3339_seconds(SystemTime::now()),
            record.level(),
            record.target(),
            message
        ))
    });

    if cfg!(debug_assertions) {
        base_config = base_config
            .level(log::LevelFilter::Debug)
            .chain(fern::log_file("debug.log")?);
    } else {
        base_config = base_config.level(log::LevelFilter::Info);
    }

    base_config.apply()?;
    Ok(())
}
