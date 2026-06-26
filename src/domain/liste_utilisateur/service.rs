use tracing::error;
use uuid::Uuid;

use crate::{
  domain::{
    liste_fiche::{self, ListeFiche, ListeRevision},
    liste_utilisateur::{ListeUtilisateur, TypeListeFiche, repository::ListeUtilisateurRepository},
    utilisateur::{StatUtilisateur, repository::UtilisateurRepository},
  },
  server::handlers::liste_utilisateur_handler::ResultatSessionPayload,
};

#[derive(Clone)]
pub struct ListeUtilisateurService<L: ListeUtilisateurRepository, U: UtilisateurRepository> {
  repo: L,
  user_repo: U,
}

impl<L: ListeUtilisateurRepository, U: UtilisateurRepository> ListeUtilisateurService<L, U> {
  pub fn new(repo: L, user_repo: U) -> Self { Self { repo, user_repo } }

  async fn actualiser_stats_utilisateur(&mut self, user_id: Uuid, stat: &String) -> Option<StatUtilisateur> {
    if let Some(mut existing_stats) = self.user_repo.get_stats(user_id).await {

      if let Some(value) = existing_stats.stats.get_mut(stat) {
        *value += 1;
      } else {
        existing_stats.stats.insert(stat.clone(), 1);
      }
      existing_stats.nombre_total_revisions += 1;
      self.user_repo.update_stats(existing_stats).await
    } else {
      let mut new_stats = StatUtilisateur::new(Uuid::new_v4(), user_id);
      new_stats.nombre_total_revisions += 1;
      self.user_repo.create_stats(new_stats).await
    }
  }

  // CREATE
  pub async fn generate(&mut self, user_id: Uuid) -> Option<Vec<ListeUtilisateur>> {
    if let Some(_) = self.find_by_user_id(user_id).await {
      error!("Failed to generate lists for user: {}", user_id);
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
  pub async fn resultat_revision(&mut self, id: Uuid, index: usize, resultat: bool) -> Option<ListeUtilisateur> {
    if let Some(mut liste_utilisateur) = self.repo.find_by_id(id).await {
      liste_utilisateur.liste_fiche.resultat_revision(index, resultat);
      liste_utilisateur.nombre_revisions_liste += 1;

      if let Some(updated_liste) = self.repo.update(liste_utilisateur).await {
        self.actualiser_stats_utilisateur(updated_liste.user_id, &updated_liste.type_liste.to_string()).await;
        Some(updated_liste)
      } else {
        None
      }
    } else {
      None
    }
  }

  pub async fn resultat_revision_groupe(&mut self, id: Uuid, groupe: &mut Vec<usize>, index: usize, resultat: bool) -> Option<ListeUtilisateur> {
    if let Some(mut liste_utilisateur) = self.repo.find_by_id(id).await {
      liste_utilisateur.liste_fiche.resultat_revision_groupe(groupe, index, resultat);
      liste_utilisateur.nombre_revisions_liste += 1;

      if let Some(updated_liste) = self.repo.update(liste_utilisateur).await {
        self.actualiser_stats_utilisateur(updated_liste.user_id, &updated_liste.type_liste.to_string()).await;
        Some(updated_liste)
      } else {
        None
      }
    } else {
      None
    }
  }

  pub async fn resultat_session(&mut self, id: Uuid, session: &mut Vec<ResultatSessionPayload>) -> Option<ListeUtilisateur> {
    if let Some(mut liste_utilisateur) = self.repo.find_by_id(id).await {
      let user_id = liste_utilisateur.user_id;
      let mut reponses = Vec::new();
      let mut index = 0;
      for fiche in &liste_utilisateur.liste_fiche {
        if let Some(reponse) = session.iter().find(|reponse| reponse.fiche_id == fiche.id) {
          reponses.push((index, reponse.result));
        }
        index += 1;
      }
      for reponse in reponses {
        liste_utilisateur.liste_fiche.resultat_revision(reponse.0, reponse.1);
        self.actualiser_stats_utilisateur(user_id, &liste_utilisateur.type_liste.to_string()).await;
      }

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