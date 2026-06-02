use axum::{Router, routing::{delete, get, post}};

use crate::{domain::{liste_utilisateur::repository::ListeUtilisateurRepository, utilisateur::repository::UtilisateurRepository}, server::{handlers::liste_utilisateur_handler, state::{self, AppState}}};

pub fn routes<R: UtilisateurRepository, L: ListeUtilisateurRepository>(state: AppState<R, L>) -> Router {
  Router::new()
    // CREATE
    .route(
      "/liste_utilisateur/create",
      post(liste_utilisateur_handler::create)
    )
    .route(
      "/liste_utilisateur/generate",
      post(liste_utilisateur_handler::generate)
    )
    // READ
    .route(
      "/liste_utilisateur/{id}",
      get(liste_utilisateur_handler::find_by_id)
    )
    .route(
      "/liste_utilisateur/utilisateur/{id}",
      get(liste_utilisateur_handler::find_by_user_id)
    )
    .route(
      "/liste_utilisateur/{id}/revision",
      get(liste_utilisateur_handler::nouvelle_revision)
    )
    .route(
      "/liste_utilisateur/{id}/groupe/",
      post(liste_utilisateur_handler::revision_groupe)
    )
    .route(
      "/liste_utilisateur/{id}/groupe/{groupe}",
      get(liste_utilisateur_handler::groupe_de_revision)
    )
    // UPDATE
    .route(
      "/liste_utilisateur/{id}/revision",
      post(liste_utilisateur_handler::resultat_revision)
    )
    .route(
      "/liste_utilisateur/{id}/revision/groupe",
      post(liste_utilisateur_handler::resultat_revision_groupe)
    )
    // DELETE
    .route(
      "/liste_utilisateur/{id}",
      delete(liste_utilisateur_handler::delete)
    )
    .route(
      "/liste_utilisateur/utilisateur",
      delete(liste_utilisateur_handler::delete_by_user_id)
    )
    .with_state(state)
}