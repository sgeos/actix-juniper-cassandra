#![deny(warnings)]

use crate::graphql::types::*;

use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};
use std::{env};

#[derive(Default, Clone)]
pub struct Context {
    ///this could be a context connection
    version: String,
    ip_address: String
}
impl Context {
    pub fn new(ip_address: String) -> Context {
        let version = env!("CARGO_PKG_VERSION").to_string();
        Context { version: version, ip_address: ip_address }
    }
}

// To make our Context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn ping(_context: &Context) -> FieldResult<Pong> {
        Ok(Pong::new())
    }

    fn info(context: &Context) -> FieldResult<Info> {
        Ok(Info::new(context.version.to_string(), context.ip_address.to_string()))
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

