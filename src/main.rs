
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::{Path, State}, Extension, response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::{
    net::{SocketAddr},
    sync::{Arc, RwLock}, collections::HashMap, convert::Infallible
};

#[derive(Clone, Serialize, Deserialize)]
struct Movie {
        id: String,
        name: String,
        year: u16,
        was_good: bool
    }

impl Default for Movie{
    fn default()->Self{
        Self { id: "".to_string(), name: "".to_string(), year: 0, was_good: true }
    }
}

#[tokio::main]
async fn main() {
    // Create Axum server with the following endpoints:
    // 1. GET /movie/{id} - This should return back a movie given the id
    // 2. POST /movie - this should save move in a DB (HashMap<String, Movie>). This movie will be sent
    // via a JSON payload. 
    
    // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.
let addr = "127.0.0.1:9000".parse().unwrap();
    spawn_server(addr).await;
}


async fn get_handler(Path(movie_id): Path<String>, state: State<Arc<AppState>>)-> impl IntoResponse{
   if let Some(movie) = state.clone().read().unwrap().get(&movie_id){
        return Json(movie.clone());
    } 
    Json(Default::default())

}

type AppState = Arc<RwLock<HashMap<String, Movie>>>;



async fn post_handler(Json(movie): Json<Movie>,state: Extension<AppState>)-> Result<StatusCode, Infallible> {
    state.write().unwrap().insert(movie.id.clone(), movie);
    Ok(StatusCode::CREATED)

}   





async fn spawn_server(addr: SocketAddr){
        let shared_state = AppState::default();


    let app = Router::new()
        .route("/movie", post(|_: State<M>|async {post_handler}))
    
        .route("/movie{id}", get(get_handler))
        .layer(Extension(shared_state));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    tokio::spawn(async move{
        
    });
}
