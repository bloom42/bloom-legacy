package billing

import "gitlab.com/bloom42/bloom/server/errors"

var (
	ErrStripeIDIsNotValid                        = errors.InvalidArgument("\"stripe_id\" is not valid.")
	ErrPaymentMethodIsAlreadyDefault             = errors.InvalidArgument("Payment method is already the default. Please change and try again.")
	ErrPaymentMethodNotFound                     = errors.NotFound("Payment method not found.")
	ErrUserIdAndGroupIdCantBeBothNonNullrr       = errors.InvalidArgument("\"user_id\" and \"group_id\" can't be both non null. Please fix and try again.")
	ErrPlanNotFound                              = errors.InvalidArgument("Plan not found.")
	ErrOldPlanIsTheSameAsNewPlan                 = errors.InvalidArgument("The new plan is the same as the new plan. Please fix and try again.")
	ErrTooMuchStorageUsedForNewPlan              = errors.InvalidArgument("Your used storage is superior to the allowed storage for the new plan. Please fix and try again.")
	ErrPlanStorageCantBeNegative                 = errors.InvalidArgument("Plan storage can't be negative. Please fix and try again.")
	ErrCantDeleteDefaultPlan                     = errors.InvalidArgument("Can't delete default plan. Please change and try again.")
	ErrCantDeletePlanWithSubscribers             = errors.InvalidArgument("Can't delete plan with subscribers. Please change and try again.")
	ErrRemovingDefaultPaymentMethodOnNonFreePlan = errors.InvalidArgument("Please change your plan to FREE before removeing your payment method.")
	ErrRemovingDefaultPaymentMethod              = errors.InvalidArgument("Please change your default payment method before removing it.")
	ErrInvalidCustomerStorage                    = errors.InvalidArgument("Invalid storage amount.")
)
