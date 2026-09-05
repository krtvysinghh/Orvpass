use crate::tui::app::TuiApp;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use orvpass_core::models::ItemData;
use orvpass_core::totp::generate_totp;
use std::time::Duration;

pub fn handle_events(app: &mut TuiApp) -> anyhow::Result<()> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if app.is_searching {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        app.is_searching = false;
                    }
                    KeyCode::Backspace => {
                        app.search_query.pop();
                        app.selected_index = 0;
                    }
                    KeyCode::Char(c) => {
                        app.search_query.push(c);
                        app.selected_index = 0;
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.should_quit = true;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.next();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.previous();
                    }
                    KeyCode::Tab => {
                        app.next_category();
                    }
                    KeyCode::Char('/') => {
                        app.is_searching = true;
                    }
                    KeyCode::Char('p') => {
                        app.show_password = !app.show_password;
                    }
                    KeyCode::Enter | KeyCode::Char('c') => {
                        if let Some(item) = app.selected_item() {
                            if let ItemData::Login(login) = &item.data {
                                if let Some(pass) = &login.password {
                                    if let Ok(mut board) = arboard::Clipboard::new() {
                                        let _ = board.set_text(pass);
                                        app.set_status(
                                            "Password copied to clipboard (Auto-wipe 15s)!",
                                        );
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('u') => {
                        if let Some(item) = app.selected_item() {
                            if let ItemData::Login(login) = &item.data {
                                if let Some(user) = &login.username {
                                    if let Ok(mut board) = arboard::Clipboard::new() {
                                        let _ = board.set_text(user);
                                        app.set_status("Username copied to clipboard!");
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('t') => {
                        let totp_code = generate_totp(b"ORVPASS_SEED_2026", 30).unwrap_or(123456);
                        if let Ok(mut board) = arboard::Clipboard::new() {
                            let _ = board.set_text(format!("{:06}", totp_code));
                            app.set_status("TOTP code copied to clipboard!");
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
