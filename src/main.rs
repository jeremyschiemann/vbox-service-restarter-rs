use log;
use log::Log;
use scraper::{Html, Selector};
use std::env;
use tokio::time;

const MINUTE: u64 = 60;
const URL_PATH: &str = "cgi-bin/ManageApp/ManageApp";

const USERNAME_KEY: &str = "VBOX_USERNAME";
const PASSWORD_KEY: &str = "VBOX_PASSWORD";
const BASEURL_KEY: &str = "VBOX_IP";
const SLEEPTIME_KEY: &str = "VBOX_SLEEPTIME";

#[tokio::main]
async fn main() {
    log::set_max_level(log::LevelFilter::Info);
    log::info!("Service started.");
    log::logger().flush();

    let mut errors = Vec::new();

    let username = env::var(USERNAME_KEY)
        .map_err(|e| errors.push(format!("{USERNAME_KEY}: {}", e.to_string())));

    let password = env::var(PASSWORD_KEY)
        .map_err(|e| errors.push(format!("{PASSWORD_KEY}: {}", e.to_string())));

    let baseurl = env::var(BASEURL_KEY)
        .map(|val| format!("http://{}", val))
        .map_err(|e| errors.push(format!("{BASEURL_KEY}: {}", e.to_string())));

    let sleeptime: Result<u64, ()> = env::var(SLEEPTIME_KEY)
        .unwrap_or("10".to_string())
        .parse::<u64>()
        .map_err(|e| errors.push(format!("{SLEEPTIME_KEY}: {}", e.to_string())));

    if !errors.is_empty() {
        errors.iter().for_each(|e| println!("{}", e));
        std::process::exit(1);
    }

    let username = username.unwrap();
    let password = password.unwrap();
    let baseurl = baseurl.unwrap();
    let sleeptime = sleeptime.unwrap();

    let fetch_query = [("OPTION", 2)];


    let client = reqwest::Client::new();

    loop {
        let res = client
            .get(format!("{baseurl}/{URL_PATH}"))
            .basic_auth(&username, Some(&password))
            .query(&fetch_query)
            .send()
            .await;

        let res = if let Ok(val) = res { val } else { continue };

        let res_txt = if let Ok(val) = res.text().await {
            val
        } else {
            continue;
        };

        let doc = Html::parse_document(res_txt.as_str());
        let streamer_selector = Selector::parse("#SysServicesForm > center > table > tbody > tr:nth-child(5) > td:nth-child(2) > font > center").unwrap();
        let upnp_server_selector = Selector::parse("#SysServicesForm > center > table > tbody > tr:nth-child(6) > td:nth-child(2) > font > center").unwrap();
        //#SysServicesForm > center > table > tbody > tr:nth-child(5) > td.bldowncolor > font > center

        let streamer_element = doc.select(&streamer_selector).next();
        let upnp_server_element = doc.select(&upnp_server_selector).next();

        let streamer_down = match streamer_element
            .unwrap()
            .inner_html()
            .trim_start_matches("&nbsp;")
        {
            "STARTED" => false,
            _ => true,
        };
        let upnp_server_down = match upnp_server_element
            .unwrap()
            .inner_html()
            .trim_start_matches("&nbsp;")
        {
            "STARTED" => false,
            _ => true,
        };

        println!("Status: {streamer_down}, {upnp_server_down}");

        if streamer_down || upnp_server_down {

            println!("Needs update");

            let update_query = [
                ("OPTION", 3),
                ("NEXT_STATE", 0),
                ("NEXT_STATE", 0),
                ("NEXT_STATE", 0),
                ("NEXT_STATE", streamer_down.into()),
                ("NEXT_STATE", upnp_server_down.into()),
                ("NEXT_STATE", 0),
                ("NEXT_STATE", 0),
            ];


            let res = client
                .get(format!("{baseurl}/{URL_PATH}"))
                .basic_auth(&username, Some(&password))
                .query(&update_query)
                .send()
                .await;

            println!("{:?}", res.unwrap().status())
        }


        time::sleep(time::Duration::from_secs(sleeptime * MINUTE)).await;
    }
}
