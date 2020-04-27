use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}
