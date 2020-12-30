pub mod editor_view;
pub mod screen; 

pub trait Widget {
    fn render(&self, screen: &screen::Screen);
}