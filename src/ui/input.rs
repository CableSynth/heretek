use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{layout::Rect, style::Style, Frame};

use super::{GREEN, YELLOW};

use crate::{App, InputMode};

pub fn draw_input(title_area: Rect, app: &App, f: &mut Frame, input: Rect) {
    // Input
    let width = title_area.width.max(3) - 3;
    // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let stream_lock = app.stream_output_prompt.lock().unwrap();
    let prompt_len = stream_lock.len();

    let txt_input = Paragraph::new(format!("{}{}", stream_lock, app.input.value()))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(GREEN),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input".fg(YELLOW)));
    f.render_widget(txt_input, input);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor_position((
                // Put cursor past the end of the input text
                input.x
                    + ((app.input.visual_cursor()).max(scroll) - scroll) as u16
                    + 1
                    + prompt_len as u16,
                // Move one line down, from the border to the input line
                input.y + 1,
            ))
        }
    }
}