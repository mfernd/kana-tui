pub mod app;
pub mod config;
pub mod event;
pub mod handler;
pub mod tui;

mod models;
mod pages;
mod widgets;

pub type AppResult<T> = std::result::Result<T, Box<dyn core::error::Error>>;
