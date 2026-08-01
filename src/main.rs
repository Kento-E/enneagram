mod app;
mod models;
mod questions;
mod storage;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
