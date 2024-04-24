mod app;
mod pages;
mod components;
mod data;

use app::App;
fn main() {
    yew::Renderer::<App>::new().render();
}