package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindPlanByID(ctx context.Context, db db.Queryer, planID uuid.UUID) (ret billing.Plan, err error) {
	return
}
