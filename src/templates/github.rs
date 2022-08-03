use std::collections::HashMap;

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub type Gitignores = Vec<String>;

pub type GitignoreResponse = HashMap<String, Vec<u8>>;

pub struct GithubApi {
    response: GitignoreResponse,
}

impl GithubApi {
    // TODO: Implement a better error type
    pub fn new() -> anyhow::Result<Self> {
        const API_URL: &str = "https://api.github.com/gitignore/templates";

        let response: Gitignores = reqwest::blocking::get(API_URL)?.json()?;

        let mapped: Vec<(String, Vec<u8>)> = response
            .par_iter()
            .progress_count(response.len() as u64)
            .map(|template| {
                let download_url = format!(
                    "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
                    template
                );

                let file = reqwest::blocking::get(download_url)?;

                Ok((template.to_string(), vec![9]))
            })
            .collect::<Vec<Result<_, _>>>();

        Ok(Self { response })
    }
}
