use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::utilisateur::{StatUtilisateur, Utilisateur};

#[async_trait]
pub trait UtilisateurRepository: Clone + Send + Sync + 'static {
  // CREATE
  async fn create(&mut self, email: String) -> Option<Utilisateur>;
  async fn create_stats(&mut self, stats: StatUtilisateur) -> Option<StatUtilisateur>;

  // READ
  async fn get_all(&self) -> Vec<Utilisateur>;
  async fn find_by_id(&self, id: Uuid) -> Option<Utilisateur>;
  async fn find_by_email(&self, email: &String) -> Option<Utilisateur>;
  async fn get_stats(&self, uuid: Uuid) -> Option<StatUtilisateur>;

  // UPDATE
  async fn update(&mut self, user: Utilisateur) -> Option<Utilisateur>;
  async fn update_stats(&mut self, stats: StatUtilisateur) -> Option<StatUtilisateur>;

  // DELETE
  async fn delete(&mut self, id: Uuid) -> Option<Utilisateur>;
}