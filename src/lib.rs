pub mod app;
pub mod event;
pub mod handler;
pub mod tui;

mod pages;
mod study;

pub type AppResult<T> = std::result::Result<T, Box<dyn core::error::Error>>;
