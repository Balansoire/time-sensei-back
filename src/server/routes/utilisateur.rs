use axum::{Router, routing::{delete, get, post}};

use crate::{domain::{utilisateur::{repository::UtilisateurRepository, service::UtilisateurService}}, server::handlers::utilisateur_handler};

pub fn routes<
  R: UtilisateurRepository,
>(state: UtilisateurService<R>) -> Router {
  Router::new()
    .route(
      "/all",
      get(utilisateur_handler::get_all_users),
    )
    .route(
      "/{id}",
      get(utilisateur_handler::get_user),
    )
    .route(
      "/{id}/stats",
      get(utilisateur_handler::get_stats)
    )
    .route(
      "/",
      post(utilisateur_handler::create_user),
    )
    .route(
      "/{id}",
      delete(utilisateur_handler::delete_user)
    )
    .route(
      "/{id}/change_pseudo",
      post(utilisateur_handler::change_pseudo)
    )
    .route(
      "/{id}/change_email",
      post(utilisateur_handler::change_email)
    )
    .with_state(state)
}