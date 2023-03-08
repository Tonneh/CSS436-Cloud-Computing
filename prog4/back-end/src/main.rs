#[macro_use]
extern crate rocket;

use aws_sdk_dynamodb::model::InputFormat::DynamodbJson;
use back_end::dynamodb::ddb_lib::*;
use back_end::file::file_lib::*;
use back_end::http_call::get_site_data::get_names;
use back_end::s3::s3_lib::s3_upload;
use std::collections::HashMap;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
use serde_json::json;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // let name = get_names("https://s3-us-west-2.amazonaws.com/css490/input.txt").await?;
//     // write("data_to_upload_s3.txt", name)?;
//     // s3_upload("data_to_upload_s3.txt", "tonyleprog4", "names.txt").await?;
//     // let map = parse_input_file("data_to_upload_s3.txt").unwrap();
//     // //println!("{:#?}", map);
//     // let combined_map = ddb_combine_maps(map).await?;
//     // //println!("{:#?}", &combined_map);
//     // ddb_upload(&combined_map).await?;
//     // ddb_query("Dimpsey").await?;
//     ddb_clear().await?;
//     Ok(())
// }

#[get("/query/<name>")]
async fn query(name: &str) -> String {
    match ddb_query(name).await {
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
    let combined_map = ddb_combine_maps(map).await.unwrap_or_default();
    match ddb_upload(&combined_map).await {
        Ok(uploaded_map) => {
            let json_string = serde_json::to_string_pretty(&uploaded_map).unwrap();
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
    let names: String = "Sucessfully Unloaded".to_string();
    names
}

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![query, load, unload])
}
