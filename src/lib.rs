pub mod config;
pub use config::{DataFile, Config};

pub mod pocket_helpers;
// pub use pocket_helpers;

// const APPLICATION_NAME: &str = "pocket_news";
// const CONFIG_FILE: &str = "config.toml";
// const DATA_FILE: &str = "settings.toml";

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let pocket = get_pocket_instance(CONFIG_FILE, DATA_FILE).await;

//     // Delete all articles matching a certain filter
//     let mut f = pocket.filter();
//     f.complete();
//     f.tag(PocketGetTag::Tagged("NOS"));

//     let current_items = pocket.get(&f).await?;

//     println!("Found {} old articles, deleting them", current_items.len());
//     let delete_actions = pocket_items_to_delete_action(&current_items);

//     if delete_actions.len() > 0 {
//         let delete_action_refs: Vec<&PocketSendAction> = delete_actions.iter().collect();

//         pocket.send(&PocketSendRequest {
//             actions: &delete_action_refs.as_slice()
//         }).await?;
//     } else {
//         println!("No articles found. Continuing")
//     }

//     // # Create pocket entries for all items in a RSS Feed
//     let nos_channel = nos_feed().await.expect("Could not load NOS");

//     let create_actions = rss_feed_to_create_actions(&nos_channel);

//     println!("Found {} new articles. Adding them", create_actions.len());
//     let action_refs: Vec<&PocketSendAction> = create_actions.iter().collect::<Vec<&PocketSendAction>>();
//     pocket.send(&PocketSendRequest {
//      actions: &action_refs.as_slice()
//     }).await?;

//     Ok(())
// }
