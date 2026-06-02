use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{liste_fiche::ListeFiche, liste_utilisateur::{ListeUtilisateur, TypeListeFiche}};

#[async_trait]
pub trait ListeUtilisateurRepository: Clone + Send + Sync + 'static {

  // CREATE
  async fn create(
    &mut self,
    user_id: Uuid,
    type_liste: TypeListeFiche,
    liste_fiche: ListeFiche
  ) -> Option<ListeUtilisateur>;

  // READ
  async fn find_by_id(&self, id: Uuid) -> Option<ListeUtilisateur>;
  async fn find_by_user_id(&self, user_id: Uuid) -> Option<Vec<ListeUtilisateur>>;

  //UPDATE
  async fn update(&mut self, liste: ListeUtilisateur) -> Option<ListeUtilisateur>;

  // DELETE
  async fn delete(&mut self, id: Uuid) -> Option<ListeUtilisateur>;
}