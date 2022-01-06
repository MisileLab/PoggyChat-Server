mod main;

use actix_web::{
    get, 
    web, 
    App, 
    HttpServer, 
    Responder
};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}