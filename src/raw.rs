use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/robocraftstaticdata")]
pub fn static_json() -> Result<Json<crate::staticdata::StaticData>, Status> {
    if let Ok(ctx) = crate::CONTEXT.read() {
        if ctx.staticdata_ok {
            return Ok(Json(ctx.staticdata.clone()));
            //return Ok(());
        } else {
            return Err(Status::ServiceUnavailable);
        }
    } else {
        return Err(Status::Conflict);
    }
}

#[get("/robots.txt")]
pub fn robots() -> &'static str {
"# Taking Robocraft literally, nice!

User-agent: *
Allow: /
Disallow: /robocraftstaticdata
Disallow: /static/

# Get out of here with that grindy lookalike
User-agent: Crossout
Disallow: /"
}
