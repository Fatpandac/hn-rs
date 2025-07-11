use std::{
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use hackernews::{
    StoryType,
    get_items::{ItemResponse, get_item},
    get_stories::get_stories,
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use tokio::sync::watch;

use crate::app::APP;

mod app;
mod article;
mod component;
mod list;

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
    let _ = terminal.clear();

    let (tx_topic, rx_topic) = watch::channel::<StoryType>(StoryType::Show);
    let (tx, rx) = watch::channel::<Option<ItemResponse>>(None);

    let mut app = APP::new(tx_topic.clone(), rx.clone());

    tokio::spawn(async move {
        let mut last_topic: Option<StoryType> = None;

        loop {
            if last_topic == Some(*rx_topic.borrow()) {
                continue;
            }

            let topic = rx_topic.borrow().clone();
            last_topic = Some(topic);

            let list = get_stories(topic).await.unwrap();

            for &id in list.iter().take(30) {
                let res = get_item(id).await.unwrap();
                if last_topic != Some(*rx_topic.borrow()) {
                    let _ = tx.send(None);
                    break;
                } else {
                    let _ = tx.send(Some(res));
                }
            }
        }
    });

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                } else {
                    app.handle_event(key);
                }
            }
        }

        terminal.draw(|f| {
            app.draw(f).unwrap();
        })?;

        app.update_data();
    }

    restore_terminal()?;
    Ok(())
}
