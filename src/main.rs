use tracing::info;

use crate::{domain::{liste_utilisateur::service::ListeUtilisateurService, utilisateur::service::UtilisateurService}, infrastructure::mock::{liste_utilisateur_repo_mock::ListeUtilisateurRepoMock, utilisateur_repo_mock::UtilisateurRepoMock}, server::state::AppState};

mod server;
mod domain;
pub mod infrastructure;

#[tokio::main]
async fn main() {
    println!("Starting tracing");
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting Server");
    
    let user_repo = UtilisateurRepoMock::new();
    let listes_repo = ListeUtilisateurRepoMock::new();

    let mut state = AppState {
        utilisateur_service: UtilisateurService::new(user_repo.clone()),
        liste_utilisateur_service: ListeUtilisateurService::new(listes_repo, user_repo)
    };

    let first_user = state.utilisateur_service.create_user("dev@email.com".into()).await.unwrap();

    info!("Created first user with ID: {}", &first_user.id.as_braced());

    state.liste_utilisateur_service.generate(first_user.id).await;

    server::start(state).await;
}
