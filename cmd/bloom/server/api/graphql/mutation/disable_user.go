package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
)

func (r *Resolver) DisableUser(ctx context.Context, id string) (bool, error) {
	ret := false
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	err := users.DisableUser(ctx, currentUser, id)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = true
	return ret, nil
}
