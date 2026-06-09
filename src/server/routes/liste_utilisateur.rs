use axum::{Router, routing::{delete, get, post}};

use crate::{domain::{liste_utilisateur::repository::ListeUtilisateurRepository, utilisateur::repository::UtilisateurRepository}, server::{handlers::liste_utilisateur_handler, state::AppState}};

pub fn routes<R: UtilisateurRepository, L: ListeUtilisateurRepository>(state: AppState<R, L>) -> Router {
  Router::new()
    // CREATE
    .route(
      "/create",
      post(liste_utilisateur_handler::create)
    )
    .route(
      "/generate",
      post(liste_utilisateur_handler::generate)
    )
    // READ
    .route(
      "/{id}",
      get(liste_utilisateur_handler::find_by_id)
    )
    .route(
      "/utilisateur/{id}",
      get(liste_utilisateur_handler::find_by_user_id)
    )
    .route(
      "/{id}/revision",
      get(liste_utilisateur_handler::nouvelle_revision)
    )
    .route(
      "/{id}/groupe/",
      post(liste_utilisateur_handler::revision_groupe)
    )
    .route(
      "/{id}/groupe/{groupe}",
      get(liste_utilisateur_handler::groupe_de_revision)
    )
    // UPDATE
    .route(
      "/{id}/revision",
      post(liste_utilisateur_handler::resultat_revision)
    )
    .route(
      "/{id}/revision/groupe",
      post(liste_utilisateur_handler::resultat_revision_groupe)
    )
    // DELETE
    .route(
      "/{id}",
      delete(liste_utilisateur_handler::delete)
    )
    .route(
      "/utilisateur",
      delete(liste_utilisateur_handler::delete_by_user_id)
    )
    .with_state(state)
}