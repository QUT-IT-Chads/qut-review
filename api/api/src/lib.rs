use application::response_models::ResponseMessage;
use rocket::{http::Status, serde::{json::Json, Serialize}};

pub mod review_handler;
pub mod unit_handler;
pub mod user_handler;

fn convert_err((status, message): (Status, Option<String>)) -> (Status, Json<ResponseMessage>) {
    (status, Json(ResponseMessage { message }))
}

fn convert_result<T: Serialize>(
    res: Result<T, (Status, Option<String>)>,
) -> Result<Json<T>, (Status, Json<ResponseMessage>)> {
    res.map(Json).map_err(convert_err)
}
