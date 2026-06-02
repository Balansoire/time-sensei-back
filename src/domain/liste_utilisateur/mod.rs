pub mod repository;
pub mod service;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::liste_fiche::ListeFiche;

#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum TypeListeFiche {
  Hiragana,
  Katakana,
  Kanji,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ListeUtilisateur {
  pub id: Uuid,
  pub user_id: Uuid,
  pub type_liste: TypeListeFiche,

  pub nombre_revisions_liste: u32,
  pub liste_fiche: ListeFiche,
}

impl ListeUtilisateur {
  pub fn new(id: Uuid, user_id: Uuid, type_liste: TypeListeFiche, liste_fiche: ListeFiche) -> Self {
    Self {
      id,
      user_id,
      type_liste,
      nombre_revisions_liste: 0,
      liste_fiche,
    }
  }

  pub fn new_without_list(id: Uuid, user_id: Uuid, type_liste: TypeListeFiche) -> Self{
    Self {
      id,
      user_id,
      type_liste,
      nombre_revisions_liste: 0,
      liste_fiche: ListeFiche::new(),
    }
  }
}