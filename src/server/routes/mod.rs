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
  Router::new()
    .merge(utilisateur::routes(state.clone()))
    .merge(liste_utilisateur::routes(state.clone()))
}