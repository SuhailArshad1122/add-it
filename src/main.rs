use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[get("/")]
async fn aoa() -> impl Responder {
    HttpResponse::Ok().body("AOA everyone..............prepared by Suhail")
}
async fn add_number(req: HttpRequest) -> impl Responder {
    let number  = req.match_info().get("number").unwrap();
    let resp = format!("adding number 23 + {} = {:#?}  ",  number, number.parse::<i32>().unwrap()+23);
    HttpResponse::Ok().body(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(aoa)
            .service(
                web::resource("/number/{number}")
                .route(web::get().to(add_number))
            )
            
    })
        .bind("127.0.0.1:8088")?
        .run()
       .await
}