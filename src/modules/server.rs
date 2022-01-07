use actix_web::{
    get, 
    web, 
    Responder
};

#[get("/{id}/{name}/index.html")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}