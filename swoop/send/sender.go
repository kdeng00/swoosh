package send

import (
	"encoding/json"
	"fmt"
	"time"

	auxcfg "git.kundeng.us/phoenix/textsender-models/tx0/config/auxiliary"
	"git.kundeng.us/phoenix/textsender-models/tx0/contact"
	"git.kundeng.us/phoenix/textsender-models/tx0/message"
	"github.com/twilio/twilio-go"
	twilioApi "github.com/twilio/twilio-go/rest/api/v2010"

	"git.kundeng.us/phoenix/swoosh/swoop/types"
)

const Schedule_Type = "fixed"
const Schedulable_Limit_In_Seconds = 300

type MessageSender struct {
	Config *auxcfg.TwilioConfig
}

// Sends a message to a contact via Twilio
// Has support for scheduling a time based on if it is in the future
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
