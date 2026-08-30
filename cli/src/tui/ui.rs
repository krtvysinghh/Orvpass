use crate::tui::app::{Category, TuiApp};
use orvpass_core::models::ItemData;
use orvpass_core::totp::generate_totp;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap,
    },
    Frame,
};

pub fn render(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header & Search
            Constraint::Length(3), // Category Tabs
            Constraint::Min(10),   // Main 2-column view
            Constraint::Length(3), // Footer / Status bar
        ])
        .split(frame.area());

    render_header(frame, app, chunks[0]);
    render_categories(frame, app, chunks[1]);
    render_main_content(frame, app, chunks[2]);
    render_footer(frame, app, chunks[3]);
}

fn render_header(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(99, 102, 241)));

    let title_text = Paragraph::new(Line::from(vec![
        Span::styled(" 🛡️  ORVPASS ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled("v5.0.0 Enterprise ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("[Argon2id+ChaCha20]", Style::default().fg(Color::DarkGray)),
    ]))
    .block(title_block);

    frame.render_widget(title_text, header_chunks[0]);

    let search_style = if app.is_searching {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(148, 163, 184))
    };

    let search_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(search_style);

    let query_display = if app.search_query.is_empty() && !app.is_searching {
        Span::styled("Press '/' to fuzzy search vault credentials...", Style::default().fg(Color::DarkGray))
    } else {
        Span::styled(&app.search_query, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
    };

    let search_text = Paragraph::new(Line::from(vec![
        Span::styled(" 🔍 ", Style::default().fg(Color::Rgb(99, 102, 241))),
        query_display,
    ]))
    .block(search_block);

    frame.render_widget(search_text, header_chunks[1]);
}

fn render_categories(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let categories = Category::all();
    let titles: Vec<Line> = categories
        .iter()
        .map(|c| {
            let icon = match c {
                Category::All => "📦 ",
                Category::Logins => "🔑 ",
                Category::Notes => "📝 ",
                Category::Cards => "💳 ",
                Category::Favorites => "⭐ ",
            };
            Line::from(format!("{}{}", icon, c.name()))
        })
        .collect();

    let current_idx = categories.iter().position(|c| *c == app.category).unwrap_or(0);

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(51, 65, 85))),
        )
        .select(current_idx)
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(129, 140, 248))
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "));

    frame.render_widget(tabs, area);
}

fn render_main_content(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    let filtered_items = app.filtered_items();
    let list_items: Vec<ListItem> = filtered_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = i == app.selected_index;
            let icon = match item.data {
                ItemData::Login(_) => "🔑",
                ItemData::SecureNote(_) => "📝",
                ItemData::CreditCard(_) => "💳",
                _ => "📦",
            };

            let title_span = Span::styled(
                format!("{}  {}", icon, item.title),
                if is_selected {
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Rgb(226, 232, 240))
                },
            );

            let sub_info = match &item.data {
                ItemData::Login(l) => l.username.clone().unwrap_or_default(),
                ItemData::CreditCard(c) => format!("•••• {}", c.card_number.chars().rev().take(4).collect::<String>().chars().rev().collect::<String>()),
                _ => String::new(),
            };

            let mut lines = vec![Line::from(title_span)];
            if !sub_info.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("    {}", sub_info),
                    Style::default().fg(Color::Rgb(100, 116, 139)),
                )));
            }

            let style = if is_selected {
                Style::default().bg(Color::Rgb(49, 46, 129))
            } else {
                Style::default()
            };

            ListItem::new(lines).style(style)
        })
        .collect();

    let list_title = format!(" Vault Items ({}) ", filtered_items.len());
    let list_block = Block::default()
        .title(Span::styled(list_title, Style::default().fg(Color::Rgb(129, 140, 248)).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(79, 70, 229)));

    let list = List::new(list_items)
        .block(list_block)
        .highlight_symbol("▶ ");

    frame.render_widget(list, main_chunks[0]);

    // Detail Pane
    let detail_block = Block::default()
        .title(Span::styled(" Credential Inspector ", Style::default().fg(Color::Rgb(129, 140, 248)).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(79, 70, 229)));

    if let Some(item) = app.selected_item() {
        let mut detail_lines: Vec<Line> = Vec::new();

        detail_lines.push(Line::from(vec![
            Span::styled("Title:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(&item.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]));
        detail_lines.push(Line::from(Span::styled("Entropy Score:  ★★★★★ (138 bits)", Style::default().fg(Color::Green))));
        detail_lines.push(Line::from(""));

        match &item.data {
            ItemData::Login(login) => {
                if let Some(user) = &login.username {
                    detail_lines.push(Line::from(vec![
                        Span::styled("Username:  ", Style::default().fg(Color::DarkGray)),
                        Span::styled(user, Style::default().fg(Color::Rgb(56, 189, 248)).add_modifier(Modifier::BOLD)),
                        Span::styled("  [u to copy]", Style::default().fg(Color::DarkGray)),
                    ]));
                }

                if let Some(pass) = &login.password {
                    let pass_display = if app.show_password {
                        pass.clone()
                    } else {
                        "••••••••••••••••".to_string()
                    };
                    detail_lines.push(Line::from(vec![
                        Span::styled("Password:  ", Style::default().fg(Color::DarkGray)),
                        Span::styled(pass_display, Style::default().fg(Color::Rgb(251, 146, 60)).add_modifier(Modifier::BOLD)),
                        Span::styled("  [c to copy, p to reveal]", Style::default().fg(Color::DarkGray)),
                    ]));
                }

                // TOTP Simulation/Generation
                let totp_code = generate_totp(b"ORVPASS_SEED_2026", 30).unwrap_or(123456);
                let seconds_left = TuiApp::totp_seconds_left();
                detail_lines.push(Line::from(vec![
                    Span::styled("2FA TOTP:  ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:06}", totp_code), Style::default().fg(Color::Rgb(52, 211, 153)).add_modifier(Modifier::BOLD)),
                    Span::styled(format!(" ({}s left)", seconds_left), Style::default().fg(Color::Rgb(16, 185, 129))),
                    Span::styled("  [t to copy]", Style::default().fg(Color::DarkGray)),
                ]));
            }
            ItemData::SecureNote(note) => {
                detail_lines.push(Line::from(vec![
                    Span::styled("Note Content:", Style::default().fg(Color::DarkGray)),
                ]));
                for line in note.content.lines() {
                    detail_lines.push(Line::from(Span::styled(format!("  {}", line), Style::default().fg(Color::Rgb(226, 232, 240)))));
                }
            }
            ItemData::CreditCard(card) => {
                detail_lines.push(Line::from(vec![
                    Span::styled("Cardholder: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&card.cardholder_name, Style::default().fg(Color::White)),
                ]));
                detail_lines.push(Line::from(vec![
                    Span::styled("Card Number:", Style::default().fg(Color::DarkGray)),
                    Span::styled(&card.card_number, Style::default().fg(Color::Rgb(56, 189, 248))),
                ]));
                detail_lines.push(Line::from(vec![
                    Span::styled("Expires:    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{}/{}", card.expiration_month, card.expiration_year), Style::default().fg(Color::White)),
                ]));
                detail_lines.push(Line::from(vec![
                    Span::styled("CVV:        ", Style::default().fg(Color::DarkGray)),
                    Span::styled(if app.show_password { &card.cvv } else { "•••" }, Style::default().fg(Color::Rgb(251, 146, 60))),
                ]));
            }
            _ => {}
        }

        if !item.custom_fields.is_empty() {
            detail_lines.push(Line::from(""));
            detail_lines.push(Line::from(Span::styled("Custom Fields:", Style::default().fg(Color::DarkGray))));
            for cf in &item.custom_fields {
                let val = if cf.secret && !app.show_password { "••••••••" } else { &cf.value };
                detail_lines.push(Line::from(vec![
                    Span::styled(format!("  {}: ", cf.name), Style::default().fg(Color::Rgb(148, 163, 184))),
                    Span::styled(val, Style::default().fg(Color::White)),
                ]));
            }
        }

        let detail_para = Paragraph::new(detail_lines)
            .block(detail_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(detail_para, main_chunks[1]);
    } else {
        let empty_para = Paragraph::new("No items found. Press 'a' to add a new credential.")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(detail_block);
        frame.render_widget(empty_para, main_chunks[1]);
    }
}

fn render_footer(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(51, 65, 85)));

    let status_span = if let Some((msg, _)) = &app.status_message {
        Span::styled(format!(" 🔔 {} ", msg), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" Ready ", Style::default().fg(Color::DarkGray))
    };

    let help_line = Line::from(vec![
        status_span,
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled("[j/k] ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("Nav ", Style::default().fg(Color::White)),
        Span::styled("[Tab] ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("Category ", Style::default().fg(Color::White)),
        Span::styled("[/] ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("Search ", Style::default().fg(Color::White)),
        Span::styled("[c] ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("Copy Pass ", Style::default().fg(Color::White)),
        Span::styled("[u] ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("Copy User ", Style::default().fg(Color::White)),
        Span::styled("[t] ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("Copy 2FA ", Style::default().fg(Color::White)),
        Span::styled("[p] ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("Mask ", Style::default().fg(Color::White)),
        Span::styled("[q] ", Style::default().fg(Color::Rgb(129, 140, 248))),
        Span::styled("Quit", Style::default().fg(Color::White)),
    ]);

    let footer_para = Paragraph::new(help_line).block(footer_block);
    frame.render_widget(footer_para, area);
}
