pub mod types;

use crate::schedulable;
use crate::url::twilio_url;
use crate::util;

/// Converts the response into a json value
/// That way you do not need to share swoosh's dependency
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
    msg: &schedtxt_models::message::Message,
    contact: &schedtxt_models::contact::Contact,
    param: &crate::twilio::types::Parameters,
    config: &schedtxt_models::config::auxiliary::TwilioConfig,
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

                let url = twilio_url(&config.account_sid);

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

fn generate_auth(
    config: &schedtxt_models::config::auxiliary::TwilioConfig,
) -> Result<String, base64_ng::EncodeError> {
    let input = format!("{}:{}", config.account_sid, config.auth_token);
    match base64_ng::encode(input.as_bytes()) {
        Ok(encoded) => Ok(format!("Basic {encoded}")),
        Err(err) => Err(err),
    }
}

fn init_params(
    contact: &schedtxt_models::contact::Contact,
    message: &schedtxt_models::message::Message,
    config: &schedtxt_models::config::auxiliary::TwilioConfig,
    param: &crate::twilio::types::Parameters,
) -> std::collections::HashMap<String, String> {
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
        Some(date_value) => match util::convert_time_to_iso(date_value) {
            Ok(converted) => converted,
            Err(err) => {
                eprintln!("Error: {err:?}");
                String::new()
            }
        },
        None => String::new(),
    };
    let scheduled_at = match param.schedule_at {
        Some(s) => s,
        None => time::OffsetDateTime::now_utc(),
    };
    if param.schedule && schedulable::is_scheduleable(Some(&now), Some(&scheduled_at)) {
        params.insert(String::from("ScheduleType"), String::from("fixed"));
        params.insert(String::from("SendAt"), date.clone());
    }
    params.insert(String::from("SendAsMms"), String::from("true"));
    params.insert(String::from("RiskCheck"), String::from("enable"));
    params.insert(String::from("From"), config.phone_number.clone());
    params.insert(
        String::from("MessagingServiceSid"),
        config.service_sid.clone(),
    );
    params.insert(String::from("Body"), message.content.clone());

    params
}
