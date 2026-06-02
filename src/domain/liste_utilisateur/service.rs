use uuid::Uuid;

use crate::domain::{liste_fiche::{self, ListeFiche, ListeRevision}, liste_utilisateur::{ListeUtilisateur, TypeListeFiche, repository::ListeUtilisateurRepository}};

#[derive(Clone)]
pub struct ListeUtilisateurService<R: ListeUtilisateurRepository> {
  repo: R
}

impl<R: ListeUtilisateurRepository> ListeUtilisateurService<R> {
  pub fn new(repo: R) -> Self { Self { repo } }

  // CREATE
  pub async fn generate(&mut self, user_id: Uuid) -> Option<Vec<ListeUtilisateur>> {
    if let Some(_) = self.find_by_user_id(user_id).await {
      return None
    }
    
    let (hiragana, katakana, kanji) = liste_fiche::generate();
    
    let hiragana = self.create(
      user_id,
      TypeListeFiche::Hiragana,
      hiragana,
    ).await.expect("Hiragana should be created");
    let katakana = self.create(
      user_id,
      TypeListeFiche::Katakana,
      katakana,
    ).await.expect("Katakana should be created");
    let kanji = self.create(
      user_id,
      TypeListeFiche::Kanji,
      kanji,
    ).await.expect("Kanji should be created");

    Some(vec![hiragana, katakana, kanji])
  }

  pub async fn create(&mut self,
    user_id: Uuid,
    type_liste: TypeListeFiche,
    liste: ListeFiche
  ) -> Option<ListeUtilisateur> {
    self.repo.create(user_id, type_liste, liste).await
  }

  // READ
  pub async fn find_by_id(&self, id: Uuid) -> Option<ListeUtilisateur> {
    self.repo.find_by_id(id).await
  }

  pub async fn find_by_user_id(&self, user_id: Uuid) -> Option<Vec<ListeUtilisateur>> {
    self.repo.find_by_user_id(user_id).await
  }

  pub async fn nouvelle_revision(&self, id: Uuid) -> Option<usize> {
    if let Some(liste_utilisateur) = self.repo.find_by_id(id).await {
      Some(liste_utilisateur.liste_fiche.nouvelle_revision())
    } else {
      None
    }
  }

  pub async fn revision_groupe(&self, id: Uuid, groupe: &mut Vec<usize>) -> Option<usize> {
    if let Some(liste_utilisateur) = self.repo.find_by_id(id).await {
      Some(liste_utilisateur.liste_fiche.revision_groupe(groupe))
    } else {
      None
    }
  }

  pub async fn groupe_de_revision(&self, id: Uuid, groupe: &String) -> Option<Vec<usize>> {
    if let Some(liste_utilisateur) = self.repo.find_by_id(id).await {
      Some(liste_utilisateur.liste_fiche.groupe_de_revision(groupe))
    } else {
      None
    }
  }

  // UPDATE
  pub async fn resultat_revision (&mut self, id: Uuid, index: usize, resultat: bool) -> Option<ListeUtilisateur> {
    if let Some(mut liste_utilisateur) = self.repo.find_by_id(id).await {
      liste_utilisateur.liste_fiche.resultat_revision(index, resultat);
      liste_utilisateur.nombre_revisions_liste += 1;
      self.repo.update(liste_utilisateur).await
    } else {
      None
    }
  }
  
  pub async fn resultat_revision_groupe (&mut self, id: Uuid, groupe: &mut Vec<usize>, index: usize, resultat: bool) -> Option<ListeUtilisateur> {
    if let Some(mut liste_utilisateur) = self.repo.find_by_id(id).await {
      liste_utilisateur.liste_fiche.resultat_revision_groupe(groupe, index, resultat);
      liste_utilisateur.nombre_revisions_liste += 1;
      self.repo.update(liste_utilisateur).await
    } else {
      None
    }
  }

  // DELETE
  pub async fn delete(&mut self, id: Uuid) -> Option<ListeUtilisateur> {
    self.repo.delete(id).await
  }

  pub async fn delete_by_user_id(&mut self, user_id: Uuid) -> Option<Vec<ListeUtilisateur>> {
    if let Some(listes) = self.repo.find_by_user_id(user_id).await {
      let mut deleted = Vec::new();
      for liste in listes {
        deleted.push(self.delete(liste.id).await.expect("list should be deleted"));
      }
      Some(deleted)
    } else {
      None
    }
  }
}