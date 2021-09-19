use rocket_contrib::templates::Template;
use rocket::http::Status;

#[get("/")]
pub fn index() -> Result<Template, Status> {
    Ok(Template::render("index", &*crate::CONTEXT.read().unwrap()))
}
