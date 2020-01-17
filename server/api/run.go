package api

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/go-chi/cors"
	"gitlab.com/bloom42/bloom/core/consts"
	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
	accountshandler "gitlab.com/bloom42/bloom/server/bloom/accounts/handler"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
	"gitlab.com/bloom42/libs/rz-go/rzhttp"
)

func Run() error {
	var allowedOrigins []string
	router := chi.NewRouter()

	// replace size field name by latency and disable userAgent logging
	loggingMiddleware := rzhttp.Handler(log.Logger(), rzhttp.Duration("latency"))

	accountsHandler := rpcaccounts.NewAccountsServer(accountshandler.Handler{}, nil)

	/*
		router.Use(SetRequestID)
		router.Use(rzhttp.Handler(log.Logger()))
		router.Use(injectLoggerMiddleware(log.Logger()))
		router.Use(middleware.Recoverer)
		router.Use(middleware.Timeout(30 * time.Second))
	*/

	// here the order matters, otherwise loggingMiddleware won't see the request ID
	router.Use(middleware.RealIP)
	router.Use(SetRequestIDMiddleware)
	router.Use(loggingMiddleware)
	router.Use(SetLoggerMiddleware(log.Logger()))
	router.Use(middleware.Recoverer)
	// router.Use(middleware.Timeout(60 * time.Second))
	if config.Config.Env == consts.ENV_PRODUCTION {
		allowedOrigins = []string{"https://*.bloom.sh", "https://bloom.sh"}
	} else {
		allowedOrigins = []string{"*"}
	}
	cors := cors.New(cors.Options{
		AllowedOrigins:   allowedOrigins,
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "Origin"},
		ExposedHeaders:   []string{},
		AllowCredentials: false,
		MaxAge:           3600,
	})
	router.Use(cors.Handler)
	if config.Config.Env != consts.ENV_DEVELOPMENT {
		router.Use(SetSecurityHeadersMiddleware)
	}
	router.Use(SetContextMiddleware)

	router.Get("/", HelloWorlHandler)
	router.Mount(accountsHandler.PathPrefix(), accountsHandler)
	router.NotFound(http.HandlerFunc(NotFoundHandler))

	log.Info("starting server", rz.Uint16("port", config.Config.Port))
	return http.ListenAndServe(fmt.Sprintf(":%d", config.Config.Port), router)
}
