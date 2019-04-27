use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
};
use futures::future;
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
use crate::{
    api::v1::models,
    controllers,
};


pub fn put((upload_data, req): (Json<models::CompleteUploadSessionBody>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::CompleteUploadSession{
        upload_session_id: upload_data.upload_session_id.clone(),
        s3_bucket: state.config.s3_bucket(),
        s3_client: state.s3_client.clone(),
        account_id: auth.account.expect("error unwraping non none account").id,
        session_id: auth.session.expect("error unwraping non none session").id,
        request_id,
    })
    .from_err()
    .and_then(move |file| {
        match file {
            Ok(file) => {
               let file = models::FileBody{
                    id: file.id,
                    created_at: file.created_at,
                    updated_at: file.updated_at,
                    name: file.name,
                    type_: file.type_,
                    size: file.size,
                };
                let res = api::Response::data(file);
                Ok(HttpResponse::Created().json(&res))
            },
            Err(err) => Err(err),
        }
    })
    .from_err()
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
    .responder();
}