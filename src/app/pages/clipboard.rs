//! The elements for the clipboard history page
use iced::widget::{
    Scrollable,
    image::{Handle, Viewer},
    scrollable::{Direction, Scrollbar},
};

use crate::{
    app::{ToApp, pages::prelude::*},
    clipboard::ClipBoardContentType,
};

/// The clipboard view
///
/// Takes:
/// - the clipboard content to render,
/// - the id of which element is focussed,
/// - and the [`Theme`]
///
/// Returns:
/// - the iced Element to render
pub fn clipboard_view(
    clipboard_content: Vec<ClipBoardContentType>,
    focussed_id: u32,
    theme: Theme,
) -> Element<'static, Message> {
    let theme_clone = theme.clone();
    let theme_clone_2 = theme.clone();
    let viewport_content: Element<'static, Message> =
        match clipboard_content.get(focussed_id as usize) {
            Some(content) => match content {
                ClipBoardContentType::Text(txt) => Text::new(txt.to_owned())
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .align_x(Alignment::Start)
                    .font(theme.font())
                    .size(16)
                    .into(),

                ClipBoardContentType::Image(data) => {
                    let bytes = data.to_owned_img().into_owned_bytes();
                    Viewer::new(
                        Handle::from_rgba(data.width as u32, data.height as u32, bytes.to_vec())
                            .clone(),
                    )
                    .width(500)
                    .height(500)
                    .into()
                }
            },
            None => Text::new("").into(),
        };
    container(Row::from_vec(vec![
        container(
            iced::widget::scrollable(
                Column::from_iter(clipboard_content.iter().enumerate().map(|(i, content)| {
                    content
                        .to_app()
                        .render(theme.clone(), i as u32, focussed_id)
                }))
                .width(WINDOW_WIDTH / 3.),
            )
            .id("results"),
        )
        .height(10000)
        .style(move |_| result_row_container_style(&theme_clone_2, false))
        .into(),
        container(Scrollable::with_direction(
            viewport_content,
            Direction::Both {
                vertical: Scrollbar::new().scroller_width(0.).width(0.),
                horizontal: Scrollbar::new().scroller_width(0.).width(0.),
            },
        ))
        .height(10000)
        .padding(10)
        .style(move |_| result_row_container_style(&theme_clone, false))
        .width((WINDOW_WIDTH / 3.) * 2.)
        .into(),
    ]))
    .height(280)
    .into()
}
