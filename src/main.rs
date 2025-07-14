#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::data::ToByteUnit;
use rocket::Data;
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, RawField};
use std::collections::HashMap;
use std::io::Cursor;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

#[post("/convert", data = "<data>")]
async fn convert(content_type: &ContentType, data: Data<'_>) -> Result<(ContentType, Vec<u8>), Status> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::raw("file").size_limit(10.megabytes().into()));
    options.allowed_fields.push(MultipartFormDataField::text("type"));

    let multipart_form_data = MultipartFormData::parse(content_type, data, options).await
        .map_err(|_| Status::BadRequest)?;

    let file_field = multipart_form_data.raw.get("file")
        .and_then(|files| files.get(0))
        .ok_or(Status::BadRequest)?;

    let bytes = file_field.raw.clone();

    Ok((ContentType::Binary, bytes))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, convert])
        .attach(Template::fairing())
}
