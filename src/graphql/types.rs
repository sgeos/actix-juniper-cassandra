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

#[derive(GraphQLObject)]
///Info.
pub struct Info {
    date: NaiveDate,
    datetime: DateTime<Utc>,
    remote_ip: String,
    time: NaiveTime,
    version: String,
}

impl Info {
    pub fn new(version: String, remote_ip: String) -> Info {
        let datetime = Utc::now();
        Info {
            date: datetime.naive_utc().date(),
            datetime: datetime,
            remote_ip: remote_ip,
            time: datetime.time(),
            version: version,
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

