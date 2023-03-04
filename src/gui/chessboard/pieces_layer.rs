use iced::{
    theme,
    widget::{self, container, svg, Svg},
    Element, Length, Renderer, Size,
};
use iced_lazy::Component;
use iced_native::widget::Container;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct PiecesLayer {
    size: u16,
    bishop: Svg<Renderer>,
}

impl PiecesLayer {
    pub fn new(size: u16) -> Self {
        let handle = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_bdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let bishop = svg(handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Svg::Default);

        Self { size, bishop }
    }
}

impl<Message, Renderer> Component<Message, Renderer> for PiecesLayer
where
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme:
        widget::button::StyleSheet + widget::text_input::StyleSheet + widget::text::StyleSheet,
{
    type State = ();

    type Event = ();

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        None
    }

    fn view(&self, state: &Self::State) -> iced_native::Element<'_, Self::Event, Renderer> {
        let content = self.bishop;
        container(content)
            .width(Length::Units(self.size))
            .height(Length::Units(self.size))
            .center_x()
            .center_y()
    }
}

impl<'a, Message, Renderer> From<PiecesLayer> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + iced_native::text::Renderer,
    Renderer::Theme:
        widget::button::StyleSheet + widget::text_input::StyleSheet + widget::text::StyleSheet,
{
    fn from(pieces_layer: PiecesLayer) -> Self {
        iced_lazy::component(pieces_layer)
    }
}
