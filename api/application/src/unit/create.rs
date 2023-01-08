use crate::token::JWT;
use domain::models::unit::Unit;
use infrastructure::ServerState;
use infrastructure::{review::read::db_does_unit_exist, unit::create::db_insert_unit};
use rocket::http::Status;

use crate::auth::has_admin_permissions;

pub fn create_unit(
    unit: Unit,
    state: &ServerState,
    token: JWT,
) -> Result<Unit, (Status, Option<String>)> {
    has_admin_permissions(&token)?;

    match db_does_unit_exist(&unit.unit_code, &state) {
        Ok(exists) => {
            if exists {
                return Err((Status::Conflict, Some(String::from("Unit already exist."))));
            }
        }
        Err(err) => return Err(err),
    };

    db_insert_unit(unit, &state)
}
