pub mod app;
pub mod event;
pub mod handler;
mod pages;
mod study;
pub mod tui;

pub type AppResult<T> = std::result::Result<T, Box<dyn core::error::Error>>;
