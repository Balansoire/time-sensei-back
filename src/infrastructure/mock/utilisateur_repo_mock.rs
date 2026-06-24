use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::utilisateur::{
  StatUtilisateur, Utilisateur, repository::UtilisateurRepository,
};

#[derive(Clone)]
pub struct UtilisateurRepoMock {
  pub users: Arc<Mutex<Vec<Utilisateur>>>,
  pub stats: Arc<Mutex<Vec<StatUtilisateur>>>,
}

impl UtilisateurRepoMock {
  pub fn new() -> Self {
    Self{
      users: Arc::new(Mutex::new(vec![])),
      stats: Arc::new(Mutex::new(vec![])),
    }
  }
}

#[async_trait]
impl UtilisateurRepository for UtilisateurRepoMock {

  // CREATE
  async fn create(&mut self, email: String) -> Option<Utilisateur> {
    let mut users = self.users.lock().unwrap();
    for utilisateur in users.iter() {
      if email == *utilisateur.get_email() { return None }
    }

    let id = Uuid::new_v4();

    let new_user = Utilisateur::new(id, &email, &email);
    users.push(new_user.clone());
    Some(new_user)
  }

  async fn create_stats(&mut self, stats: StatUtilisateur) -> Option<StatUtilisateur> {
    let mut stats_list = self.stats.lock().unwrap();
    stats_list.push(stats.clone());
    Some(stats)
  }

  // READ
  async fn get_all(&self) -> Vec<Utilisateur> {
    self.users.lock().unwrap().clone()
  }

  async fn find_by_id(&self, id: Uuid) -> Option<Utilisateur> {
    self.users.lock().unwrap().iter().find(|u| u.get_id() == id).cloned()
  }

  async fn find_by_email(&self, email: &String) -> Option<Utilisateur> {
    self.users.lock().unwrap().iter().find(|u| u.get_email() == email).cloned()
  }

  async fn get_stats(&self, id: Uuid) -> Option<StatUtilisateur> {
    self.stats.lock().unwrap().iter().find(|s| s.user_id == id).cloned()
  }

  // UPDATE
  async fn update(&mut self, user: Utilisateur) -> Option<Utilisateur> {
    let mut users = self.users.lock().unwrap();
    if let Some(u) = users.iter_mut().find(|u| u.get_id() == user.get_id()) {
      *u = user.clone();
      Some(u.clone())
    } else {
      None
    }
  }

  async fn update_stats(&mut self, stats: StatUtilisateur) -> Option<StatUtilisateur> {
    let mut stats_list = self.stats.lock().unwrap();
    if let Some(existing) = stats_list.iter_mut().find(|s| s.user_id == stats.user_id) {
      *existing = stats.clone();
      Some(stats)
    } else {
      stats_list.push(stats.clone());
      Some(stats)
    }
  }

  // DELETE
  async fn delete(&mut self, id: Uuid) -> Option<Utilisateur> {
    let mut users = self.users.lock().unwrap();
    for i in 0..users.len() {
      if let Some(user) = users.get(i) {
        if user.get_id() == id {
          return Some(users.remove(i))
        }
      }
    }
    None
  }
}