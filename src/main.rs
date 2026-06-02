use std::sync::Arc;

use tracing::info;

use crate::{domain::{liste_utilisateur::service::ListeUtilisateurService, utilisateur::service::UtilisateurService}, infrastructure::mock::{liste_utilisateur_repo_mock::ListeUtilisateurRepoMock, utilisateur_repo_mock::UtilisateurRepoMock}, server::state::AppState};

mod server;
mod domain;
pub mod infrastructure;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting Server");
    
    let user_repo = UtilisateurRepoMock::new();
    let listes_repo = ListeUtilisateurRepoMock::new();

    let state = AppState {
        utilisateur_service: UtilisateurService::new(user_repo),
        liste_utilisateur_service: ListeUtilisateurService::new(listes_repo)
    };

    server::start(state).await;
}
