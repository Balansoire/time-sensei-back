use axum::{Json, extract::{Path, State}};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::{domain::{liste_fiche::ListeFiche, liste_utilisateur::{ListeUtilisateur, TypeListeFiche, repository::ListeUtilisateurRepository}, utilisateur::repository::UtilisateurRepository}, server::state::AppState};

#[derive(Deserialize)]
pub struct GenerateListePayload {
  user_id: Uuid
}

pub async fn generate<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(mut state): State<AppState<R, L>>,
  Json(payload): Json<GenerateListePayload>
) -> Json<Option<Vec<ListeUtilisateur>>> {
  Json(state.liste_utilisateur_service.generate(payload.user_id).await)
}

#[derive(Deserialize)]
pub struct CreateListePayload {
  user_id: Uuid,
  type_liste: TypeListeFiche,
  liste: ListeFiche
}

pub async fn create<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(mut state): State<AppState<R, L>>,
  Json(payload): Json<CreateListePayload>
) -> Json<Option<ListeUtilisateur>> {
  Json(state.liste_utilisateur_service.create(payload.user_id, payload.type_liste, payload.liste).await)
}

pub async fn find_by_id<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
) -> Json<Option<ListeUtilisateur>> {
  info!("recherche de ListeUtilisateur: {}", id);
  Json(state.liste_utilisateur_service.find_by_id(id).await)
}

pub async fn find_by_user_id<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(user_id): Path<Uuid>,
) -> Json<Option<Vec<ListeUtilisateur>>> {
  info!("recherche des ListeUtilisateur de l'utilisateur: {}", user_id);
  Json(state.liste_utilisateur_service.find_by_user_id(user_id).await)
}

pub async fn nouvelle_revision<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
) -> Json<Option<usize>> {
  Json(state.liste_utilisateur_service.nouvelle_revision(id).await)
}

#[derive(Deserialize)]
pub struct NouvelleRevisionGroupePayload {
  groupe: Vec<usize>
}

pub async fn revision_groupe<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
  Json(mut payload): Json<NouvelleRevisionGroupePayload>,
) -> Json<Option<usize>> {
  Json(state.liste_utilisateur_service.revision_groupe(id, &mut payload.groupe).await)
}

pub async fn groupe_de_revision<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
  Path(groupe): Path<String>,
) -> Json<Option<Vec<usize>>> {
  Json(state.liste_utilisateur_service.groupe_de_revision(id, &groupe).await)
}

#[derive(Deserialize)]
pub struct ResultatRevisionPayload {
  index: usize,
  resultat: bool
}

pub async fn resultat_revision<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(mut state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
  Json(payload): Json<ResultatRevisionPayload>,
) -> Json<Option<ListeUtilisateur>> {
  Json(state.liste_utilisateur_service.resultat_revision(id, payload.index, payload.resultat).await)
}

#[derive(Deserialize)]
pub struct ResultatRevisionGroupePayload {
  index: usize,
  groupe: Vec<usize>,
  resultat: bool
}

pub async fn resultat_revision_groupe<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(mut state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
  Json(mut payload): Json<ResultatRevisionGroupePayload>,
) -> Json<Option<ListeUtilisateur>> {
  Json(state.liste_utilisateur_service.resultat_revision_groupe(id, &mut payload.groupe, payload.index, payload.resultat).await)
}

#[derive(Deserialize)]
pub struct ResultatSessionPayload {
  pub fiche_id: Uuid,
  pub result: bool,
}

pub async fn resultat_session<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
  >(
    State(mut state): State<AppState<R, L>>,
    Path(id): Path<Uuid>,
    Json(mut payload): Json<Vec<ResultatSessionPayload>>,
  ) -> Json<Option<ListeUtilisateur>> {
    info!("Fin de session pour: {}", id);
    Json(state.liste_utilisateur_service.resultat_session(id, &mut payload).await)
  }

pub async fn delete<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(mut state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
) -> Json<Option<ListeUtilisateur>> {
  Json(state.liste_utilisateur_service.delete(id).await)
}

pub async fn delete_by_user_id<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(mut state): State<AppState<R, L>>,
  Path(user_id): Path<Uuid>,
) -> Json<Option<Vec<ListeUtilisateur>>> {
  Json(state.liste_utilisateur_service.delete_by_user_id(user_id).await)
}