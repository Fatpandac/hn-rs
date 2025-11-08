use std::time::Duration;

use crossbeam_channel::unbounded;
use futures::future::join_all;
use hackernews::{
    StoryType,
    get_items::{ItemResponse, get_item},
    get_stories::get_stories,
};
use ratatui::{DefaultTerminal, crossterm::event};
use tokio::{task::JoinHandle, time::sleep};

use crate::app::App;

mod app;
mod components;
mod panels;
mod storages;

#[derive(PartialEq, Debug, Clone)]
enum AppAction {
    Story(StoryType),
    Items(Vec<usize>),
}

#[derive(PartialEq, Debug, Clone)]
enum AppData {
    Story(Option<Vec<ItemResponse>>),
    Comment(Option<ItemResponse>),
}

async fn fetch_tree_item(item_id: usize) -> Option<ItemResponse> {
    let mut item = match get_item(item_id).await {
        Ok(it) => it,
        Err(_) => return None,
    };

    if let Some(kids) = &item.kids {
        let futures = kids
            .iter()
            .map(|&kid_id| async move { fetch_tree_item(kid_id).await });

        item.children = Some(
            join_all(futures)
                .await
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        );
    } else {
        item.children = None;
    }

    Some(item)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    run(&mut terminal)?;
    ratatui::restore();

    Ok(())
}

fn run(terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    let (tx_aciton, rx_action) = unbounded();
    let (tx_data, rx_data) = unbounded();

    let mut app = App::new(tx_aciton.clone(), tx_data.clone());

    tokio::spawn(async move {
        let mut last_topic: Option<StoryType> = None;
        let mut last_item: Option<Vec<usize>> = None;
        let mut story_handle: Option<JoinHandle<()>> = None;
        let mut comment_handle: Option<JoinHandle<()>> = None;

        loop {
            if let Ok(rx_topic) = rx_action.try_recv() {
                match rx_topic {
                    AppAction::Story(topic) => {
                        if Some(topic) == last_topic {
                            sleep(Duration::from_millis(300)).await;
                            continue;
                        }

                        if let Some(handle) = story_handle.take() {
                            handle.abort();
                        }

                        last_topic = Some(topic);

                        let tx_data = tx_data.clone();
                        let topic_copy = topic;
                        story_handle = Some(tokio::spawn(async move {
                            let list = get_stories(topic_copy).await.unwrap_or_default();
                            let responses = join_all(list.iter().map(|&id| get_item(id)))
                                .await
                                .into_iter()
                                .filter_map(Result::ok)
                                .collect::<Vec<_>>();

                            let _ = tx_data.send(AppData::Story(Some(responses)));
                        }));
                    }

                    AppAction::Items(items) => {
                        if Some(items.clone()) == last_item {
                            sleep(Duration::from_millis(300)).await;
                            continue;
                        }

                        if let Some(handle) = comment_handle.take() {
                            handle.abort();
                        }

                        last_item = Some(items.clone());

                        if !items.is_empty() {
                            let tx_data = tx_data.clone();
                            let items_copy = items.clone();
                            comment_handle = Some(tokio::spawn(async move {
                                for &id in &items_copy {
                                    if let Some(item) = fetch_tree_item(id).await {
                                        let _ = tx_data.send(AppData::Comment(Some(item)));
                                    }
                                }
                            }));
                        }
                    }
                }
            }

            sleep(Duration::from_millis(300)).await;
        }
    });

    // Initial load
    tx_aciton.send(AppAction::Story(StoryType::Show))?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            let ev = event::read()?;
            app.handle_event(ev);
        }

        if !app.is_running {
            break;
        }

        if app.should_draw() {
            terminal.draw(|f| {
                app.draw(f).unwrap();
            })?;
        }

        while let Ok(data_event) = rx_data.try_recv() {
            app.update_data(data_event);
        }
    }

    Ok(())
}
