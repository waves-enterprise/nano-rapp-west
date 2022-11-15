use nanos_sdk::buttons::{ButtonEvent, ButtonsState};
use nanos_ui::bagls::*;
use nanos_ui::layout::{Draw, Layout, Location, StringPlace};
use nanos_ui::screen_util;
use nanos_ui::ui;

pub struct TxScroller<'a> {
    titles: &'a [&'a str],
    messages: &'a [&'a str],
}

impl<'a> TxScroller<'a> {
    pub fn new(titles: &'a [&'a str], messages: &'a [&'a str]) -> Self {
        TxScroller { titles, messages }
    }

    pub fn show(&self) -> bool {
        let mut buttons = ButtonsState::new();
        let mut cur_page = 0;

        let titles_len = self.titles.len();
        let messages_len = self.messages.len();
        if titles_len != messages_len {
            return false;
        }

        let page_count = titles_len;
        if page_count == 0 {
            return false;
        }

        // A closure to draw common elements of the screen
        // cur_page passed as parameter to prevent borrowing
        let draw = |page: usize| {
            ui::clear_screen();

            if page == 0 {
                [self.titles[page], self.messages[page]].place(
                    Location::Middle,
                    Layout::Centered,
                    true,
                )
            } else if page == page_count {
                // Confirmation of transaction signing
                CHECKMARK_ICON.display();
                ["Accept", "and send"].place(Location::Middle, Layout::Centered, true)
            } else if page == page_count + 1 {
                // Cancel the signing of a transaction
                CROSS_ICON.display();
                "Reject".place(Location::Middle, Layout::Centered, true)
            } else {
                self.titles[page].place(Location::Top, Layout::Centered, true);

                self.messages[page].place(Location::Middle, Layout::Centered, false);
            };

            if page > 0 {
                LEFT_ARROW.display();
            }

            if page + 1 < page_count + 2 {
                RIGHT_ARROW.display();
            }

            screen_util::screen_update();
        };

        draw(cur_page);

        let mut response = false;

        loop {
            match ui::get_event(&mut buttons) {
                Some(ButtonEvent::LeftButtonPress) => {
                    LEFT_S_ARROW.instant_display();
                }
                Some(ButtonEvent::RightButtonPress) => {
                    RIGHT_S_ARROW.instant_display();
                }
                Some(ButtonEvent::LeftButtonRelease) => {
                    if cur_page > 0 {
                        cur_page -= 1;
                    }
                    // We need to draw anyway to clear button press arrow
                    draw(cur_page);
                }
                Some(ButtonEvent::RightButtonRelease) => {
                    if cur_page + 1 < page_count + 2 {
                        cur_page += 1;
                    }
                    // We need to draw anyway to clear button press arrow
                    draw(cur_page);
                }
                Some(ButtonEvent::BothButtonsRelease) => {
                    // If the user chose to confirm the signing of the transaction
                    if cur_page == page_count {
                        return true;
                    }
                    // If the user refused to sign the transaction
                    if cur_page == page_count + 1 {
                        return false;
                    }
                }
                Some(_) | None => (),
            }
        }
    }
}
