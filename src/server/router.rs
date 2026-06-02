use axum::Router;

use crate::{domain::{liste_utilisateur::repository::ListeUtilisateurRepository, utilisateur::repository::UtilisateurRepository}, server::{routes, state::AppState}};

pub fn create_router<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(state: AppState<R, L>) -> Router {
  Router::new()
    .merge(routes::routes(state))
}