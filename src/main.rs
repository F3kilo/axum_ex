mod error;
mod mysq;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use mysq::{BoardData, Boards, TaskData};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    tracing_subscriber::fmt().init();

    let db = Boards::new().await;

    let app = Router::new()
        .route("/board", get(read_boards).post(create_board))
        .route(
            "/board/:id",
            get(read_board).put(update_board).delete(delete_board),
        )
        .route("/board/:id/task", get(read_tasks).post(create_task))
        .route(
            "/board/:board_id/task/:task_name",
            get(read_task).put(update_task).delete(delete_task),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_board(State(db): State<Boards>, Json(data): Json<BoardData>) -> impl IntoResponse {
    db.create_board(&data).await.map(Json)
}

async fn read_boards(State(db): State<Boards>) -> impl IntoResponse {
    db.read_boards().await.map(Json)
}

async fn read_board(Path(id): Path<u64>, State(db): State<Boards>) -> impl IntoResponse {
    db.read_board(id).await.map(Json)
}

async fn update_board(
    Path(id): Path<u64>,
    State(db): State<Boards>,
    Json(data): Json<BoardData>,
) -> impl IntoResponse {
    db.update_board(id, &data).await.map(Json)
}

async fn delete_board(Path(id): Path<u64>, State(db): State<Boards>) -> impl IntoResponse {
    db.delete_board(id).await.map(Json)
}

async fn create_task(
    Path(id): Path<u64>,
    State(db): State<Boards>,
    Json(data): Json<TaskData>,
) -> impl IntoResponse {
    db.create_task(id, &data).await.map(Json)
}

async fn read_tasks(Path(id): Path<u64>, State(db): State<Boards>) -> impl IntoResponse {
    db.read_tasks(id).await.map(Json)
}

async fn read_task(
    Path((board_id, task_name)): Path<(u64, String)>,
    State(db): State<Boards>,
) -> impl IntoResponse {
    db.read_task(board_id, &task_name).await.map(Json)
}

async fn update_task(
    Path((board_id, task_name)): Path<(u64, String)>,
    State(db): State<Boards>,
    Json(data): Json<TaskData>,
) -> impl IntoResponse {
    db.update_task(board_id, &task_name, &data).await.map(Json)
}

async fn delete_task(
    Path((board_id, task_name)): Path<(u64, String)>,
    State(db): State<Boards>,
) -> impl IntoResponse {
    db.delete_task(board_id, &task_name).await.map(Json)
}
