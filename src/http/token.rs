use std::io::{Error, Result};

use crate::http::def::HttpSession;

impl HttpSession {
    pub(super) fn get_token(&self) -> Result<&String> {
        if self.is_token_expired() {
            Err::<&String, Error>(Error::new(std::io::ErrorKind::PermissionDenied, "Token"));
            // FIXME let new_token = self.fetch_token()?;
            // self.update_token(new_token);
        }
        Ok(&self.token.current)
    }

    fn is_token_expired(&self) -> bool {
        self.token.created.elapsed().as_secs() >= self.token.expires
    }

    // fn fetch_token(&self) -> Result<TokenResponse> {
    //     let auth = &self.config.auth;

    //     let secret = self.secret_provider.get_secret(self.config.as_ref());

    //     let mut response = match self.agent.post(&auth.issuer).send_form([
    //         ("grant_type", "client_credentials"),
    //         ("client_id", &auth.client_id),
    //         ("client_secret", &secret),
    //         ("scope", &format!("{}/.default", auth.client_id)),
    //     ]) {
    //         Ok(response) if response.status() == 200 => Ok(response),
    //         Ok(mut response) => {
    //             self.secret_provider.invalidate_secret(self.config.as_ref());
    //             io_error!(format!(
    //                 "HTTP {}\n{}",
    //                 response.status(),
    //                 response
    //                     .body_mut()
    //                     .read_to_string()
    //                     .ok()
    //                     .unwrap_or_default()
    //             ))
    //         }
    //         Err(err) => io_error!(err),
    //     }?;

    //     let body = match response.body_mut().read_to_string() {
    //         Ok(body) => Ok(body),
    //         Err(err) => io_error!(err),
    //     }?;

    //     Ok(serde_json::from_str::<TokenResponse>(&body)?)
    // }

    // fn update_token(&self, new_token: TokenResponse) {
    //     let maximum_grace_period = min(
    //         new_token.expires_in,
    //         self.config.auth.expiration_grace_period,
    //     );
    //     self.token.replace(Token {
    //         current: format!("Bearer {}", new_token.access_token),
    //         expires: new_token.expires_in - maximum_grace_period,
    //         created: Instant::now(),
    //     });
    // }
}
