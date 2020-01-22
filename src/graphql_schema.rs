extern crate dotenv;

use diesel::prelude::*;
use juniper::RootNode;

use crate::db::PgPool;
use crate::schema::patients;

#[derive(Clone)]
pub struct Context {
  pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
  fn patients(context: &Context) -> Vec<Patient> {
    use crate::schema::patients::dsl::*;
    let connection = context.db.get().unwrap();;
    patients
      .limit(100)
      .load::<Patient>(&connection)
      .expect("Error loading patients")
  }

  fn visit(context: &Context) -> Vec<Visit> {
    use crate::schema::visits::dsl::*;
    let connection = context.db.get().unwrap();;
    visits
      .limit(10)
      .load::<Visit>(&connection)
      .expect("Error loading visits")
  }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
  fn create_patient(context: &Context, data: NewPatient) -> Patient {
    let connection = context.db.get().unwrap();;
    diesel::insert_into(patients::table)
      .values(&data)
      .get_result(&connection)
      .expect("Error saving new patient")
  }
}

#[derive(Queryable)]
pub struct Patient {
  pub id: i32,
  pub first_name: String,
  pub second_name: String,
  pub phone_number: String,
  pub email: String,
  pub visit_id: i32,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "patients"]
pub struct NewPatient {
  pub first_name: String,
  pub second_name: String,
  pub phone_number: String,
  pub email: String,
  pub visit_id: i32,
}

#[juniper::object(description = "A patient of a visit")]
impl Patient {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn first_name(&self) -> &str {
    self.first_name.as_str()
  }

  pub fn second_name(&self) -> &str {
    self.second_name.as_str()
  }

  pub fn phone_number(&self) -> &str {
    self.phone_number.as_str()
  }

  pub fn email(&self) -> &str {
    self.email.as_str()
  }

  pub fn visit_id(&self) -> i32 {
    self.visit_id
  }
}

#[derive(Queryable)]
pub struct Visit {
  pub id: i32,
  pub visit_name: String,
  pub visit_date: String,
  pub doctor_id: i32,
}

#[juniper::object(Context = Context, description = "A team of members")]
impl Visit{
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn visit_name(&self) -> &str {
    self.visit_name.as_str()
  }

  pub fn visit_date(&self) -> &str {
    self.visit_date.as_str()
  }

  pub fn patients(&self, context: &Context) -> Vec<Patient> {
    use crate::schema::patients::dsl::*;
    let connection = context.db.get().unwrap();
    patients
      .filter(visit_id.eq(self.id))
      .limit(100)
      .load::<Patient>(&connection)
      .expect("Error loading patients")
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
