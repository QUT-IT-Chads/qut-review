use shared::response_models::Response;

pub fn serialize_response(response: Response) -> String {
    serde_json::to_string(&response).expect("Return 500 internal server error.")
}
