mod constants;
mod service;

use constants::*;
use std::env;

#[tokio::main]
async fn main() {
    simple_logger::init_utc().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("Service started.");

    let mut errors = Vec::new();

    let username =
        env::var(USERNAME_KEY).map_err(|e| errors.push(format!("{USERNAME_KEY}: {}", e)));

    let password =
        env::var(PASSWORD_KEY).map_err(|e| errors.push(format!("{PASSWORD_KEY}: {}", e)));

    let baseurl = env::var(BASEURL_KEY)
        .map(|val| format!("http://{}", val))
        .map_err(|e| errors.push(format!("{BASEURL_KEY}: {}", e)));

    let sleeptime: Result<u64, ()> = env::var(SLEEPTIME_KEY)
        .unwrap_or("10".to_string())
        .parse::<u64>()
        .map_err(|e| errors.push(format!("{SLEEPTIME_KEY}: {}", e)));

    if !errors.is_empty() {
        errors.iter().for_each(|e| log::warn!("{}", e));
        std::process::exit(1);
    }

    let username = username.unwrap();
    let password = password.unwrap();
    let baseurl = baseurl.unwrap();
    let sleeptime = sleeptime.unwrap();

    service::run_update_loop(baseurl, username, password, sleeptime).await;
}
