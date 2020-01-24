extern crate dotenv;

use diesel::prelude::*;
use juniper::RootNode;

use crate::db::PgPool;
use crate::schema::patients;
use crate::schema::doctors;
use crate::schema::visits;

include!("models/visit.rs");
include!("models/doctor.rs");
include!("models/patient.rs");


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

  fn visits(context: &Context) -> Vec<Visit> {
    use crate::schema::visits::dsl::*;
    let connection = context.db.get().unwrap();;
    visits
      .limit(10)
      .load::<Visit>(&connection)
      .expect("Error loading visits")
  }

  fn doctors(context: &Context) -> Vec<Doctor> {
    use crate::schema::doctors::dsl::*;
    let connection = context.db.get().unwrap();;
    doctors
      .limit(10)
      .load::<Doctor>(&connection)
      .expect("Error loading doctors")
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

  fn create_doctor(context: &Context, data: NewDoctor) -> Doctor {
    let connection = context.db.get().unwrap();;
    diesel::insert_into(doctors::table)
      .values(&data)
      .get_result(&connection)
      .expect("Error saving new doctor")
  }

  fn create_visit(context: &Context, data: NewVisit) -> Visit {
    let connection = context.db.get().unwrap();;
    diesel::insert_into(visits::table)
      .values(&data)
      .get_result(&connection)
      .expect("Error saving new visit")
  }
}

#[juniper::object(Context = Context, description = "A patients list")]
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

  pub fn visits(&self, context: &Context) -> Vec<Visit> {
    use crate::schema::visits::dsl::*;
    let connection = context.db.get().unwrap();
    visits
      .filter(patient_id.eq(self.id))
      .load::<Visit>(&connection)
      .expect("Error loading visit data")
  }
}

#[juniper::object(Context = Context, description = "A doctors list")]
impl Doctor {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn specialization(&self) -> &str {
    self.specialization.as_str()
  }

  pub fn visits(&self, context: &Context) -> Vec<Visit> {
    use crate::schema::visits::dsl::*;
    let connection = context.db.get().unwrap();
    visits
      .filter(doctor_id.eq(self.id))
      .load::<Visit>(&connection)
      .expect("Error loading visit data")
  }
}

#[juniper::object(Context = Context, description = "A visits list")]
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
      .filter(id.eq(self.patient_id))
      .load::<Patient>(&connection)
      .expect("Error loading patient data")
  }

  pub fn doctors(&self, context: &Context) -> Vec<Doctor> {
    use crate::schema::doctors::dsl::*;
    let connection = context.db.get().unwrap();
    doctors
      .filter(id.eq(self.doctor_id))
      .load::<Doctor>(&connection)
      .expect("Error loading doctor data")
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
