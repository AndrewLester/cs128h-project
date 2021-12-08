#[macro_use] extern crate rocket;

mod algorithms;
mod graphing;
mod form;

use std::path::Path; 
use rocket::fs::NamedFile;
use rocket::data::{Data, ToByteUnit};
use rocket::response::Debug;
use algorithms::map_reduce::{map_reduce, get_graph_points};
use form::splice_form_boundary;
use graphing::create_bar_graph;

#[get("/")]
async fn index() -> Option<NamedFile> {
    let page_directory_path = 
        format!("{}/client", env!("CARGO_MANIFEST_DIR"));
    NamedFile::open(Path::new(&page_directory_path).join("index.html")).await.ok()
}

#[post("/mapreduce", data = "<data>")]
async fn mapreduce(data: Data<'_>) -> Result<String, Debug<std::io::Error>> {
    let to_reduce = data.open((1 as i64).mebibytes()).into_string().await?;
    // println!("{:#?}", to_reduce);
    let mut to_reduce = to_reduce.value;
    let delim = match to_reduce.contains("Content-Type: text/csv") {
        true => "\r",
        false => " "
    };

    let to_reduce = splice_form_boundary(&mut to_reduce);
    // println!("{:#?}", to_reduce);

    let results = get_graph_points(map_reduce(to_reduce, delim));

    let svg = create_bar_graph(
        "Relative Frequency of Words by Word Length",
        "percent occurence of word length",
        "word length",
        results
    );

    Ok(svg)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/", routes![mapreduce])
}
