#[derive(Queryable)]
pub struct Visit {
  pub id: i32,
  pub visit_name: String,
  pub visit_date: String,
  pub doctor_id: i32,
  pub patient_id: i32,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "visits"]
pub struct NewVisit {
  pub visit_name: String,
  pub visit_date: String,
  pub doctor_id: i32,
  pub patient_id: i32,
}