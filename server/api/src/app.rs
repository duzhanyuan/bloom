
use actix_web::{
    App,
    middleware::cors::Cors,
    http::{header},
};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use std::str::FromStr;
use notes::api::v1 as notesv1;
use contacts::api::v1 as contactsv1;
use drive::api::v1 as drivev1;
use gallery::api::v1 as galleryv1;
use music::api::v1 as musicv1;
use bitflow::api::v1 as bitflowv1;
use sentry_actix::SentryMiddleware;
use kernel::{
    db::DbActor,
    api,
    api::middlewares,
    config,
    accounts::api::v1 as accountsv1,
};


pub fn init(db: actix::Addr<DbActor>, cfg: config::Config) -> App<api::State> {

    let region = Region::from_str(&cfg.aws_region()).expect("AWS region not valid");
    let api_state = api::State{
        db,
        config: cfg,
        s3_client: S3Client::new(region),
    };

    App::with_state(api_state.clone())
    .middleware(middlewares::RequestIdMiddleware)
    .middleware(middlewares::LoggerMiddleware)
    .middleware(middlewares::DefaultHeaders)
    .middleware(
        // cors 2 times because otherwise authmiddleware doesn't works...
        Cors::build()
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .max_age(3600)
        .finish()
    )
    .middleware(middlewares::AuthMiddleware)
    .middleware(SentryMiddleware::new())
    .default_resource(|r| r.f(api::route_404))
    .configure(|app| {
        Cors::for_app(app)
            // .allowed_origin("*")
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600)
            .resource("/", |r| r.method(http::Method::GET).f(api::index))

            // account
            .resource("/account/v1/registration/start", |r| r.method(http::Method::POST).with_config(accountsv1::registration::start::post, api::json_default_config))
            .resource("/account/v1/registration/verify", |r| r.method(http::Method::POST).with_config(accountsv1::registration::verify::post, api::json_default_config))
            .resource("/account/v1/registration/complete", |r| r.method(http::Method::POST).with_config(accountsv1::registration::complete::post, api::json_default_config))
            .resource("/account/v1/sign-in", |r| r.method(http::Method::POST).with_config(accountsv1::sign_in::post, api::json_default_config))
            .resource("/account/v1/sign-out", |r| r.method(http::Method::POST).f(accountsv1::sign_out::post))
            .resource("/account/v1/recover", |r| {
                r.method(http::Method::POST).with_config(accountsv1::recover::post, api::json_default_config);
                r.method(http::Method::PUT).with_config(accountsv1::recover::put, api::json_default_config);
            })
            .resource("/account/v1/me", |r| {
                r.method(http::Method::GET).f(accountsv1::me::get);
                r.method(http::Method::PUT).with_config(accountsv1::me::put, api::json_default_config);
            })
            .resource("/account/v1/me/password", |r| r.method(http::Method::PUT).with_config(accountsv1::me::password::put, api::json_default_config))
            .resource("/account/v1/me/avatar", |r| r.method(http::Method::PUT).f(accountsv1::me::avatar::put))
            .resource("/account/v1/me/email", |r| r.method(http::Method::PUT).with_config(accountsv1::me::email::put, api::json_default_config))
            .resource("/account/v1/me/email/verify", |r| r.method(http::Method::POST).with_config(accountsv1::me::email::verify::post, api::json_default_config))
            .resource("/account/v1/me/sessions", |r| r.method(http::Method::GET).f(accountsv1::me::sessions::get))
            .resource("/account/v1/me/sessions/{session_id}/revoke", |r| r.method(http::Method::POST).with(accountsv1::me::sessions::revoke::post))

            // notes
            .resource("/notes/v1/notes", |r| {
                r.method(http::Method::GET).f(notesv1::notes::get);
                r.method(http::Method::POST).with_config(notesv1::notes::post, api::json_default_config);
            })
            .resource("/notes/v1/notes/{note_id}", |r| {
                r.method(http::Method::DELETE).with(notesv1::notes::delete);
                r.method(http::Method::PUT).with_config(notesv1::notes::put, api::json_default_config_path);
            })
            .resource("/notes/v1/notes/{note_id}/archive", |r| r.method(http::Method::POST).with(notesv1::notes::archive::post))
            .resource("/notes/v1/notes/{note_id}/unarchive", |r| r.method(http::Method::POST).with(notesv1::notes::unarchive::post))
            .resource("/notes/v1/notes/{note_id}/remove", |r| r.method(http::Method::POST).with(notesv1::notes::remove::post))
            .resource("/notes/v1/notes/{note_id}/restore", |r| r.method(http::Method::POST).with(notesv1::notes::restore::post))
            .resource("/notes/v1/archive", |r| r.method(http::Method::GET).f(notesv1::archive::get))
            .resource("/notes/v1/trash", |r| r.method(http::Method::GET).f(notesv1::trash::get))

            // contacts
            .resource("/contacts/v1/contacts", |r| {
                r.method(http::Method::GET).f(contactsv1::contacts::get);
                r.method(http::Method::POST).with_config(contactsv1::contacts::post, api::json_default_config);
            })
            .resource("/contacts/v1/contacts/{contact_id}", |r| {
                r.method(http::Method::GET).with(contactsv1::contacts::id::get);
                r.method(http::Method::PUT).with_config(contactsv1::contacts::put, api::json_default_config_path);
                r.method(http::Method::DELETE).with(contactsv1::contacts::delete);
            })

            // drive
            .resource("/drive/v1/upload", |r| {
                r.method(http::Method::POST).with_config(drivev1::upload::post, api::json_default_config);
                r.method(http::Method::PUT).with_config(drivev1::upload::put, api::json_default_config);
            })
            .resource("/drive/v1/me", |r| r.method(http::Method::GET).f(drivev1::me::get))
            .resource("/drive/v1/folders", |r| {
                r.method(http::Method::GET).with(drivev1::folders::get);
                r.method(http::Method::POST).with_config(drivev1::folders::post, api::json_default_config);
            })
            .resource("/drive/v1/files/{file_id}/url", |r| r.method(http::Method::GET).with(drivev1::files::url::get))
            .resource("/drive/v1/files/move", |r| r.method(http::Method::POST).with_config(drivev1::files::move_::post, api::json_default_config))
            .resource("/drive/v1/files/restore", |r| r.method(http::Method::POST).with_config(drivev1::files::restore::post, api::json_default_config))
            .resource("/drive/v1/files/delete", |r| r.method(http::Method::POST).with_config(drivev1::files::delete::post, api::json_default_config))
            .resource("/drive/v1/files/copy", |r| r.method(http::Method::POST).with_config(drivev1::files::copy::post, api::json_default_config))
            .resource("/drive/v1/trash", |r| {
                r.method(http::Method::GET).f(drivev1::trash::get);
                r.method(http::Method::POST).with_config(drivev1::trash::post, api::json_default_config);
            })

            // gallery
            .resource("/gallery/v1/media", |r| r.method(http::Method::GET).f(galleryv1::media::get))
            .resource("/gallery/v1/albums", |r| {
                r.method(http::Method::GET).f(galleryv1::albums::get);
                r.method(http::Method::POST).with_config(galleryv1::albums::post, api::json_default_config);
            })
            .resource("/gallery/v1/albums/{album_id}", |r| {
                r.method(http::Method::GET).with(galleryv1::albums::album::get);
                r.method(http::Method::DELETE).with(galleryv1::albums::album::delete);
                r.method(http::Method::PUT).with_config(galleryv1::albums::album::put, api::json_default_config_path);
            })
            .resource("/gallery/v1/albums/{album_id}/add", |r| {
                r.method(http::Method::POST).with_config(galleryv1::albums::album::add::post, api::json_default_config_path);
            })
            .resource("/gallery/v1/albums/{album_id}/remove", |r| {
                r.method(http::Method::POST).with_config(galleryv1::albums::album::remove::post, api::json_default_config_path);
            })

            // music
            .resource("/music/v1/musics", |r| r.method(http::Method::GET).f(musicv1::musics::get))
            .resource("/music/v1/playlists", |r| {
                r.method(http::Method::GET).f(musicv1::playlists::get);
                r.method(http::Method::POST).with_config(musicv1::playlists::post, api::json_default_config);
            })
            .resource("/music/v1/playlists/{playlist_id}", |r| {
                r.method(http::Method::GET).with(musicv1::playlists::playlist::get);
                r.method(http::Method::DELETE).with(musicv1::playlists::playlist::delete);
                r.method(http::Method::PUT).with_config(musicv1::playlists::playlist::put, api::json_default_config_path);
            })
            .resource("/music/v1/playlists/{playlist_id}/add", |r| {
                r.method(http::Method::POST).with_config(musicv1::playlists::playlist::add::post, api::json_default_config_path);
            })
            .resource("/music/v1/playlists/{playlist_id}/remove", |r| {
                r.method(http::Method::POST).with_config(musicv1::playlists::playlist::remove::post, api::json_default_config_path);
            })

            // bitflow
            .resource("/bitflow/v1/downloads", |r| {
                r.method(http::Method::GET).f(bitflowv1::downloads::get);
                r.method(http::Method::POST).with_config(bitflowv1::downloads::post, api::json_default_config);
            })
            .resource("/bitflow/v1/downloads/remove", |r| {
                r.method(http::Method::POST).with_config(bitflowv1::downloads::remove::post, api::json_default_config);
            })
            .resource("/bitflow/v1/downloads/{download_id}", |r| {
                r.method(http::Method::GET).with(bitflowv1::downloads::download::get);
                r.method(http::Method::GET).with(bitflowv1::downloads::download::get);
            })
            .resource("/bitflow/v1/history", |r| {
                r.method(http::Method::GET).f(bitflowv1::history::get);
            })

            .register()
    })
}