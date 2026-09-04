use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use itertools::Itertools;
use serde::Serialize;

use crate::ALISTRAL_CLIENT;

/// Lunch a metrics server to load into grafana
#[derive(clap::Parser, Debug, Clone)]
pub struct MetricsCommand {}

impl MetricsCommand {
    pub async fn run(&self) {
        HttpServer::new(|| App::new().service(hello))
            .bind(("127.0.0.1", 4115))
            .unwrap()
            .run()
            .await
            .unwrap();
    }
}

#[get("/stats/listen_count")]
async fn hello() -> impl Responder {
    let stats = ALISTRAL_CLIENT
        .statistics_of_user("RustyNova".to_string())
        .await;
    let mut listens = stats.listens().iter().collect_vec();
    listens.sort_by_key(|l| l.listened_at);

    let mut data = Vec::with_capacity(listens.len());
    let mut count = 0;
    for listen in listens {
        count += 1;
        data.push(Datapoint {
            listen_count: count,
            timestamp: listen.listened_at,
        });
    }

    HttpResponse::Ok().json(DataResponse {
        listen_count: count,
        series: data,
    })
}

#[derive(Debug, Serialize)]
struct DataResponse {
    pub listen_count: u64,
    pub series: Vec<Datapoint>,
}

#[derive(Debug, Serialize)]
struct Datapoint {
    pub listen_count: u64,
    pub timestamp: i64,
}
