pub mod utilisateur;
pub mod liste_utilisateur;

use axum::{
  Router
};

use crate::{domain::{liste_utilisateur::repository::ListeUtilisateurRepository, utilisateur::repository::UtilisateurRepository}, server::state::AppState};

pub fn routes<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(state: AppState<R, L>) -> Router {
  let api_routes = Router::new()
    .nest("/utilisateur", utilisateur::routes(state.clone().utilisateur_service))
    .nest("/liste_utilisateur", liste_utilisateur::routes(state.clone().liste_utilisateur_service));

  Router::new().nest("/api", api_routes)
}