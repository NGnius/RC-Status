use rocket_contrib::templates::Template;
use rocket::http::Status;

#[get("/cardlife")]
pub fn cardlife() -> Result<Template, Status> {
    Ok(Template::render("cardlife", &*super::CL_CONTEXT.read().unwrap()))
}
