pub mod service;
pub mod repository;

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Utilisateur {
  pub(crate) id: Uuid,
  pseudo: String,
  email: String,
}

impl Utilisateur {
  pub fn new(id: Uuid, pseudo: &String, email: &String) -> Self {
    Utilisateur { id, pseudo: pseudo.to_string(), email: email.to_string() }
  }

  pub fn get_id(&self) -> Uuid {
    self.id
  }

  pub fn get_pseudo(&self) -> &String {
    &self.pseudo
  }

  pub fn change_pseudo(&mut self, new_pseudo: String) -> &String {
    self.pseudo = new_pseudo;
    &self.pseudo
  }

  pub fn get_email(&self) -> &String {
    &self.email
  }

  pub fn change_email(&mut self, new_email: String) -> &String {
    self.email = new_email;
    &self.email
  }
}

pub struct StatUtilisateur {
  pub id: Uuid,
  pub user_id: Uuid,

  pub nombre_total_revisions: u32,
}

impl StatUtilisateur {
  pub fn new(id: Uuid, user_id: Uuid) -> Self {
    Self { id, user_id, nombre_total_revisions: 0 }
  }
}