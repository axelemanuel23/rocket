#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rocket!"
}

#[post("/")]
fn indexPost() -> &'static str {
    "Esto es un post"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, indexPost])
}
