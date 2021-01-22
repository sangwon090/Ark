use log::*;
use simplelog::*;
use chrono::prelude::*;

use std::fs;
use std::fs::File;

fn main() {
    let logger_config = ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%d %H:%M:%S%.3f")
        .set_time_to_local(true)
        .build();
    
    let datetime = Local::now();
    let datetime = datetime.format("%Y-%m-%d %H.%M.%S%.3f").to_string();

    fs::create_dir_all("./logs/").unwrap();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, logger_config.clone(), TerminalMode::Mixed),
            WriteLogger::new(LevelFilter::Trace, logger_config, File::create(format!("./logs/{}.log", datetime)).unwrap()),
        ]
    ).unwrap();
}
