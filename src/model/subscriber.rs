use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use crate::model::notfication::Notification;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub sruct Subscriber {
    pub url: String,
    pub name: String,
}