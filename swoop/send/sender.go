package send

import (
	"fmt"

	"git.kundeng.us/phoenix/textsender-models/pkg/contact"
)

type MessageSender struct {
	Numbers []contact.Contact
}

func (m *MessageSender) Send() error {
	if len(m.Numbers) == 0 {
		return fmt.Errorf("No numbers to send")
	}
	return nil
}
