use crate::{
    api,
    log::macros::*,
    accounts::controllers,
    accounts::api::v1::models,
    accounts,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use actix_web::{
    ResponseError, AsyncResponder, Error, HttpMessage, FutureResponse,
    HttpRequest, HttpResponse, dev, multipart, error,
};
use futures::{Future, Stream, IntoFuture};
use futures::future;


pub fn put(req: &HttpRequest<api::State>) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return req.multipart()
        .map_err(error::ErrorInternalServerError)
        .map(handle_multipart_item)
        .flatten()
        .collect()
        .into_future()
        .map_err(|_| KernelError::Validation("file too large".to_string()))
        .and_then(move |avatar| {
            state.db
            .send(controllers::UpdateAvatar{
                account: auth.account.expect("unwrapping non none account"),
                avatar: avatar.get(0).expect("processing file").to_vec(),
                s3_bucket: state.config.s3_bucket(),
                s3_base_url: state.config.s3_base_url(),
                s3_client: state.s3_client.clone(),
                request_id,
                session_id: auth.session.expect("unwraping non none session").id,
            }).flatten()
        })
        .and_then(move |account| {
            let res = models::MeResponse{
                id: account.id,
                created_at: account.created_at,
                first_name: account.first_name,
                last_name: account.last_name,
                username: account.username,
                email: account.email,
                avatar_url: account.avatar_url,
            };
            let res = api::Response::data(res);
            Ok(HttpResponse::Ok().json(&res))
        })
        .from_err()
        .map_err(move |err: KernelError| {
            slog_error!(logger, "{}", err);
            return err;
        })
        .from_err()
        .responder();
}

fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>,
) -> Box<Stream<Item = Vec<u8>, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => {
            Box::new(read_file(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => {
            Box::new(
                mp.map_err(error::ErrorInternalServerError)
                    .map(handle_multipart_item)
                    .flatten(),
            )
        },
    }
}

fn read_file(
    field: multipart::Field<dev::Payload>,
) -> Box<Future<Item = Vec<u8>, Error = Error>> {
    Box::new(
        field
        .fold(Vec::new(), |mut acc, bytes| -> future::FutureResult<_, error::MultipartError> {
            acc.extend_from_slice(&bytes);
            if acc.len() > accounts::AVATAR_MAX_SIZE {
                return future::err(error::MultipartError::Payload(error::PayloadError::Overflow))
            }
            future::ok(acc)
        })
        .map_err(|e| error::ErrorInternalServerError(e)),
    )
}