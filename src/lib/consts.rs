use std::path::PathBuf;

use directories::BaseDirs;
use lazy_static::lazy_static;

use crate::sleep_for;

lazy_static! {
    pub static ref CACHE_DIR: Option<PathBuf> =
        BaseDirs::new().map(|dirs| dirs.cache_dir().to_owned().join("gitignore"));
    pub static ref CACHE_ENABLED: bool = {
        if let Some(mut dir) = CACHE_DIR.to_owned() {
            dir.pop();
            dir.exists()
        } else {
            false
        }
    };
    pub static ref IS_ONLINE: bool = {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        let res = if let Ok(req) = client.get("https://github.com").send() {
            drop(client);
            req.status().is_success()
        } else {
            false
        };

        if !res {
            use colored::Colorize;
            println!("{}","warning: you are offline. you will only be able to use cached templates which may be out of date".yellow());
            sleep_for!(3000);
        }

        res
    };
}
