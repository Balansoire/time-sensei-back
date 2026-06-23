use axum::{Router, http::{HeaderValue, Method, header::{self, AUTHORIZATION, CONTENT_TYPE}}};
use tower_http::cors::{CorsLayer, Any};

use crate::{domain::{liste_utilisateur::repository::ListeUtilisateurRepository, utilisateur::repository::UtilisateurRepository}, server::{routes, state::AppState}};

pub fn create_router<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(state: AppState<R, L>) -> Router {

  let cors = CorsLayer::new()
    .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
    .allow_methods([
      Method::GET,
      Method::POST,
      Method::PUT,
      Method::DELETE,
    ])
    .allow_headers([
      AUTHORIZATION,
      CONTENT_TYPE,
    ]);

  Router::new()
    .merge(routes::routes(state))
    .layer(cors)
}