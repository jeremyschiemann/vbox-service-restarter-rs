use scraper::Selector;

pub(crate) const MINUTE: u64 = 60;
pub(crate) const URL_PATH: &str = "cgi-bin/ManageApp/ManageApp";

pub(crate) const USERNAME_KEY: &str = "VBOX_USERNAME";
pub(crate) const PASSWORD_KEY: &str = "VBOX_PASSWORD";
pub(crate) const BASEURL_KEY: &str = "VBOX_IP";
pub(crate) const SLEEPTIME_KEY: &str = "VBOX_SLEEPTIME";

pub(crate) const STREAMER_SELECTOR: &str =
    "#SysServicesForm > center > table > tbody > tr:nth-child(5) > td:nth-child(2) > font > center";
pub(crate) const UPNP_SERVER_SELECTOR: &str =
    "#SysServicesForm > center > table > tbody > tr:nth-child(6) > td:nth-child(2) > font > center";


pub(crate) fn get_parsed_selector(selector_str: &str) -> Selector {
    match Selector::parse(selector_str) {
        Ok(selector) => selector,
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1);
        }
    }
}