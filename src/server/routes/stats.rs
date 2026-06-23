use axum::{Router, routing::{delete, get, post}};

use crate::server::state::AppState;

pub fn routes<R: UtilisateurRepository, L: ListeUtilisateurRepository>(state: AppState<R, L>) -> Router {
  Router::new()
    .route("/{id}", stats_handler::get_by_id)
    .with_state(state)
}