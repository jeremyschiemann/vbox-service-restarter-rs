use crate::constants::*;
use scraper::{Html, Selector};
use std::string::String;
use tokio::time;

fn is_service_down(service_element: &str) -> bool {
    !matches!(service_element, "STARTED")
}

async fn update_services(
    base_request: reqwest::RequestBuilder,
    streamer_down: bool,
    upnp_server_down: bool,
) {
    log::info!("Sending restart request.");
    let res = base_request
        .query(&[
            ("OPTION", 3),
            ("NEXT_STATE", 0),
            ("NEXT_STATE", 0),
            ("NEXT_STATE", 0),
            ("NEXT_STATE", streamer_down.into()),
            ("NEXT_STATE", upnp_server_down.into()),
            ("NEXT_STATE", 0),
            ("NEXT_STATE", 0),
        ])
        .send()
        .await;

    let status = res.unwrap().status();
    log::info!(
        "Received code {}: {}",
        status.as_str(),
        status.canonical_reason().unwrap()
    );
}

async fn get_service_status(
    base_request: reqwest::RequestBuilder,
    streamer_selector: &Selector,
    upnp_server_selector: &Selector,
) -> Result<(String, String), reqwest::Error> {
    log::info!("Checking status of services");

    let res = base_request.query(&[("OPTION", 2)]).send().await?;

    let status = res.status();
    log::info!(
        "Received code {}: {}",
        status.as_str(),
        status.canonical_reason().unwrap()
    );

    let res_txt = res.text().await?;

    let doc = Html::parse_document(res_txt.as_str());

    let streamer_element = doc.select(streamer_selector).next().unwrap();
    let upnp_server_element = doc.select(upnp_server_selector).next().unwrap();

    let streamer_html = streamer_element.inner_html();
    let upnp_server_html = upnp_server_element.inner_html();

    let streamer_str = streamer_html.trim_start_matches("&nbsp;");
    let upnp_server_str = upnp_server_html.trim_start_matches("&nbsp;");

    Ok((streamer_str.to_string(), upnp_server_str.to_string()))
}

pub(crate) async fn run_update_loop(
    baseurl: String,
    username: String,
    password: String,
    sleeptime: u64,
) {
    log::info!("Starting update loop");

    let client = reqwest::Client::new();
    let base_request = client
        .get(format!("{baseurl}/{URL_PATH}"))
        .basic_auth(&username, Some(&password));

    let streamer_selector = get_parsed_selector(STREAMER_SELECTOR);
    let upnp_server_selector = get_parsed_selector(UPNP_SERVER_SELECTOR);

    loop {
        let (streamer_element, upnp_server_element) = match get_service_status(
            base_request.try_clone().unwrap(),
            &streamer_selector,
            &upnp_server_selector,
        )
        .await
        {
            Ok(stat) => stat,
            Err(_) => continue,
        };

        let streamer_down = is_service_down(&streamer_element);
        let upnp_server_down = is_service_down(&upnp_server_element);

        log::info!(
            "Status: STREAMER={}, UPNP_SERVER={}",
            streamer_element,
            upnp_server_element
        );

        if streamer_down || upnp_server_down {
            log::info!("One or more services are not running!");

            update_services(
                base_request.try_clone().unwrap(),
                streamer_down,
                upnp_server_down,
            )
            .await;
        }

        time::sleep(time::Duration::from_secs(sleeptime)).await;
    }
}
