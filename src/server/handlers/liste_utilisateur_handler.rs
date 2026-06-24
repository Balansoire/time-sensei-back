use axum::{Json, extract::{Path, State}};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::domain::{liste_fiche::ListeFiche, liste_utilisateur::{ListeUtilisateur, TypeListeFiche, repository::ListeUtilisateurRepository, service::ListeUtilisateurService}};

#[derive(Deserialize)]
pub struct GenerateListePayload {
  user_id: Uuid
}

pub async fn generate<
  L: ListeUtilisateurRepository,
>(
  State(mut service): State<ListeUtilisateurService<L>>,
  Json(payload): Json<GenerateListePayload>
) -> Json<Option<Vec<ListeUtilisateur>>> {
  Json(service.generate(payload.user_id).await)
}

#[derive(Deserialize)]
pub struct CreateListePayload {
  user_id: Uuid,
  type_liste: TypeListeFiche,
  liste: ListeFiche
}

pub async fn create<
  L: ListeUtilisateurRepository,
>(
  State(mut service): State<ListeUtilisateurService<L>>,
  Json(payload): Json<CreateListePayload>
) -> Json<Option<ListeUtilisateur>> {
  Json(service.create(payload.user_id, payload.type_liste, payload.liste).await)
}

pub async fn find_by_id<
  L: ListeUtilisateurRepository,
>(
  State(service): State<ListeUtilisateurService<L>>,
  Path(id): Path<Uuid>,
) -> Json<Option<ListeUtilisateur>> {
  info!("recherche de ListeUtilisateur: {}", id);
  Json(service.find_by_id(id).await)
}

pub async fn find_by_user_id<
  L: ListeUtilisateurRepository,
>(
  State(service): State<ListeUtilisateurService<L>>,
  Path(user_id): Path<Uuid>,
) -> Json<Option<Vec<ListeUtilisateur>>> {
  info!("recherche des ListeUtilisateur de l'utilisateur: {}", user_id);
  Json(service.find_by_user_id(user_id).await)
}

pub async fn nouvelle_revision<
  L: ListeUtilisateurRepository,
>(
  State(service): State<ListeUtilisateurService<L>>,
  Path(id): Path<Uuid>,
) -> Json<Option<usize>> {
  Json(service.nouvelle_revision(id).await)
}

#[derive(Deserialize)]
pub struct NouvelleRevisionGroupePayload {
  groupe: Vec<usize>
}

pub async fn revision_groupe<
  L: ListeUtilisateurRepository,
>(
  State(service): State<ListeUtilisateurService<L>>,
  Path(id): Path<Uuid>,
  Json(mut payload): Json<NouvelleRevisionGroupePayload>,
) -> Json<Option<usize>> {
  Json(service.revision_groupe(id, &mut payload.groupe).await)
}

pub async fn groupe_de_revision<
  L: ListeUtilisateurRepository,
>(
  State(service): State<ListeUtilisateurService<L>>,
  Path(id): Path<Uuid>,
  Path(groupe): Path<String>,
) -> Json<Option<Vec<usize>>> {
  Json(service.groupe_de_revision(id, &groupe).await)
}

#[derive(Deserialize)]
pub struct ResultatRevisionPayload {
  index: usize,
  resultat: bool
}

pub async fn resultat_revision<
  L: ListeUtilisateurRepository,
>(
  State(mut service): State<ListeUtilisateurService<L>>,
  Path(id): Path<Uuid>,
  Json(payload): Json<ResultatRevisionPayload>,
) -> Json<Option<ListeUtilisateur>> {
  Json(service.resultat_revision(id, payload.index, payload.resultat).await)
}

#[derive(Deserialize)]
pub struct ResultatRevisionGroupePayload {
  index: usize,
  groupe: Vec<usize>,
  resultat: bool
}

pub async fn resultat_revision_groupe<
  L: ListeUtilisateurRepository,
>(
  State(mut service): State<ListeUtilisateurService<L>>,
  Path(id): Path<Uuid>,
  Json(mut payload): Json<ResultatRevisionGroupePayload>,
) -> Json<Option<ListeUtilisateur>> {
  Json(service.resultat_revision_groupe(id, &mut payload.groupe, payload.index, payload.resultat).await)
}

#[derive(Deserialize)]
pub struct ResultatSessionPayload {
  pub fiche_id: Uuid,
  pub result: bool,
}

pub async fn resultat_session<
  L: ListeUtilisateurRepository,
  >(
    State(mut service): State<ListeUtilisateurService<L>>,
    Path(id): Path<Uuid>,
    Json(mut payload): Json<Vec<ResultatSessionPayload>>,
  ) -> Json<Option<ListeUtilisateur>> {
    info!("Fin de session pour: {}", id);
    Json(service.resultat_session(id, &mut payload).await)
  }

pub async fn delete<
  L: ListeUtilisateurRepository,
>(
  State(mut service): State<ListeUtilisateurService<L>>,
  Path(id): Path<Uuid>,
) -> Json<Option<ListeUtilisateur>> {
  Json(service.delete(id).await)
}

pub async fn delete_by_user_id<
  L: ListeUtilisateurRepository,
>(
  State(mut service): State<ListeUtilisateurService<L>>,
  Path(user_id): Path<Uuid>,
) -> Json<Option<Vec<ListeUtilisateur>>> {
  Json(service.delete_by_user_id(user_id).await)
}