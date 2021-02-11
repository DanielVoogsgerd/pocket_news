use crate::DataFile;
use crate::Config;

use pocket::auth::PocketAuthentication;
use pocket::get::PocketItem;
use pocket::Pocket;
use pocket::send::PocketSendAction;


pub async fn get_pocket_instance(config_file: &str, data_file: &str) -> Pocket {
    let config = Config::read(config_file).expect("Could not load config");
    let consumer_key = config.consumer_key.expect("No consumer key defined");

    let access_token_result: Option<String> = {
        match DataFile::read(data_file) {
            Some(data_file) => {
                data_file.access_token
            }
            None => None
        }
    };

    let pocket = match access_token_result {
        Some(access_token) => {
            Pocket::new(&consumer_key, &access_token)
        },
        None => {
            let (access_token, pocket) =
                authorize_pocket(&consumer_key).await
                .expect("Could not authorize");

            DataFile::store_access_token(data_file, &access_token).unwrap_or_else(|err| {
                eprintln!("Could not store access token. {}", err)
            });

            pocket
        }
    };

    pocket
}


pub fn pocket_items_to_delete_action(items: &Vec<PocketItem>) -> Vec<PocketSendAction> {
    let actions = items.iter().map(|item| {
        PocketSendAction::Delete {
            item_id: item.item_id,
            time: None
        }
    }).collect();

    return actions;
}

pub fn request_access_token() -> Result<String, String> {
    return Ok(String::from("token"));
}

pub async fn authorize_pocket(consumer_key: &str) -> Option<(String, Pocket)> {
    let auth = PocketAuthentication::new(consumer_key, "rustapi:finishauth");
    let state = None;
    let code = auth
        .request(state).await
        .expect("Could not request authentication");

    println!("Follow auth URL to provide access and press enter when finished: {}", auth.authorize_url(&code));
    let _ = std::io::stdin().read_line(&mut String::new());

    let user = auth
        .authorize(&code, state).await
        .expect("Could not authorize");

    println!("Access token: {}", &user.access_token);
    Some((user.access_token.clone(), user.pocket()))
}

