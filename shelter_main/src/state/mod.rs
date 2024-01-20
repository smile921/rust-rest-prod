use std::sync::Arc;

use crate::settings::Settings;
use arc_swap::ArcSwap;
use sea_orm::DatabaseConnection;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub db_conn: ArcSwap<DatabaseConnection>,
}

impl ApplicationState {
    pub fn new(settings: &Settings, db_conn: DatabaseConnection) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            db_conn: ArcSwap::new(Arc::new(db_conn)),
        })
    }
}
