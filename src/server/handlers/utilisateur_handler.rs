use axum::{Json, extract::{Path, State}};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::{domain::{liste_utilisateur::repository::ListeUtilisateurRepository, utilisateur::{
  Utilisateur, repository::UtilisateurRepository
}}, server::state::AppState};

pub async fn get_all_users<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>
) -> Json<Vec<Utilisateur>> {

  info!("Getting all users");

  let service = state.utilisateur_service;

  let users = service.get_all_users().await;

  Json(users)
}

pub async fn get_user<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(uuid): Path<Uuid>
) -> Json<Option<Utilisateur>> {

  info!("Getting user {}", &uuid);

  let service =state.utilisateur_service;

  let user = service.get_user_by_id(uuid).await;

  Json(user)
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
  email: String,
}

pub async fn create_user<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Json(payload): Json<CreateUserPayload>
) -> Json<Option<Utilisateur>> {

  info!("creating user for email: {}", &payload.email);

  let mut service = state.utilisateur_service;

  let created_user = service.create_user(payload.email).await;

  Json(created_user)
}

pub async fn delete_user<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(id): Path<Uuid>
) -> Json<Option<Utilisateur>> {

  info!("Deleting user {}", id);

  let mut service = state.utilisateur_service;

  let deleted_user = service.delete_user(id).await;

  Json(deleted_user)
}

#[derive(Deserialize)]
pub struct ChangePseudoPayload {
  new_pseudo: String,
}
pub async fn change_pseudo<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
  Json(payload): Json<ChangePseudoPayload>
) -> Json<Option<Utilisateur>> {
  let mut service = state.utilisateur_service;

  let user = service.change_pseudo(id, payload.new_pseudo).await;

  Json(user)
}

#[derive(Deserialize)]
pub struct ChangeEmailPayload {
  new_email: String,
}
pub async fn change_email<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(
  State(state): State<AppState<R, L>>,
  Path(id): Path<Uuid>,
  Json(payload): Json<ChangeEmailPayload>
) -> Json<Option<Utilisateur>> {
  let mut service = state.utilisateur_service;

  let user = service.change_email(id, payload.new_email).await;

  Json(user)
}