pub mod types;

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

pub fn twilio_url(account_sid: &str) -> String {
    format!(
        "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
        account_sid
    )
}

pub fn init_headers(
    account_sid: &str,
    auth_token: &str,
) -> Result<reqwest::header::HeaderMap, std::io::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        match "application/x-www-form-urlencoded".parse() {
            Ok(value) => value,
            Err(err) => {
                return Err(std::io::Error::other(err));
            }
        },
    );
    let auth = match g_auth(account_sid, auth_token) {
        Ok(auth) => auth,
        Err(err) => {
            return Err(std::io::Error::other(err));
        }
    };
    headers.insert(
        "Authorization",
        match format!("Basic {auth}").parse() {
            Ok(auth) => auth,
            Err(err) => {
                return Err(std::io::Error::other(err));
            }
        },
    );

    Ok(headers)
}

pub fn initial_params(
    recpient: &str,
    content: &str,
    config_source_phone_number: &str,
    config_service_sid: &str,
    param: &crate::twilio::types::Parameters,
) -> std::collections::HashMap<String, String> {
    let now = time::OffsetDateTime::now_utc();
    let mut params = std::collections::HashMap::new();
    params.insert(String::from("To"), recpient.to_string());
    params.insert(String::from("ProvideFeedback"), String::from("true"));
    params.insert(String::from("ForceDelivery"), String::from("false"));
    params.insert(String::from("ContentRetention"), String::from("retain"));
    params.insert(String::from("AddressRetention"), String::from("obfuscate"));
    params.insert(String::from("SmartEncoded"), String::from("true"));
    params.insert(String::from("ShortenUrls"), String::from("true"));
    let date = match param.schedule_at {
        Some(date_value) => match convert_time_to_iso(date_value) {
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
    if param.schedule && is_scheduleable(Some(&now), Some(&scheduled_at)) {
        params.insert(String::from("ScheduleType"), String::from("fixed"));
        params.insert(String::from("SendAt"), date.clone());
    }
    params.insert(String::from("SendAsMms"), String::from("true"));
    params.insert(String::from("RiskCheck"), String::from("enable"));
    params.insert(String::from("From"), config_source_phone_number.to_string());
    params.insert(
        String::from("MessagingServiceSid"),
        config_service_sid.to_string(),
    );
    params.insert(String::from("Body"), content.to_string());

    params
}

fn init_params(
    contact: &textsender_models::contact::Contact,
    message: &textsender_models::message::Message,
    config: &textsender_models::config::auxiliary::TwilioConfig,
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
        Some(date_value) => match convert_time_to_iso(date_value) {
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
    if param.schedule && is_scheduleable(Some(&now), Some(&scheduled_at)) {
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

fn convert_time_to_iso(time: time::OffsetDateTime) -> Result<String, time::error::Format> {
    use time::format_description::well_known::Iso8601;
    time.format(&Iso8601::DEFAULT)
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

fn g_auth(account_sid: &str, auth_token: &str) -> Result<String, base64_ng::EncodeError> {
    let input = format!("{}:{}", account_sid, auth_token);
    base64_ng::encode(input.as_bytes())
}

const DEFAULT_SCHEDULING_SECONDS: i64 = 300;

fn is_scheduleable(
    now: Option<&time::OffsetDateTime>,
    scheduled: Option<&time::OffsetDateTime>,
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
