use crate::{
    api,
    accounts::api::v1::models,
    log::macros::*,
    accounts::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
    },
    KernelError,
};
use std::time::Duration;
use futures::{
    future::{
        IntoFuture,
        Future,
        ok,
    },
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse,
};
use rand::Rng;


pub fn post(verify_data: web::Json<models::VerifyPendingAccountBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let mut rng = rand::thread_rng();
    let logger = req.logger();
    let request_id = req.request_id();

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(500, 650))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::VerifyPendingAccount{
            id: verify_data.id,
            code: verify_data.code.clone(),
            request_id: request_id.0,
        }).flatten()
    )
    .from_err()
    .and_then(move |res| {
       ok(HttpResponse::Ok().json(api::Response::data(models::NoData{})))
    })
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
}
