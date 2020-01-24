#[derive(Queryable)]
pub struct Patient {
  pub id: i32,
  pub first_name: String,
  pub second_name: String,
  pub phone_number: String,
  pub email: String,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "patients"]
pub struct NewPatient {
  pub first_name: String,
  pub second_name: String,
  pub phone_number: String,
  pub email: String,
}