use crate::domain::{liste_utilisateur::{repository::ListeUtilisateurRepository, service::ListeUtilisateurService}, utilisateur::{repository::UtilisateurRepository, service::UtilisateurService}};

#[derive(Clone)]
pub struct AppState<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository
 > {
  pub utilisateur_service: UtilisateurService<R>,
  pub liste_utilisateur_service: ListeUtilisateurService<L, R>,
}