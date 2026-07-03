pub mod twilio;

#[derive(Default, Debug)]
pub struct SendMsg {
    config: VendorConfig,
    message: Message,
    recipient: Recipient,
}

impl SendMsg {
    pub fn load_config(
        &mut self,
        auth_token: &str,
        phone_number: &str,
        service_sid: &str,
        auth_sid: &str,
    ) {
        self.config = VendorConfig {
            auth_token: auth_token.to_string(),
            phone_number: phone_number.to_string(),
            service_sid: service_sid.to_string(),
            auth_sid: auth_sid.to_string(),
        }
    }

    pub fn load_message(&mut self, content: &str) {
        self.message = Message {
            content: content.to_string(),
        }
    }

    pub fn load_recipient(&mut self, recpient: &str) {
        self.recipient = Recipient {
            phone_number: recpient.to_string(),
        }
    }

    pub async fn send_message(
        &self,
        param: &crate::twilio::types::Parameters,
    ) -> Result<reqwest::Response, std::io::Error> {
        if self.recipient.phone_number.is_empty() {
            return Err(std::io::Error::other("Recipient not provided"));
        } else if self.message.content.is_empty() {
            return Err(std::io::Error::other("Message not provided"));
        } else if self.config.phone_number.is_empty()
            || self.config.service_sid.is_empty()
            || self.config.auth_token.is_empty()
            || self.config.auth_sid.is_empty()
        {
            return Err(std::io::Error::other("Config not populated"));
        } else {
            let client = match reqwest::Client::builder().build() {
                Ok(client) => client,
                Err(err) => {
                    return Err(std::io::Error::other(err));
                }
            };

            let url = twilio::api::twilio_url(&self.config.auth_sid);
            let headers =
                match twilio::api::init_headers(&self.config.auth_sid, &self.config.auth_token) {
                    Ok(headers) => headers,
                    Err(err) => {
                        return Err(std::io::Error::other(err));
                    }
                };
            let params = twilio::api::initial_params(
                &self.recipient.phone_number,
                &self.message.content,
                &self.config.phone_number,
                &self.config.service_sid,
                param,
            );
            let request = client.post(url).headers(headers).form(&params);
            match request.send().await {
                Ok(response) => match response.status() {
                    reqwest::StatusCode::CREATED => Ok(response),
                    status => Err(std::io::Error::other(format!(
                        "Unaccounted status: {status:?}"
                    ))),
                },
                Err(err) => Err(std::io::Error::other(err)),
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct VendorConfig {
    pub auth_token: String,
    pub phone_number: String,
    pub service_sid: String,
    pub auth_sid: String,
}

#[derive(Default, Debug)]
pub struct Message {
    pub content: String,
}

#[derive(Default, Debug)]
pub struct Recipient {
    pub phone_number: String,
}
