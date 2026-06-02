pub mod router;
pub mod routes;
pub mod handlers;
pub mod state;

use axum::serve;
use tokio::net::TcpListener;

use crate::{domain::{liste_utilisateur::repository::ListeUtilisateurRepository, utilisateur::repository::UtilisateurRepository}, server::state::AppState};

pub async fn start<
  R: UtilisateurRepository,
  L: ListeUtilisateurRepository,
>(state: AppState<R, L>) {

  let app = router::create_router(state);

  let listener = TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

  tracing::info!("Server running on port 3000");

  serve(listener, app)
    .await
    .unwrap();
}