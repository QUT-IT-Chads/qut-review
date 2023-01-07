use crate::token::JWT;
use infrastructure::unit::delete::db_delete_unit;
use infrastructure::ServerState;
use rocket::{http::Status, State};

use crate::auth::has_admin_permissions;

pub fn delete_unit(
    unit_code: &str,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Option<String>, (Status, Option<String>)> {
    has_admin_permissions(&token)?;

    db_delete_unit(unit_code, state)
}
