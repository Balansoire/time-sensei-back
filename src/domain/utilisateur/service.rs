
use uuid::Uuid;

use crate::domain::utilisateur::{
  Utilisateur,
  repository::UtilisateurRepository,
};

#[derive(Clone)]
pub struct UtilisateurService<R: UtilisateurRepository> {
  repo: R
}

impl<R: UtilisateurRepository> UtilisateurService<R> {
  pub fn new(repo: R) -> Self { Self{repo} }

  pub async fn create_user(&mut self, email: String) -> Option<Utilisateur> {
    self.repo.create(email).await
  }

  pub async fn get_all_users(&self) -> Vec<Utilisateur> {
    self.repo.get_all().await
  }

  pub async fn get_user_by_id(&self, uuid: Uuid) -> Option<Utilisateur> {
    self.repo.find_by_id(uuid).await
  }

  pub async fn change_pseudo(&mut self, id: Uuid, new_pseudo: String) -> Option<Utilisateur> {
    if let Some(mut user) = self.repo.find_by_id(id).await {
      user.change_pseudo(new_pseudo);
      self.repo.update(user).await
    } else {
      None
    }
  }

  pub async fn change_email(&mut self, id: Uuid, new_email: String) -> Option<Utilisateur> {
    if let Some(mut user) = self.repo.find_by_id(id).await {
      user.change_email(new_email);
      self.repo.update(user).await
    } else {
      None
    }
  }

  pub async fn delete_user(&mut self, id: Uuid) -> Option<Utilisateur> {
    self.repo.delete(id).await
  }
}