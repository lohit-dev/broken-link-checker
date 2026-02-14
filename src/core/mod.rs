use std::sync::Arc;

use crate::context::AppContext;
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

impl Checker {
    pub fn new(settings: Settings, ctx: Arc<AppContext>) -> Self {
        Self {
            settings,
            crawler: Crawler::new(Arc::clone(&ctx)),
            validator: Validator::default(),
        }
    }
}
