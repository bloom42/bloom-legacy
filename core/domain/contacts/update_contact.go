package contacts

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
)

func UpdateContact(contact objects.Object) (objects.Object, error) {
	now := time.Now().UTC()
	var err error

	// TODO: validators
	cont, err := ObjectToContact(&contact)
	if err != nil {
		return contact, err
	}
	cleanContactCollections(cont)

	cleanedContact, err := objects.ToObject(contact.ID, kernel.OBJECT_TYPE_CONTACT, contact.CreatedAt, now, contact.GroupID, true, cont)
	if err != nil {
		return contact, err
	}

	err = objects.SaveObject(context.Background(), nil, cleanedContact)

	// request sync
	// objects.SyncChan <- true

	return *cleanedContact, err
}
