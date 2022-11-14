use nanos_sdk::buttons::{ButtonEvent, ButtonsState};
use nanos_ui::bagls::*;
use nanos_ui::layout::{Draw, Layout, Location, StringPlace};
use nanos_ui::ui;

pub struct TxScroller<'a> {
    titles: &'a [&'a str],
    messages: &'a [&'a str],
}

impl<'a> TxScroller<'a> {
    pub fn new(titles: &'a [&'a str], messages: &'a [&'a str]) -> Self {
        TxScroller { titles, messages }
    }

    pub fn event_loop(&self) {
        let mut buttons = ButtonsState::new();
        let mut cur_page = 0;

        let titles_len = self.titles.len();
        let messages_len = self.messages.len();
        if titles_len != messages_len {
            return;
        }

        let page_count = titles_len;
        if page_count == 0 {
            return;
        }

        // A closure to draw common elements of the screen
        // cur_page passed as parameter to prevent borrowing
        let draw = |page: usize| {
            ui::clear_screen();

            [self.titles[page], self.messages[page]].place(
                Location::Middle,
                Layout::Centered,
                false,
            );

            if page > 0 {
                LEFT_ARROW.display();
            }
            if page + 1 < page_count {
                RIGHT_ARROW.display();
            }
        };

        draw(cur_page);

        loop {
            match ui::get_event(&mut buttons) {
                Some(ButtonEvent::LeftButtonPress) => {
                    LEFT_S_ARROW.display();
                }
                Some(ButtonEvent::RightButtonPress) => {
                    RIGHT_S_ARROW.display();
                }
                Some(ButtonEvent::LeftButtonRelease) => {
                    if cur_page > 0 {
                        cur_page -= 1;
                    }
                    // We need to draw anyway to clear button press arrow
                    draw(cur_page);
                }
                Some(ButtonEvent::RightButtonRelease) => {
                    if cur_page + 1 < page_count {
                        cur_page += 1;
                    }
                    // We need to draw anyway to clear button press arrow
                    draw(cur_page);
                }
                Some(ButtonEvent::BothButtonsRelease) => break,
                Some(_) | None => (),
            }
        }
    }
}
