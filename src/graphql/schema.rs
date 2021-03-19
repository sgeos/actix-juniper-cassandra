#![deny(warnings)]

use crate::graphql::types::*;

use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};
use std::{env};

#[derive(Default, Clone)]
pub struct Context {
    ///this could be a context connection
    version: String,
}
impl Context {
    pub fn new() -> Context {
        let version = env!("CARGO_PKG_VERSION").to_string();
        Context { version: version }
    }
}

// To make our Context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn apiVersion(context: &Context) -> &String {
        &context.version
    }

    fn ping(_context: &Context) -> FieldResult<Pong> {
        Ok(Pong::new())
    }

    #[graphql(arguments(id(description = "id of the user")))]
    fn user(_context: &Context, id: i32) -> FieldResult<User> {
        let name = "Aaron".to_string();
        Ok(User::new(id, name))
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}

