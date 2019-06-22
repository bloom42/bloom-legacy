use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    KernelError,
};
use futures::{
    future::Future,
    future::ok,
    future::Either,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use crate::{
    api::v1::models,
    controllers,
};


pub fn post(create_data: web::Json<models::CreateEventBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        state.db
        .send(controllers::CreateEvent{
            title: create_data.title.clone(),
            description: create_data.description.clone(),
            start_at: create_data.start_at,
            end_at: create_data.end_at,
            owner_id: auth.account.expect("error unwraping non none account").id,
            session_id: auth.session.expect("error unwraping non none session").id,
            request_id,
        })
        .map_err(|_| KernelError::ActixMailbox)
        .from_err()
        .and_then(move |new_event| {
            match new_event {
                Ok(new_event) => {
                    let res = models::EventBody{
                        id: new_event.id,
                        created_at: new_event.created_at,
                        updated_at: new_event.updated_at,
                        title: new_event.title,
                        description: new_event.description,
                        start_at: new_event.start_at,
                        end_at: new_event.end_at,
                    };
                    let res = api::Response::data(res);
                    ok(HttpResponse::Created().json(&res))
                },
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        })
    );
}