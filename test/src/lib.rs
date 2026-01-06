#[cfg(test)]
mod meta;

use std::{env::var, str::FromStr, time::Duration};

use anyhow::Result;
use reqwest::{Method, Url};
use wait_on::{WaitOptions, Waitable, resource::http::HttpWaiter};

pub fn release_binary_path() -> Result<String> {
    let path = var("TARGET")?;
    Ok(format!("../target/{path}/release/bookworm"))
}
