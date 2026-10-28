use actix_web::{App, Error, HttpResponse, HttpServer, get, dev::ServiceRequest, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::{
    extractors::{
        basic::{self, BasicAuth},
        AuthenticationError
    },
    headers::www_authenticate, 
};


async fn validate(req: ServiceRequest, auth: BasicAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
     if auth.user_id() == "admin" && auth.password().unwrap() == "admin123"{
        Ok(req)
    } else {
        Err((
            AuthenticationError::new(www_authenticate::basic::Basic::default()).into(),
            req,   // no semicolon here
        ))
    }
}

#[get("/public")]
async fn get_public() -> HttpResponse {
    HttpResponse::Ok().body("Ruta Pública")
}

#[get("/private")]
async fn get_private(auth: BasicAuth) -> Result<HttpResponse, Error> {
    if auth.user_id() == "miusuario" && auth.password().unwrap() == "123456"{
        Ok(HttpResponse::Ok().body("Ruta Privada"))
    } else {
        Err(AuthenticationError::new(www_authenticate::basic::Basic::default()).into())
    }
}

#[get("/conficencial")]
async fn confidencial () -> HttpResponse {
    HttpResponse::Ok().body("Confidencial")
}

#[get("/super-conficencial")]
async fn super_confidencial () -> HttpResponse {
    HttpResponse::Ok().body("Súper Confidencial")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        let auth = HttpAuthentication::basic(validate);
        App::new()
        .service(
            web::scope("/admin")
            .wrap(auth)
            .service(confidencial)
            .service(super_confidencial)
        )
        .app_data(basic::Config::default().realm("privado"))
        .service(get_public)
        .service(get_private)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
