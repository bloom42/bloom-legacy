package repository

/*

queryCreate := `INSERT INTO billing_payment_methods
		(id, created_at, updated_at, is_default, stripe_id, card_last_4, card_expiration_month, card_expiration_year, customer_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`
	_, err = tx.Exec(queryCreate, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.IsDefault, ret.StripeID, ret.CardLast4,
		ret.CardExpirationMonth, ret.CardExpirationYear, ret.CustomerID)

*/
