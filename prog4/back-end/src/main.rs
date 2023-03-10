extern crate rocket;

use back_end::dynamodb::ddb_lib::*;
use back_end::file::file_lib::*;
use back_end::http_call::get_site_data::get_names;
use back_end::s3::s3_lib::s3_upload;

use rocket::*;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/query/full/<name>")]
async fn query_full_name(name: &str) -> String {
    match ddb_query_full_name(name).await {
        Ok(response) => {
            let json_string = serde_json::to_string_pretty(&response).unwrap();
            json_string
        }
        Err(err) => {
            format!("Error: {}", err.to_string())
        }
    }
}

#[get("/query/last/<name>")]
async fn query_last_name(name: &str) -> String {
    match ddb_query_last_name(name).await {
        Ok(response) => {
            let json_string = serde_json::to_string_pretty(&response).unwrap();
            json_string
        }
        Err(err) => {
            format!("Error: {}", err.to_string())
        }
    }
}
#[get("/query/first/<name>")]
async fn query_first_name(name: &str) -> String {
    match ddb_query_first_name(name).await {
        Ok(response) => {
            let json_string = serde_json::to_string_pretty(&response).unwrap();
            json_string
        }
        Err(err) => {
            format!("Error: {}", err.to_string())
        }
    }
}

#[get("/load")]
async fn load() -> String {
    let names = get_names("https://s3-us-west-2.amazonaws.com/css490/input.txt")
        .await
        .unwrap_or_default();
    write("data_to_upload_s3.txt", names).unwrap_or_default();
    s3_upload("data_to_upload_s3.txt", "tonyleprog4", "names.txt")
        .await
        .unwrap_or_default();
    let map = parse_input_file("data_to_upload_s3.txt").unwrap_or_default();
    let combined_map = ddb_combine_maps(map.clone()).await.unwrap_or_default();
    match ddb_upload(combined_map).await {
        Ok(uploaded_map) => {
            let json_string = serde_json::to_string_pretty(&map).unwrap();
            json_string
        }
        Err(err) => {
            format!("Error: {}", err.to_string())
        }
    }
}

#[get("/unload")]
async fn unload() -> String {
    ddb_clear().await.unwrap_or_default();
    let names: String = "Successfully Unloaded".to_string();
    names
}

#[get("/")]
async fn hello() -> String {
    "Tony API for Prog4 CSS436".to_string()
}

#[launch]
fn rocket() -> _ {
    build().attach(CORS).mount(
        "/",
        routes![
            hello,
            query_first_name,
            query_last_name,
            query_full_name,
            load,
            unload
        ],
    )
}
