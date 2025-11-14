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

func (m *MessageSender) Send(msg message.Message, number contact.Contact, sendTime *time.Time) (*TwilioResult, error) {
	now := time.Now()
	client := twilio.NewRestClientWithParams(twilio.ClientParams{
		Username: m.Config.AccountSID,
		Password: m.Config.AuthToken,
	})

	params := &twilioApi.CreateMessageParams{}
	params.SetTo(number.PhoneNumber)
	params.SetFrom(m.Config.Number)
	params.SetBody(msg.Content)
	if sendTime != nil {
		if sendTime.After(now) {
			params.SetSendAt(*sendTime)
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
