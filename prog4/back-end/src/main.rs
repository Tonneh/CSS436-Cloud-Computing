use back_end::http_call::get_site_data::get_names;
use back_end::s3::upload::s3_upload;
use back_end::file::write::write;
use back_end::file::read::read;
use back_end::file::parse::parse_input_file;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = get_names("https://s3-us-west-2.amazonaws.com/css490/input.txt").await?;
    write("data_to_upload_s3.txt", name)?;
    s3_upload("data_to_upload_s3.txt", "tonyleprog4", "names.txt").await?;
    let map = parse_input_file("data_to_upload_s3.txt").unwrap();
    println!("{:#?}", map);

    Ok(())
}