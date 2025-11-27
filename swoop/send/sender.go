package send

import (
	"encoding/json"
	"fmt"
	"time"

	"git.kundeng.us/phoenix/textsender-models/tx0/config"
	"git.kundeng.us/phoenix/textsender-models/tx0/contact"
	"git.kundeng.us/phoenix/textsender-models/tx0/message"
	"github.com/twilio/twilio-go"
	twilioApi "github.com/twilio/twilio-go/rest/api/v2010"

	"git.kundeng.us/phoenix/swoosh/swoop/types"
)

type MessageSender struct {
	Config *config.TwiloConfig
}

func (m *MessageSender) Send(msg message.Message, number contact.Contact, sendTime *time.Time) (*types.TwilioResult, error) {
	if m.Config == nil {
		return nil, fmt.Errorf("Config has not been initialized")
	}
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

	if resp, err := client.Api.CreateMessage(params); err != nil {
		return nil, fmt.Errorf("Error sending message: %v", err)
	} else {
		if _, err := json.Marshal(*resp); err != nil {
			return nil, fmt.Errorf("Error parsing result: %v", err)
		} else {
			return resp, nil
		}
	}
}
