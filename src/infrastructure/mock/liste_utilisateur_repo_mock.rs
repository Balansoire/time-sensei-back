use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{liste_fiche::ListeFiche, liste_utilisateur::{ListeUtilisateur, TypeListeFiche, repository::ListeUtilisateurRepository}};

#[derive(Clone)]
pub struct ListeUtilisateurRepoMock {
  listes: Arc<Mutex<Vec<ListeUtilisateur>>>
}

impl ListeUtilisateurRepoMock {
  pub fn new() -> Self {
    Self { listes: Arc::new(Mutex::new(vec![])) }
  }
}

#[async_trait]
impl ListeUtilisateurRepository for ListeUtilisateurRepoMock {
  
  // CREATE
  async fn create(
    &mut self,
    user_id: Uuid,
    type_liste: TypeListeFiche,
    liste_fiche: ListeFiche
  ) -> Option<ListeUtilisateur> {
    let mut listes = self.listes.lock().unwrap();
    for liste in listes.iter() {
      if (liste.user_id == user_id) && (liste.type_liste == type_liste) {
        return None
      }
    }
    
    let liste = ListeUtilisateur::new(
      Uuid::new_v4(),
      user_id,
      type_liste,
      liste_fiche
    );

    listes.push(liste.clone());
    Some(liste)
  }

  // READ
  async fn find_by_id(&self, id: Uuid) -> Option<ListeUtilisateur> {
    self
      .listes
      .lock()
      .unwrap()
      .iter()
      .find(|l| l.id == id)
      .cloned()
  }
  
  async fn find_by_user_id(&self, user_id: Uuid) -> Option<Vec<ListeUtilisateur>> {
    let mut listes = Vec::new();
    for l in self.listes.lock().unwrap().iter()
      .filter(|l| l.user_id == user_id) {
        listes.push(l.clone());
      }
    if listes.len() > 0 {
      Some(listes)
    } else {
      None
    }
  }

  // UPDATE
  async fn update(&mut self, liste: ListeUtilisateur) -> Option<ListeUtilisateur> {
    let mut listes = self.listes.lock().unwrap();
    if let Some(l) = listes.iter_mut().find(|u| u.id == liste.id) {
      *l = liste.clone();
      Some(l.clone())
    } else {
      None
    }
  }

  // DELETE
  async fn delete(&mut self, id: Uuid) -> Option<ListeUtilisateur> {
    let mut listes = self.listes.lock().unwrap();
    for i in 0..listes.len() {
      if let Some(user) = listes.get(i) {
        if user.id == id {
          return Some(listes.remove(i))
        }
      }
    }
    None
  }
}