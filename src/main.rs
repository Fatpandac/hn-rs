use std::{
    io::{self, stdout},
    time::Duration, usize,
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
use tokio::{sync::watch, time::sleep};

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
    let (tx_aciton, rx_action) = watch::channel::<ChannelAction>(ChannelAction::Story(StoryType::Show));
    let (tx_data, rx_data) = watch::channel::<ChannelData>(ChannelData::Story(None));

    let mut app = APP::new(tx_aciton.clone(), rx_data.clone());

    tokio::spawn(async move {
        let mut last_topic: Option<StoryType> = None;
        let mut last_item: Option<Vec<usize>> = None;

        while *rx_stop.borrow() {
            let rx_topic = rx_action.borrow().clone();
            if let Some(last) = last_topic {
                if let ChannelAction::Story(topic) = rx_topic {
                    if topic == last {
                        sleep(Duration::from_millis(300)).await;
                        continue;
                    } else {
                        tx_data.send(ChannelData::Story(None)).unwrap();
                    }
                } else if let ChannelAction::Items(ref items) = rx_topic {
                    if *items == last_item.clone().unwrap_or_default() {
                        sleep(Duration::from_millis(300)).await;
                        continue;
                    } else {
                        tx_data.send(ChannelData::Comment(None)).unwrap();
                    }
                }
            }

            match &rx_topic {
                ChannelAction::Story(topic) => {
                    last_topic = Some(*topic);

                    let list = get_stories(*topic).await.unwrap();
                    let responses = join_all(list.iter().map(|&id| get_item(id)))
                        .await
                        .into_iter()
                        .filter(|res| res.is_ok())
                        .map(|res| res.unwrap())
                        .collect::<Vec<_>>();

                    if *topic == last_topic.unwrap() {
                        tx_data.send(ChannelData::Story(Some(responses))).unwrap();
                    }
                }
                ChannelAction::Items(item) => {
                    last_item = Some(item.clone());
                    if !item.is_empty() {
                        let responses = join_all(item.iter().map(|&id| get_item(id)))
                            .await
                            .into_iter()
                            .filter(|res| res.is_ok())
                            .map(|res| res.unwrap())
                            .collect::<Vec<_>>();
                        tx_data.send(ChannelData::Comment(Some(responses))).unwrap();
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
