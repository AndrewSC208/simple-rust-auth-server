use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

// use crate::email_service::send_invitation;
use crate::dao::models::{Pool, Invitation};
use crate::service::errors::ServiceError;

/// shape of the json payload that is expected
#[derive(Deserialize)]
pub struct InvitationData {
  pub email: String,
}

/// handle a post to the invitation http route and call diesel
/// when diesel responses check if it's an error. If it's not an error
/// then return Ok, else check the type of error and map it to a
/// `ServiceError` value.
///
/// (1) http view/handler method
pub async fn post_invitation(
  invitation_data: web::Json<InvitationData>,
  pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> { 
  // run diesel blocking code
  let res = web::block(move || 
    create_invitation(invitation_data.into_inner().email, pool)).await;

  match res {
    Ok(_) => Ok(HttpResponse::Ok().finish()),
    Err(err) => match err {
      BlockingError::Error(service_error) => Err(service_error),
      BlockingError::Canceled => Err(ServiceError::InternalServerError),
    },
  }
}

/// (2) controller method
/// business logic of that action
fn create_invitation(
  email: String,
  pool: web::Data<Pool>,
) -> Result<(), crate::service::errors::ServiceError> {
  let _invitation = dbg!(query(email, pool)?);

  // un-comment when `send_invitation` is implemented holding off for this now
  // because I will need to implement some type of email service and research is
  // needed for that.
  // send_invitation(&invitation)

  Ok(())
}

/// (3) dao method
/// interations with the repository in this case a postgres database
fn query(email: String, pool: web::Data<Pool>) -> Result<Invitation, crate::service::errors::ServiceError> {
  // scoped import to of the `invitations` type to this method.
  use crate::dao::schema::invitations::dsl::invitations;

  let new_invitation: Invitation = email.into();
  let conn: &PgConnection = &pool.get().unwrap();

  let inserted_invitation = diesel::insert_into(invitations)
    .values(&new_invitation)
    .get_result(conn)?;

  Ok(inserted_invitation)
}
