use axum::{Json, extract::{Path, State}};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::domain::utilisateur::{
  StatUtilisateur, Utilisateur, repository::UtilisateurRepository, service::UtilisateurService
};

pub async fn get_all_users<
  R: UtilisateurRepository,
>(
  State(service): State<UtilisateurService<R>>
) -> Json<Vec<Utilisateur>> {

  info!("Getting all users");

  let users = service.get_all_users().await;

  Json(users)
}

pub async fn get_user<
  R: UtilisateurRepository,
>(
  State(service): State<UtilisateurService<R>>,
  Path(uuid): Path<Uuid>
) -> Json<Option<Utilisateur>> {

  info!("Getting user {}", &uuid);

  let user = service.get_user_by_id(uuid).await;

  Json(user)
}

pub async fn get_stats<
  R: UtilisateurRepository,
>(
  State(service): State<UtilisateurService<R>>,
  Path(uuid): Path<Uuid>
) -> Json<Option<StatUtilisateur>> {

  info!("Getting stats for user: {}", &uuid);

  let stats = service.get_stats(uuid).await;

  Json(stats)
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
  email: String,
}

pub async fn create_user<
  R: UtilisateurRepository,
>(
  State(mut service): State<UtilisateurService<R>>,
  Json(payload): Json<CreateUserPayload>
) -> Json<Option<Utilisateur>> {

  info!("creating user for email: {}", &payload.email);

  let created_user = service.create_user(payload.email).await;

  Json(created_user)
}

pub async fn delete_user<
  R: UtilisateurRepository,
>(
  State(mut service): State<UtilisateurService<R>>,
  Path(id): Path<Uuid>
) -> Json<Option<Utilisateur>> {

  info!("Deleting user {}", id);

  let deleted_user = service.delete_user(id).await;

  Json(deleted_user)
}

#[derive(Deserialize)]
pub struct ChangePseudoPayload {
  new_pseudo: String,
}
pub async fn change_pseudo<
  R: UtilisateurRepository,
>(
  State(mut service): State<UtilisateurService<R>>,
  Path(id): Path<Uuid>,
  Json(payload): Json<ChangePseudoPayload>
) -> Json<Option<Utilisateur>> {
  let user = service.change_pseudo(id, payload.new_pseudo).await;

  Json(user)
}

#[derive(Deserialize)]
pub struct ChangeEmailPayload {
  new_email: String,
}
pub async fn change_email<
  R: UtilisateurRepository,
>(
  State(mut service): State<UtilisateurService<R>>,
  Path(id): Path<Uuid>,
  Json(payload): Json<ChangeEmailPayload>
) -> Json<Option<Utilisateur>> {

  let user = service.change_email(id, payload.new_email).await;

  Json(user)
}