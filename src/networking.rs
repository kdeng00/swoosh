use crate::schedulable;
use crate::util;

pub fn g_auth(account_sid: &str, auth_token: &str) -> Result<String, base64_ng::EncodeError> {
    let input = format!("{}:{}", account_sid, auth_token);
    base64_ng::encode(input.as_bytes())
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
    params.insert(String::from("From"), config_source_phone_number.to_string());
    params.insert(
        String::from("MessagingServiceSid"),
        config_service_sid.to_string(),
    );
    params.insert(String::from("Body"), content.to_string());

    params
}
