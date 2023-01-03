#[macro_use]
extern crate rocket;
use api::{review_handler, unit_handler, user_handler};
use dotenvy::dotenv;
use infrastructure::{establish_connection, ServerState};
use okapi::openapi3::OpenApi;
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let state = ServerState {
        db_pool: establish_connection(),
    };

    let mut building_rocket = rocket::build().manage(state).mount(
        "/api/rapi-doc",
        make_rapidoc(&RapiDocConfig {
            title: Some("Qut Review | RapiDoc".to_owned()),
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("General", "../../openapi.json")],
                ..Default::default()
            },
            hide_show: HideShowConfig {
                allow_spec_url_load: false,
                allow_spec_file_load: false,
                ..Default::default()
            },
            ..Default::default()
        }),
    );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    let custom_route_spec = (vec![], custom_openapi_spec());
    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/external" => custom_route_spec,
        "/api/unit" => unit_handler::get_routes_and_docs(&openapi_settings),
        "/api/user" => user_handler::get_routes_and_docs(&openapi_settings),
        "/api/review" => review_handler::get_routes_and_docs(&openapi_settings),
    };

    building_rocket
}

fn custom_openapi_spec() -> OpenApi {
    use rocket_okapi::okapi::openapi3::*;
    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "Qut Review".to_owned(),
            description: Some("Qut Review public API.".to_owned()),
            terms_of_service: Some(
                "https://github.com/QUT-IT-Chads/qut-review/blob/main/LICENSE".to_owned(),
            ),
            contact: Some(Contact {
                name: Some("Qut Review".to_owned()),
                url: Some("https://github.com/QUT-IT-Chads/qut-review".to_owned()),
                email: None,
                ..Default::default()
            }),
            license: Some(License {
                name: "MIT".to_owned(),
                url: Some(
                    "https://github.com/QUT-IT-Chads/qut-review/blob/main/LICENSE".to_owned(),
                ),
                ..Default::default()
            }),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            ..Default::default()
        },
        servers: vec![Server {
            url: "http://localhost:8000/".to_owned(),
            description: Some("Localhost".to_owned()),
            ..Default::default()
        }],
        ..Default::default()
    }
}
