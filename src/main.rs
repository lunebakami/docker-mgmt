pub mod docker_client;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde_derive::Deserialize;
use docker_client::{DockerClientTrait, AsyncDockerClientTrait};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, Deserialize)]
pub struct HealthcheckParams {
    container: String,
}

#[get("/healthcheck")]
async fn healthcheck(req: HttpRequest) -> impl Responder {
    let params = web::Query::<HealthcheckParams>::from_query(req.query_string());

    let result = match params {  
        Ok(param) => param,
        Err(_) => return HttpResponse::BadRequest().body("Missing container parameter")
    };

    let docker_client = docker_client::DockerClient::new(&result.container);

    let healthcheck = docker_client.healthcheck().await;

    match healthcheck {
        Ok(success) => HttpResponse::Ok().body(success),
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}

#[post("/start")]
async fn start(req: HttpRequest) -> impl Responder {
    let params = web::Query::<HealthcheckParams>::from_query(req.query_string());

    let result = match params {  
        Ok(param) => param,
        Err(_) => return HttpResponse::BadRequest().body("Missing container parameter")
    };

    let docker_client = docker_client::DockerClient::new(&result.container);

    let start = docker_client.start().await;

    match start {
        Ok(_) => HttpResponse::Ok().body("Container started"),
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}


#[post("/stop")]
async fn stop(req: HttpRequest) -> impl Responder {
    let params = web::Query::<HealthcheckParams>::from_query(req.query_string());

    let result = match params {  
        Ok(param) => param,
        Err(_) => return HttpResponse::BadRequest().body("Missing container parameter")
    };

    let docker_client = docker_client::DockerClient::new(&result.container);

    let stop = docker_client.stop().await;

    match stop {
        Ok(_) => HttpResponse::Ok().body("Container stopped"),
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}

#[post("/restart")]
async fn restart(req: HttpRequest) -> impl Responder {
    let params = web::Query::<HealthcheckParams>::from_query(req.query_string());

    let result = match params {  
        Ok(param) => param,
        Err(_) => return HttpResponse::BadRequest().body("Missing container parameter")
    };

    let docker_client = docker_client::DockerClient::new(&result.container);

    let restart = docker_client.restart().await;

    match restart {
        Ok(_) => HttpResponse::Ok().body("Container restarted"),
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(start)
            .service(restart)
            .service(stop)
            .service(healthcheck)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
