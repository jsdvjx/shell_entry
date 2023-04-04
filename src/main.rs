#[macro_use]
extern crate rocket;

use std::fs::read_to_string;
use std::io::BufReader;
use rocket::figment::Source::File;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use regex::Regex;


struct Host<'r>(&'r str);


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Host<'r> {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let host = req.host();
        match host {
            Some(host) => Outcome::Success(Host(host.domain().as_str())),
            None => Outcome::Failure((Status::BadRequest, "unknown host"))
        }
    }
}


#[get("/")]
fn index(host: Host<'_>) -> String {
    let mut list = host.0.split(".").map(|s| s.to_string()).collect::<Vec<String>>();
    let first = list[0].to_string();
    list.remove(0);
    let base_host = list.join(".");
    let file_name = format!("{}.sh", first);
    let content = read_to_string(file_name).unwrap_or_default();
    let re = Regex::new(r"\{\{([a-z]+)}}").unwrap();
    let replaced = re.replace_all(content.as_str(), |caps: &regex::Captures| {
        format!("{}", caps[1].to_string() + "." + &*base_host)
    });
    // content.unwrap_or_default()
    replaced.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
