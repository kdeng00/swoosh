pub mod types;

pub async fn response_to_json(response: reqwest::Response) -> serde_json::Value {
    match response.json().await {
        Ok(json_body) => json_body,
        Err(_err) => serde_json::Value::Null,
    }
}

/// Backwards compatible
pub use send_message as send_mssage;

/// Send a message
pub async fn send_message(
    msg: &textsender_models::message::Message,
    contact: &textsender_models::contact::Contact,
    param: &crate::twilio::types::Parameters,
    config: &textsender_models::config::auxiliary::TwilioConfig,
) -> Result<reqwest::Response, std::io::Error> {
    if config.account_sid.is_empty() {
        Err(std::io::Error::other("Account SID is empty"))
    } else if config.auth_token.is_empty() {
        Err(std::io::Error::other(" Auth token is empty"))
    } else {
        match reqwest::Client::builder().build() {
            Ok(client) => {
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    "application/x-www-form-urlencoded".parse().unwrap(),
                );
                headers.insert("Accept", "application/json".parse().unwrap());
                let auth = generate_auth(config).unwrap();
                headers.insert("Authorization", auth.parse().unwrap());

                let params = init_params(contact, msg, config, param);

                let url = format!(
                    "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
                    config.account_sid
                );

                let request = client.post(url).headers(headers).form(&params);
                match request.send().await {
                    Ok(response) => match response.status() {
                        reqwest::StatusCode::CREATED => Ok(response),
                        status => Err(std::io::Error::other(format!(
                            "Unaccounted status: {status:?}"
                        ))),
                    },
                    Err(err) => Err(std::io::Error::other(err.to_string())),
                }
            }
            Err(err) => Err(std::io::Error::other(err.to_string())),
        }
    }
}

fn init_params(contact: &textsender_models::contact::Contact, message: &textsender_models::message::Message, config: &textsender_models::config::auxiliary::TwilioConfig, param: &crate::twilio::types::Parameters) -> std::collections::HashMap<String, String> {
        let now = time::OffsetDateTime::now_utc();
                let mut params = std::collections::HashMap::new();
                params.insert(String::from("To"), contact.phone_number.clone());
                params.insert(String::from("ProvideFeedback"), String::from("true"));
                params.insert(String::from("ForceDelivery"), String::from("false"));
                params.insert(String::from("ContentRetention"), String::from("retain"));
                params.insert(String::from("AddressRetention"), String::from("obfuscate"));
                params.insert(String::from("SmartEncoded"), String::from("true"));
                params.insert(String::from("ShortenUrls"), String::from("true"));
                let date = match param.schedule_at {
                    Some(date_value) => date_value.to_string(),
                    None => String::new(),
                };
                if param.schedule && is_scheduleable(&Some(now), &param.schedule_at) {
                    params.insert(String::from("ScheduleType"), String::from("fixed"));
                    params.insert(String::from("SendAt"), date);
                }
                params.insert(String::from("SendAsMms"), String::from("true"));
                params.insert(String::from("RiskCheck"), String::from("enable"));
                params.insert(String::from("From"), config.phone_number.clone());
                params.insert(String::from("MessagingServiceSid"), config.service_sid.clone());
                params.insert(String::from("Body"), message.content.clone());

/*
    let mut params = std::collections::HashMap::new();
    params.insert("StatusCallback", "http://OjQozHznkhNTTR.vpnrM1zdXFuiQ");
    params.insert("MaxPrice", "1");
    params.insert("ProvideFeedback", "true");
    params.insert("Attempt", "5");
    params.insert("ValidityPeriod", "1537");
    params.insert("ForceDelivery", "false");
    params.insert("ContentRetention", "retain");
    params.insert("AddressRetention", "obfuscate");
    params.insert("SmartEncoded", "true");
    params.insert("PersistentAction", "string");
    params.insert("PersistentAction", "string");
    params.insert("ShortenUrls", "true");
    params.insert("SendAsMms", "true");
    params.insert("RiskCheck", "enable");
*/

                params
}

fn generate_auth(
    config: &textsender_models::config::auxiliary::TwilioConfig,
) -> Result<String, base64_ng::EncodeError> {
    let input = format!("{}:{}", config.account_sid, config.auth_token);
    match base64_ng::encode(input.as_bytes()) {
        Ok(encoded) => Ok(format!("Basic {encoded}")),
        Err(err) => Err(err),
    }
}

const DEFAULT_SCHEDULING_SECONDS: i64 = 300;

fn is_scheduleable(
    now: &Option<time::OffsetDateTime>,
    scheduled: &Option<time::OffsetDateTime>,
) -> bool {
    match scheduled {
        Some(schedule_at) => {
            let early = now
                .unwrap()
                .checked_add(time::Duration::seconds(DEFAULT_SCHEDULING_SECONDS))
                .unwrap();
            *schedule_at > early
        }
        None => false,
    }
}
