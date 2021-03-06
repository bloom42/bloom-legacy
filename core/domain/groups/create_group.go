package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/keys"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/graphql"
)

func CreateGroup(params messages.GroupsCreateParams) (*model.Group, error) {
	client := api.Client()
	var err error
	ctx := context.Background()

	userMasterKey, err := keys.FindUserMasterKey(ctx, nil)
	defer crypto.Zeroize(userMasterKey)
	if err != nil {
		return nil, err
	}

	// generate and save a random master key
	groupMasterKey, err := crypto.NewAEADKey()
	defer crypto.Zeroize(groupMasterKey)
	if err != nil {
		return nil, err
	}

	encryptedGroupMasterKey, nonce, err := crypto.AEADEncrypt(userMasterKey, groupMasterKey, nil)
	defer crypto.Zeroize(encryptedGroupMasterKey)
	defer crypto.Zeroize(nonce)
	if err != nil {
		return nil, err
	}

	input := model.CreateGroupInput{
		Name:               params.Name,
		Description:        params.Description,
		EncryptedMasterKey: encryptedGroupMasterKey,
		MasterKeyNonce:     nonce,
	}
	var resp struct {
		CreateGroup model.Group `json:"createGroup"`
	}
	req := graphql.NewRequest(`
	mutation ($input: CreateGroupInput!) {
		createGroup(input: $input) {
			id
			createdAt
			avatarUrl
			name
			description
			members {
				totalCount
			}
		}
	}
	`)
	req.Var("input", input)

	err = client.Do(context.Background(), req, &resp)
	if err == nil {
		err = SaveGroup(ctx, nil, &resp.CreateGroup, groupMasterKey, "")
	}

	return &resp.CreateGroup, err
}
