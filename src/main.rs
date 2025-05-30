use std::{
    io::{self, stdout},
    time::Duration,
    usize,
};

use chrono::DateTime;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::stream::{FuturesUnordered, StreamExt};
use hackernews::{
    StoryType,
    get_items::{ItemResponse, get_item},
    get_stories::{
        get_beststories, get_jobstories, get_newstories, get_showstories, get_topstories,
    },
};
use html2text::config;
use ratatui::{
    Frame, Terminal,
    layout::Layout,
    prelude::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};
use tokio::sync::watch;

fn setup_terminal() -> std::io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> std::io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    setup_terminal()?;
    let mut selected: usize = 0;
    let mut data: Vec<ItemResponse> = Vec::new();
    let mut current_topic = StoryType::Show;
    let mut list_top_cursor = 0;
    let _ = terminal.clear();

    let (tx_topic, mut rx_topic) = watch::channel::<StoryType>(StoryType::Show);
    let (tx, rx) = watch::channel::<Option<ItemResponse>>(None);

    tokio::spawn(async move {
        let mut last_topic: Option<StoryType> = None;

        loop {
            if last_topic == Some(*rx_topic.borrow()) {
                continue;
            }

            let topic = rx_topic.borrow().clone();
            last_topic = Some(topic);

            let list = match topic {
                StoryType::Top => get_topstories().await,
                StoryType::New => get_newstories().await,
                StoryType::Show => get_showstories().await,
                StoryType::Best => get_beststories().await,
                StoryType::Jobs => get_jobstories().await,
            }
            .unwrap();

            let mut futures = FuturesUnordered::new();

            for chunk in list.chunks(5) {
                for item_id in chunk {
                    let fut = get_item(*item_id);
                    futures.push(async move {
                        match fut.await {
                            Ok(item) => Some(item),
                            Err(_) => None,
                        }
                    });
                }
                if last_topic != Some(*rx_topic.borrow()) {
                    break;
                }
                loop {
                    tokio::select! {
                        changed = rx_topic.changed() => {
                            if changed.is_ok() {
                                tx.send(None).ok();
                                break;
                            }
                        }

                        maybe_item = futures.next() => {
                            match maybe_item {
                                Some(item) => {
                                    tx.send(item).ok();
                                }
                                None => {
                                    futures.clear();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                } else if key.code == KeyCode::Char('j') {
                    selected = selected.saturating_add(1).min(data.len().saturating_sub(1));
                } else if key.code == KeyCode::Char('k') {
                    selected = selected.saturating_sub(1);
                } else if key.code == KeyCode::Tab {
                    current_topic = match current_topic {
                        StoryType::Show => StoryType::Best,
                        StoryType::Best => StoryType::Jobs,
                        StoryType::Jobs => StoryType::Top,
                        StoryType::Top => StoryType::New,
                        StoryType::New => StoryType::Show,
                    };
                    tx_topic.send(current_topic).unwrap();
                } else if key.code == KeyCode::BackTab {
                    current_topic = match current_topic {
                        StoryType::Show => StoryType::New,
                        StoryType::New => StoryType::Top,
                        StoryType::Top => StoryType::Jobs,
                        StoryType::Jobs => StoryType::Best,
                        StoryType::Best => StoryType::Show,
                    };
                    tx_topic.send(current_topic).unwrap();
                }
            }
        }

        terminal.draw(|f| {
            self::draw(
                f,
                data.clone(),
                selected,
                current_topic,
                &mut list_top_cursor,
            );
        })?;

        if let Some(item) = rx.borrow().clone() {
            if data.last() != Some(&item) {
                data.push(item);
            }
        } else {
            data.clear();
            selected = 0;
        }
    }

    restore_terminal()?;
    Ok(())
}

fn draw(
    frame: &mut Frame,
    data: Vec<ItemResponse>,
    selected: usize,
    topic: StoryType,
    list_top_cursor: &mut usize,
) {
    let horizontal = Layout::horizontal([
        ratatui::layout::Constraint::Percentage(30),
        ratatui::layout::Constraint::Percentage(70),
    ]);
    let [left, right] = horizontal.areas(frame.area());
    let left_height: usize = left.height.into();
    let left_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(Line::from(vec![
            Span::raw(format!("<({}/{})", selected, data.len())),
            Span::styled("T", Style::default().fg(Color::Red)),
            Span::raw(format!(" - {}>", topic.to_string())),
        ]));
    let right_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Content");

    let list_items = data
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let mut list_item = ListItem::new(item.title.clone());
            if idx == selected.try_into().unwrap_or(0) {
                list_item = list_item.style(Style::default().bg(Color::Blue));
            }

            list_item
        })
        .collect::<Vec<_>>();

    let list_len: usize = list_items.len().try_into().unwrap_or(0);
    if selected < *list_top_cursor {
        *list_top_cursor = list_top_cursor.clone().saturating_sub(1);
    } else if selected >= *list_top_cursor + left_height.saturating_sub(2) && selected < list_len {
        *list_top_cursor = list_top_cursor.clone().saturating_add(1);
    }
    let top: usize = *list_top_cursor;
    let bottom: usize = (selected + left_height)
        .try_into()
        .unwrap_or(0)
        .min(list_items.len());
    let list = List::new(list_items[top..bottom].to_vec()).block(left_block);

    let article = Paragraph::new(data.get(selected as usize).map_or(
        "No article selected".to_string(),
        |item| {
            format!(
                "Title: {}\nAuthor: {}\nTime: {}\nURL: {}\n\n{}",
                item.title,
                item.by.as_deref().unwrap_or("Unknown"),
                DateTime::from_timestamp(item.time as i64, 0)
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                item.url.as_deref().unwrap_or("No URL"),
                config::plain()
                    .string_from_read(
                        item.text
                            .as_deref()
                            .unwrap_or("No content available")
                            .as_bytes(),
                        right.width.into()
                    )
                    .unwrap()
            )
        },
    ))
    .wrap(ratatui::widgets::Wrap { trim: true })
    .block(right_block);

    frame.render_widget(list, left);
    frame.render_widget(article, right);
}
