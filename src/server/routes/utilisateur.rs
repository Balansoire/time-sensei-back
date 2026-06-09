use axum::{Router, routing::{delete, get, post}};

use crate::{domain::{liste_utilisateur::repository::ListeUtilisateurRepository, utilisateur::repository::UtilisateurRepository}, server::{handlers::utilisateur_handler, state::AppState}};

pub fn routes<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(state: AppState<R, L>) -> Router {
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
      "",
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