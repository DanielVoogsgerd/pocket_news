extern crate pocket_news;

use pocket_news::pocket_helpers::pocket_items_to_delete_action;
use pocket_news::pocket_helpers::get_pocket_instance;

use std::error::Error;

use pocket::get::PocketGetTag;
use pocket::send::PocketSendAction;
use pocket::send::PocketSendRequest;

use rss::Channel;
use tokio;
use url::Url;

const APPLICATION_NAME: &str = "pocket_news";
const CONFIG_FILE: &str = "config.toml";
const DATA_FILE: &str = "settings.toml";

const SOURCE: &str = "HackerNews";
const URL: &str = "https://hnrss.org/frontpage";

fn rss_feed_to_create_actions(channel: &Channel) -> Vec<PocketSendAction> {
    channel.items.iter().filter_map(|item| {
        if item.link.is_none() {
            return None
        }

        let url = Url::parse(&item.link.clone().unwrap());

        if url.is_err() {
            return None
        }

        let url = url.unwrap();

        let action = PocketSendAction::Add {
            item_id: None,
            ref_id: None,
            title: item.title.clone(),
            url: Some(url),
            tags: Some(SOURCE.to_string()),
            time: None
        };

        Some(action)
    }).collect()
}

async fn rss_to_channel(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let pocket = get_pocket_instance(CONFIG_FILE, DATA_FILE).await;

    // Delete all articles matching a certain filter
    let mut f = pocket.filter();
    f.complete();
    f.tag(PocketGetTag::Tagged(SOURCE));

    let current_items = pocket.get(&f).await?;

    println!("Found {} old articles, deleting them", current_items.len());
    let delete_actions = pocket_items_to_delete_action(&current_items);

    if delete_actions.len() > 0 {
        let delete_action_refs: Vec<&PocketSendAction> = delete_actions.iter().collect();

        pocket.send(&PocketSendRequest {
            actions: &delete_action_refs.as_slice()
        }).await?;
    } else {
        println!("No articles found. Continuing")
    }

    // # Create pocket entries for all items in a RSS Feed
    let nos_channel = rss_to_channel(URL).await.expect("Could not load NOS");

    let create_actions = rss_feed_to_create_actions(&nos_channel);

    println!("Found {} new articles. Adding them", create_actions.len());
    let action_refs: Vec<&PocketSendAction> = create_actions.iter().collect::<Vec<&PocketSendAction>>();

    match pocket.send(&PocketSendRequest {
        actions: &action_refs.as_slice()
    }).await {
        Ok(_) => {}
        Err(e) => {println!(":( {}", e)}
    };

    Ok(())
}
