use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use std::sync::Arc;
use tokio::sync::Mutex;

struct Lotto<'a, R: RngCore> {
    pot: Vec<u32>,
    rng: &'a mut R,
}

impl<'a, R: RngCore> Lotto<'a, R> {
    fn new(pot_size: u32, rng: &'a mut R) -> Self {
        Self {
            pot: (1..=pot_size).collect(),
            rng,
        }
    }

    fn take(&mut self, amount: usize) -> Vec<u32> {
        self.pot.shuffle(self.rng);
        self.pot.iter().take(amount).map(|e| e.to_owned()).collect()
    }
}

type SharedState = Arc<Mutex<SmallRng>>;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(SmallRng::from_entropy()));
    let router = Router::new()
        .route("/:pot/:amount", get(handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    Path((pot_size, amount)): Path<(u32, usize)>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let mut rng = state.lock().await;
    let mut lotto: Lotto<'_, SmallRng> = Lotto::new(pot_size, &mut rng);
    let results = lotto.take(amount);
    Json(results)
}
