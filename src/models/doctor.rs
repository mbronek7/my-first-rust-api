#[derive(Queryable)]
pub struct Doctor {
  pub id: i32,
  pub name: String,
  pub specialization: String,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "doctors"]
pub struct NewDoctor {
  pub name: String,
  pub specialization: String,
}

