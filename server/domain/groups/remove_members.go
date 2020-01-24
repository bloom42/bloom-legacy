package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func RemoveMembers(ctx context.Context, tx *sqlx.Tx, user users.User, group Group, usernames []string) twirp.Error {
	logger := rz.FromCtx(ctx)
	var err error
	var remainingAdmins int

	if twerr := CheckUserIsGroupAdmin(ctx, tx, user.ID, group.ID); twerr != nil {
		return twerr
	}

	// delete memberships
	queryDeleteMemberships := `DELETE FROM groups_members
		INNER JOIN users ON users.id = groups_members.user_id
		WHERE users.username IN ($1)`
	_, err = tx.Exec(queryDeleteMemberships, usernames)
	if err != nil {
		logger.Error("groups.RemoveMembers: removing members", rz.Err(err))
		return twirp.InternalError(ErrorRemovingMembersMsg)
	}

	queryRemainingAdmins := `SELECT COUNT(*) FROM groups_members
		WHERE group_id = $1 AND role = $2`
	err = tx.Get(&remainingAdmins, queryRemainingAdmins, group.ID, RoleAdministrator)
	if err != nil {
		logger.Error("users.RemoveMembers: error fetching remaining admins", rz.Err(err))
		return twirp.InternalError(ErrorRemovingMembersMsg)
	}
	if remainingAdmins != 0 {
		return twirp.NewError(twirp.PermissionDenied, "At least one administrator should remain in group.")
	}

	return nil
}