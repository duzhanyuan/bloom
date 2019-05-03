use crate::{
    api,
    accounts::api::v1::models,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
    },
    accounts::controllers,
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, Json, HttpRequest,
};
use futures::future::IntoFuture;
use rand::Rng;


pub fn post((registration_data, req): (Json<models::NewCodeBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();
    let state = req.state().clone();
    let logger = req.logger();
    let config = state.config.clone();
    let request_id = req.request_id().0;

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(400, 650))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::SendNewVerificationCode{
            pending_account_id: registration_data.id,
            config,
            request_id,
        }).flatten()
    )
    .and_then(move |_| {
        let res = api::Response::data(models::NoData{});
        Ok(HttpResponse::Ok().json(&res))
    })
    .map_err(move |err| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
    .responder();
}