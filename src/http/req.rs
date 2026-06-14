use std::io::Result;

use serde::Deserialize;
use ureq::{RequestBuilder, typestate::WithoutBody};

use crate::{
    config::{Config, ConfigConnection},
    http::def::{HEADER_AUTH, HttpSession},
};

impl HttpSession {
    pub fn get_request(&self, config: &ConfigConnection, uri: &str) -> Result<RequestBuilder<WithoutBody>> {
        let token = self.get_token()?;

        Ok(config
            .extra_headers
            .iter()
            .fold(self.agent.get(uri), |r, (k, v)| r.header(k, v))
            .header(HEADER_AUTH, token))
    }
}

pub fn read_object<T>(request: RequestBuilder<WithoutBody>) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let body = read_string(request)?;
    match serde_json::from_str::<T>(&body) {
        Ok(result) => Ok(result),
        Err(err) => io_error!(format!("{}: {}", err, &body)),
    }
}

pub fn read_string(request: RequestBuilder<WithoutBody>) -> Result<String> {
    match request.call() {
        Ok(mut response) => match response.body_mut().read_to_string() {
            Ok(body) => {
                let status = response.status();
                if status.is_success() {
                    Ok(body)
                } else {
                    io_error!(format!("{}:\n{}", status.to_string(), &body))
                }
            }
            Err(err) => io_error!(err),
        },
        Err(err) => io_error!(err),
    }
}
