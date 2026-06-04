// use josekit::jwk::alg::ec::EcCurve::P256;

pub mod types;

pub async fn send_mssage(
    msg: &textsender_models::message::Message,
    contact: &textsender_models::contact::Contact,
    param: crate::twilio::types::Parameters,
    config: &textsender_models::config::auxiliary::TwilioConfig,
) -> Result<crate::twilio::api::types::TwilioAPIMessage, std::io::Error> {
    if config.account_sid.is_nil() {
        Err(std::io::Error::other("Account SID is empty"))
    } else if config.auth_token.is_empty() {
        Err(std::io::Error::other(" Auth token is empty"))
    } else {
        let now = time::OffsetDateTime::now_utc();
        match reqwest::Client::builder().build() {
            Ok(client) => {
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    "application/x-www-form-urlencoded".parse().unwrap(),
                );
                headers.insert("Accept", "application/json".parse().unwrap());
                let auth = generate_auth(config).unwrap();
                // headers.insert("Authorization", "Basic QUNlZmExZWY1MTYzMTRjOWQxYTY4Y2JkNjU3ZGU0OTI3NzpmNGExZjJiMGI3OWVhMzczNTA3OGMyZDhlZTk2ODRlMQ==".parse()?);
                headers.insert("Authorization", auth.parse().unwrap());

                let mut params = std::collections::HashMap::new();
                params.insert("To", contact.phone_number.as_str());
                // params.insert("StatusCallback", "http://OjQozHznkhNTTR.vpnrM1zdXFuiQ");
                // params.insert("MaxPrice", "1");
                params.insert("ProvideFeedback", "true");
                // params.insert("Attempt", "5");
                // params.insert("ValidityPeriod", "1537");
                params.insert("ForceDelivery", "false");
                params.insert("ContentRetention", "retain");
                params.insert("AddressRetention", "obfuscate");
                params.insert("SmartEncoded", "true");
                // params.insert("PersistentAction", "string");
                // params.insert("PersistentAction", "string");
                params.insert("ShortenUrls", "true");
                let date = match param.schedule_at {
                    Some(date_value) => {
                        let converted = date_value.to_string();
                        converted
                    }
                    None => {
                       String::new()
                    }
                };
                if param.schedule && is_scheduleable(&Some(now), &param.schedule_at) {
                    params.insert("ScheduleType", "fixed");
                    params.insert("SendAt", date.as_str());
                }
                params.insert("SendAsMms", "true");
                params.insert("RiskCheck", "enable");
                params.insert("From", config.phone_number.as_str());
                params.insert("MessagingServiceSid", config.service_sid.as_str());
                params.insert("Body", msg.content.as_str());

                let url = format!(
                    "https://api.twilio.com/2010-04-01/Accounts/ACefa1ef516314c9d1a68cbd657de49277/Messages.json"
                );

                // let client = reqwest::Client::new();
                /*
                let request = client.request(reqwest::Method::POST, url)
                    .headers(headers)
                    .form(&params);
                        */
                let request = client.post(url).headers(headers).form(&params);

                match request.send().await {
                    Ok(response) => {
                        match response.text().await {
                            Ok(body) => {
                                let mut obj = crate::twilio::api::types::TwilioAPIMessage {
                                    ..Default::default()
                                };

                                Ok(obj)
                            }
                            Err(err) => {
                                Err(std::io::Error::other(err))
                            }
                        }
                    }
                    Err(err) => Err(std::io::Error::other(err.to_string())),
                }
            }
            Err(err) => Err(std::io::Error::other(err.to_string())),
        }
    }
}

fn generate_auth(config: &textsender_models::config::auxiliary::TwilioConfig) -> Result<String, base64_ng::EncodeError> {
    let input = format!("{}:{}", config.account_sid, config.auth_token);
    match base64_ng::encode(input.as_bytes()) {
        Ok(encoded) => {
            Ok(format!("Basic {encoded}"))
        }
        Err(err) => {
            Err(err)
        }
    }
}
/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);
    headers.insert("Accept", "application/json".parse()?);
    headers.insert("Authorization", "Basic QUNlZmExZWY1MTYzMTRjOWQxYTY4Y2JkNjU3ZGU0OTI3NzpmNGExZjJiMGI3OWVhMzczNTA3OGMyZDhlZTk2ODRlMQ==".parse()?);

    let mut params = std::collections::HashMap::new();
    params.insert("To", "16303831708");
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
    params.insert("ScheduleType", "fixed");
    params.insert("SendAt", "2026-06-03T19:50:00.0Z");
    params.insert("SendAsMms", "true");
    params.insert("RiskCheck", "enable");
    params.insert("From", "12243026041");
    params.insert("MessagingServiceSid", "MG803f3676706b92eb02e18dd820c447f2");
    params.insert("Body", "This is a test");

    let request = client.request(reqwest::Method::POST, "https://api.twilio.com/2010-04-01/Accounts/ACefa1ef516314c9d1a68cbd657de49277/Messages.json")
        .headers(headers)
        .form(&params);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
*/

/**
 func (m *MessageSender) Send(msg message.Message, number contact.Contact, sendTime *time.Time) (*types.TwilioResult, map[string]any, error) {
    if m.Config == nil {
        return nil, nil, fmt.Errorf("Config has not been initialized")
    }
    now := time.Now()
    client := twilio.NewRestClientWithParams(twilio.ClientParams{
        Username: m.Config.AccountSID,
        Password: m.Config.AuthToken,
    })

    params := &twilioApi.CreateMessageParams{}
    params.SetTo(number.PhoneNumber)
    params.SetFrom(m.Config.Number)
    params.SetMessagingServiceSid(m.Config.ServiceSID)
    params.SetBody(msg.Content)
    if sendTime != nil && isSchedulable(&now, sendTime) {
        params.SetSendAt(*sendTime)
        params.SetScheduleType(Schedule_Type)
    }

    if resp, err := client.Api.CreateMessage(params); err != nil {
        return nil, nil, fmt.Errorf("Error sending message: %v", err)
    } else {
        if twilioRespMarshaled, err := json.Marshal(*resp); err != nil {
            return nil, nil, fmt.Errorf("Error parsing result: %v", err)
        } else {
            var rawObject map[string]any
            if err := json.Unmarshal(twilioRespMarshaled, &rawObject); err != nil {
                return resp, nil, err
            } else {
                return resp, rawObject, nil
            }
        }
    }
}

func isSchedulable(now *time.Time, scheduled *time.Time) bool {
    early := now.Add(Schedulable_Limit_In_Seconds * time.Second)

    if scheduled.After(early) {
        return true
    } else {
        return false
    }
}
*/

fn is_scheduleable(
    now: &Option<time::OffsetDateTime>,
    scheduled: &Option<time::OffsetDateTime>,
) -> bool {
    match scheduled {
        Some(schedule_at) => {
            let early = now.unwrap().checked_add(time::Duration::seconds(300)).unwrap();
            if *schedule_at > early {
                true
            } else {
                false
            }
        }
        None => {
            false
        }
    }
}
