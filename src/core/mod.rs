use crate::{
    core::{crawler::Crawler, validator::Validator},
    settings::Settings,
};

pub mod command;
pub mod crawler;
pub mod validator;

pub struct Checker {
    pub settings: Settings,
    pub crawler: Crawler,
    pub validator: Validator,
}
