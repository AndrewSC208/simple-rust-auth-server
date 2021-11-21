use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::service::errors::ServiceError;
use crate::dao::models::{Invitation, Pool, SlimUser, User};
use crate::utils::secrets::hash_password;

#[derive(Debug, Deserialize)]
pub struct UserData {
  pub email: String,
  pub password: String,
}

pub async fn register_user(
  invitation_id: web::Path<String>,
  user_data: web::Json<UserData>,
  pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
  let res = web::block(move || query(invitation_id.into_inner(), user_data.into_inner(), pool)).await;

  match res {
    Ok(user) => Ok(HttpResponse::Ok().json(&user)),
    Err(err) => match err {
      BlockingError::Error(service_error) => Err(service_error),
      BlockingError::Canceled => Err(ServiceError::InternalServerError),
    },
  }
}

fn query(
  invitation_id: String,
  user_data: UserData,
  pool: web::Data<Pool>,
) -> Result<SlimUser, crate::service::errors::ServiceError> {
  use crate::dao::schema::invitations::dsl::{email, id, invitations};
  use crate::dao::schema::users::dsl::users;
  let invitation_id = uuid::Uuid::parse_str(&invitation_id)?;

  let conn: &PgConnection = &pool.get().unwrap();
  invitations
    .filter(id.eq(invitation_id))
    .filter(email.eq(&user_data.email))
    .load::<Invitation>(conn)
    .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
    .and_then(|mut result| {
      if let Some(invitation) = result.pop() {
        if invitation.expires_at > chrono::Local::now().naive_local() {
          let password: String = hash_password(&user_data.password)?;
          let user = User::from_details(invitation.email, password);
          let inserted_user: User = diesel::insert_into(users).values(&user).get_result(conn)?;
          dbg!(&inserted_user);

          return Ok(inserted_user.into());
        }
      }
      Err(ServiceError::BadRequest("Invalid Invitation".into()))
    })
}

