use crate::token::JWT;
use domain::models::unit::Unit;
use infrastructure::unit::update::db_update_unit;
use infrastructure::ServerState;
use rocket::http::Status;
use rocket::State;

use crate::auth::has_admin_permissions;

pub fn update_unit(
    unit_code: &str,
    unit: Unit,
    state: &State<ServerState>,
    token: JWT,
) -> Result<Unit, (Status, Option<String>)> {
    has_admin_permissions(&token)?;

    db_update_unit(unit_code, unit, state)
}
