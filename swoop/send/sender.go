package send

import (
	"encoding/json"
	"fmt"
	"time"

	"git.kundeng.us/phoenix/textsender-models/pkg/config"
	"git.kundeng.us/phoenix/textsender-models/pkg/contact"
	"git.kundeng.us/phoenix/textsender-models/pkg/message"
	"github.com/twilio/twilio-go"
	twilioApi "github.com/twilio/twilio-go/rest/api/v2010"
)

type TwilioResult = twilioApi.ApiV2010Message

type MessageSender struct {
	Numbers []contact.Contact
	Config  config.TwiloConfig
}

func (m *MessageSender) Send(msg message.Message, number contact.Contact, time *time.Time) (*TwilioResult, error) {
	now := time.Now()
	client := twilio.NewRestClientWithParams(twilio.ClientParams{
		Username: Config.AccountSID,
		Password: Config.AuthToken,
	})

	params := &twilioApi.CreateMessageParams{}
	params.SetTo(number.PhoneNumber)
	params.SetFrom(Config.Number)
	params.SetBody(message.Content)
	if time != nil {
		if time > now {
			params.SetSendAt(*time)
		}
	}

	resp, err := client.Api.CreateMessage(params)
	if err != nil {
		return nil, fmt.Errorf("Error sending message: %v", err)
	} else {
		if _, err := json.Marshal(*resp); err != nil {
			return nil, fmt.Errorf("Error parsing result: %v", err)
		} else {
			return resp, nil
		}
	}
}
