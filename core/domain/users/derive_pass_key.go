package users

import (
	"gitlab.com/bloom42/lily/crypto/kdf"
)

// TODO
func derivePassKey(username, password []byte) []byte {
	authKeySalt := padOrTrimBytes(username, 64)

	key, err := kdf.DeriveFromPassword(password, authKeySalt, kdf.KeySize256)
	if err != nil {
		return nil
	}
	keyID := []byte("com.bloom42.bloom.auth_key")
	key = append(key, keyID...)

	authKey, err := kdf.DeriveFromKey(key, kdf.KeySize256)
	if err != nil {
		return nil
	}

	return authKey
}
