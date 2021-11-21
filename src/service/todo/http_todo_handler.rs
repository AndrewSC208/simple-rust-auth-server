use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::models::{Pool};
use crate::errors::{ServiceError};

#[derive(Debug, Deserialize)]
pub struct NewTodo {
  pub name: String,
  pub is_complete: bool,
  pub owner_id: String,
}

pub async fn post_todo(
  todo_data: web::Json<NewTodo>,
  pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
  use crate::schema::todos::dsl::{todos};

  let conn: &PgConnection = &pool().get().unwrap();
  todos
    .filter()
}

