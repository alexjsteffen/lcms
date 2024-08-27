use std::path::PathBuf;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use rand::Rng;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlogMeta {
    pub id: u64,
    pub title: String,
    pub timestamp: u64,
    pub date: String,
    pub path: PathBuf,
    pub hero: String,
}

impl Hash for BlogMeta {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.timestamp.hash(state);
        self.path.hash(state);
    }
}

impl BlogMeta {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::new(),
            timestamp: 0,
            date: String::new(),
            path: PathBuf::new(),
            hero: String::new(),
        }
    }

    pub fn get_hash(&mut self) {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        self.id = hasher.finish();
    }

    pub fn with_path(path: &str) -> Option<Self> {
        let path = PathBuf::from(path);
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                let pat = regex::Regex::new(
                    r"(\d{2,4}\D\d{1,2}\D\d{1,2}(\D\d{1,2}){0,3})\D(?P<title>.*?)\.rmd$",
                )
                .unwrap();

                if let Some(cap) = pat.captures(file_name_str) {
                    let date = cap[1].to_string();
                    let mut time_items = [0u64; 6];
                    const UNITS: [u64; 6] = [365 * 24 * 3600, 30 * 24 * 3600, 24 * 3600, 60 * 60, 60, 1];

                    cap[1]
                        .split(|c: char| !c.is_ascii_digit())
                        .enumerate()
                        .for_each(|(ind, e)| {
                            time_items[ind] = e.parse().unwrap_or(0);
                        });

                    if time_items[0] < 100 {
                        time_items[0] += 2000;
                    }

                    let sum: u64 = UNITS.iter().zip(time_items.iter()).map(|(u, t)| u * t).sum();

                    if let Some(title_match) = cap.name("title") {
                        let mut meta = Self {
                            id: 0,
                            title: title_match.as_str().to_string(),
                            path,
                            timestamp: sum,
                            date,
                            hero: String::new(),
                        };
                        meta.get_hash();
                        meta.image_url();

                        return Some(meta);
                    }
                }
            }
            log::error!("file name is not valid: {:?}", path);
        } else {
            log::error!("file name is not valid: {:?}", path);
        }
        None
    }

    pub fn image_url(&mut self) {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(1..=8);
        self.hero = format!("https://cdn.hardysteffen.com/tpbg/BG-{}.png", random_number);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blog {
    pub meta: BlogMeta,
    pub tags: Vec<String>,
    pub content: Vec<String>,
    pub published: bool,
    pub ignored: bool,
}

impl Blog {
    pub fn date_info(&mut self, date: Option<&str>) {
        let pat = regex::Regex::new(r"(\d{2,4}\D\d{1,2}\D\d{1,2}(\D\d{1,2}){0,3})").unwrap();
        let mut time_items = [0u64; 6];
        const UNITS: [u64; 6] = [365 * 24 * 3600, 30 * 24 * 3600, 24 * 3600, 60 * 60, 60, 1];

        if let Some(path_str) = self.meta.path.to_str() {
            if let Some(cap) = pat.captures(path_str) {
                cap[1]
                    .split(|c: char| !c.is_ascii_digit())
                    .enumerate()
                    .for_each(|(ind, e)| {
                        time_items[ind] = e.parse().unwrap_or(0);
                    });

                if time_items[0] < 100 {
                    time_items[0] += 2000;
                }

                self.meta.timestamp = UNITS.iter().zip(time_items.iter()).map(|(u, t)| u * t).sum();
            } else if let Some(s) = date {
                if let Some(cap) = pat.captures(s) {
                    cap[1]
                        .split(|c: char| !c.is_ascii_digit())
                        .enumerate()
                        .for_each(|(ind, e)| {
                            time_items[ind] = e.parse().unwrap_or(0);
                        });

                    if time_items[0] < 100 {
                        time_items[0] += 2000;
                    }

                    self.meta.timestamp = UNITS.iter().zip(time_items.iter()).map(|(u, t)| u * t).sum();
                } else {
                    log::error!("Time Stamp is not found in file name nor defined in file");
                    log::error!("file is ignored to proceed: {:?}", self.meta.path);
                    self.ignored = true;
                }
            } else {
                log::error!("Time Stamp is not found in file name nor defined in file");
                log::error!("file is ignored to proceed: {:?}", self.meta.path);
                self.ignored = true;
            }
        }
    }
}

#[test]
fn test_date_info() {
    let mut blog = Blog {
        meta: BlogMeta::new(),
        tags: vec![],
        content: vec![],
        published: false,
        ignored: false,
    };
    blog.date_info(Some("2019-10-07"));
    blog.date_info(Some("2019-10-07-02-01"));
    blog.date_info(Some("2019/10/07/02/01"));
    blog.date_info(Some("2019/10/07 19:57"));
    blog.date_info(Some("2019-10/07 19:57:36"));
    blog.date_info(None);
}
