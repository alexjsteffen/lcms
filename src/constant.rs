#![allow(dead_code)]

pub enum Modes {
    Development(log::Level),
    Release,
}
pub const ITEMS_PER_PAGE: u64 = 12;
pub const MODE: Modes = Modes::Development(log::Level::Trace);

// the Default value
//pub const SUBPATH: &str = "/";
pub const SUBPATH: &str = "/";

// site info
pub const ADMIN: &str = "Hardy Steffen";
pub const SITE_NAME: &str = "Terms";
pub const LOGO_PIC: &str = "dr.ajhs.io/media/custom/mylogo.png";
pub const AVATR_PIC: &str = "hardy-steffen.b-cdn.net/li.png";
pub const SITE_DESCRIPTION: &str = "2022 Hardy Steffen
  ";

// Note That
// the string is line-separated
// each line is a key-value pair
pub const USER_INFO: &str = "
ajhs.li: https://ajhs.li
email: alex@ajhs.io,
";
// or the alternative
pub const OTHER_USER_INFO: &str = "ajhs.li: https://ajhs.li//,\nemail: alex@ajhs.io";
