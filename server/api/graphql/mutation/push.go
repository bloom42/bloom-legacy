package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/sync"
)

// Push is used to push changes
func (resolver *Resolver) Push(ctx context.Context, input model.PushInput) (ret *model.Push, err error) {
	repositories := []sync.RepositoryPush{}
	for _, repo := range input.Repositories {
		repositoryPush := sync.RepositoryPush{
			CurrentState: repo.CurrentState,
			GroupID:      repo.GroupID,
			Objects:      []sync.APIObject{},
		}
		for _, object := range repo.Objects {
			obj := sync.APIObject{
				ID:            object.ID,
				Algorithm:     object.Algorithm,
				EncryptedData: object.EncryptedData,
				EncryptedKey:  object.EncryptedKey,
				Nonce:         object.Nonce,
			}
			repositoryPush.Objects = append(repositoryPush.Objects, obj)
		}
		repositories = append(repositories, repositoryPush)
	}

	params := sync.PushParams{Repositories: repositories}
	result, err := resolver.syncService.Push(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.Push{Repositories: []*model.RepositoryPush{}}
	for _, push := range result.Repositories {
		resPush := &model.RepositoryPush{
			OldState: push.OldState,
			NewState: push.NewState,
			GroupID:  push.GroupID,
		}
		ret.Repositories = append(ret.Repositories, resPush)
	}
	return
}
