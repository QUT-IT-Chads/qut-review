use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use domain::enums::role::Role;
use rocket::{http::Status, serde::json::Json};
use shared::{response_models::ResponseMessage, token::JWT};
use uuid::Uuid;

pub fn has_user_permissions(token: &JWT, id: &Uuid) -> Result<(), (Status, Json<ResponseMessage>)> {
    if token.claims.sub != *id && token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from(
                "You do not have access to perform this action.",
            )),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    Ok(())
}

pub fn has_admin_permissions(token: &JWT) -> Result<(), (Status, Json<ResponseMessage>)> {
    if token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from(
                "You do not have access to perform this action.",
            )),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    Ok(())
}

pub fn unit_exists(
    unit_code: &String,
    pooled: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<(), (Status, Json<ResponseMessage>)> {
    use domain::schema::units;

    match pooled.transaction(|c| {
        units::table
            .select(units::all_columns)
            .filter(units::unit_code.eq(&unit_code))
            .count()
            .load::<i64>(c)
    }) {
        Ok(unit_count) => {
            if unit_count[0] == 0 {
                let response = ResponseMessage {
                    message: Some(String::from("Unit does not exist.")),
                };

                return Err((Status::NotFound, Json(response)));
            }

            Ok(())
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: Some(String::from("Unit does not exist.")),
                };

                return Err((Status::NotFound, Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
