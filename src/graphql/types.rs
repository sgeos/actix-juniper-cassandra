#![deny(warnings)]

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use juniper::{GraphQLObject};

#[derive(GraphQLObject)]
///Ping-pong.
pub struct Pong {
    date: NaiveDate,
    datetime: DateTime<Utc>,
    pong: String,
    success: bool,
    time: NaiveTime,
}

impl Pong {
    pub fn new() -> Pong {
        let datetime = Utc::now();
        Pong {
            date: datetime.naive_utc().date(),
            datetime: datetime,
            pong: "Pong".to_owned(),
            success: true,
            time: datetime.time(),
        }
    }
}

#[derive(Clone, GraphQLObject)]
///a user
pub struct User {
    ///the id
    pub id: i32,
    ///the name
    pub name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> User {
        User {
            id: id,
            name: name,
        }
    }
}

