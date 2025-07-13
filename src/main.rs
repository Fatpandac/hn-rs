use std::{
    io::{self, stdout},
    time::Duration,
    usize,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::future::join_all;
use hackernews::{
    StoryType,
    get_items::{ItemResponse, get_item},
    get_stories::get_stories,
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use tokio::{sync::watch, task::JoinHandle, time::sleep};

use crate::app::APP;

mod app;
mod components;
mod panels;

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

#[derive(PartialEq, Debug, Clone)]
enum ChannelAction {
    Story(StoryType),
    Items(Vec<usize>),
}

#[derive(PartialEq, Debug, Clone)]
enum ChannelData {
    Story(Option<Vec<ItemResponse>>),
    Comment(Option<Vec<ItemResponse>>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    setup_terminal()?;
    let _ = terminal.clear();

    let (tx_stop, rx_stop) = watch::channel::<bool>(true);
    let (tx_aciton, rx_action) =
        watch::channel::<ChannelAction>(ChannelAction::Story(StoryType::Show));
    let (tx_data, rx_data) = watch::channel::<ChannelData>(ChannelData::Story(None));

    let mut app = APP::new(tx_aciton.clone(), rx_data.clone());

    tokio::spawn(async move {
        let mut last_topic: Option<StoryType> = None;
        let mut last_item: Option<Vec<usize>> = None;
        let mut story_handle: Option<JoinHandle<()>> = None;
        let mut comment_handle: Option<JoinHandle<()>> = None;

        while *rx_stop.borrow() {
            let rx_topic = rx_action.borrow().clone();

            match &rx_topic {
                ChannelAction::Story(topic) => {
                    if Some(*topic) == last_topic {
                        sleep(Duration::from_millis(300)).await;
                        continue;
                    }

                    if let Some(handle) = story_handle.take() {
                        handle.abort();
                    }

                    last_topic = Some(*topic);
                    tx_data.send(ChannelData::Story(None)).unwrap();

                    let tx_data = tx_data.clone();
                    let topic_copy = *topic;
                    story_handle = Some(tokio::spawn(async move {
                        let list = get_stories(topic_copy).await.unwrap_or_default();
                        let responses = join_all(list.iter().map(|&id| get_item(id)))
                            .await
                            .into_iter()
                            .filter_map(Result::ok)
                            .collect::<Vec<_>>();

                        let _ = tx_data.send(ChannelData::Story(Some(responses)));
                    }));
                }

                ChannelAction::Items(items) => {
                    if Some(items.clone()) == last_item {
                        sleep(Duration::from_millis(300)).await;
                        continue;
                    }

                    if let Some(handle) = comment_handle.take() {
                        handle.abort();
                    }

                    last_item = Some(items.clone());
                    tx_data.send(ChannelData::Comment(None)).unwrap();

                    if !items.is_empty() {
                        let tx_data = tx_data.clone();
                        let items_copy = items.clone();
                        comment_handle = Some(tokio::spawn(async move {
                            let responses = join_all(items_copy.iter().map(|&id| get_item(id)))
                                .await
                                .into_iter()
                                .filter_map(Result::ok)
                                .collect::<Vec<_>>();

                            let _ = tx_data.send(ChannelData::Comment(Some(responses)));
                        }));
                    }
                }
            }

            sleep(Duration::from_millis(300)).await;
        }
    });

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    tx_stop.send(false).unwrap();
                    break;
                } else {
                    app.handle_event(key, tx_aciton.clone());
                }
            }
        }

        terminal.draw(|f| {
            app.draw(f, rx_data.clone()).unwrap();
        })?;

        app.update_data();
    }

    restore_terminal()?;
    Ok(())
}
