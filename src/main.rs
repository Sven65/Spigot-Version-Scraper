use std::{collections::HashSet, env, process};

use reqwest::blocking::Client;
use dotenvy::dotenv;

use crate::{db::Database, forum::{ForumPost, get_forum, parse_posts}};

mod constants;
mod forum;
mod db;

fn check_env_var(key: &str) {
	match env::var(key) {
		Ok(val) if !val.is_empty() => {},
		_ => {
			eprintln!("Error: {} is not set or is empty!", key);
			process::exit(1);
		}
	}
}

fn handle_new_post(post: &ForumPost) -> Result<(), reqwest::Error> {
	println!("New post found!");

    let client = Client::builder()
        .user_agent(constants::REQWEST_USER_AGENT)
        .build()?;

    let res = client.post(env::var("DISCORD_WEBHOOK_URL").unwrap())
        .header("Content-Type", "application/json")
        .body(format!(r#"{{
            "embeds": [
                {{
                    "title": "{}",
                    "description": "Posted at: {} by {}",
                    "color": 3395365,
                    "footer": {{
                        "text": ""
                    }},
                    "author": {{
                        "name": "New Spigot Main Post!",
                        "url": "http://spigotmc.com/{}"
                    }},
                    "fields": []
                }}
            ]
        }}"#,
            post.title.clone().unwrap(),
            post.meta.date.unwrap(),
            post.meta.poster.clone().unwrap(),
            post.url.clone().unwrap()
        ))
        .send()?;

    if !res.status().is_success() {
        eprintln!("Webhook failed with status: {}", res.status());
    }

    Ok(())
}

fn main() -> sqlite::Result<()>  {
    dotenv().ok();

    check_env_var("DISCORD_WEBHOOK_URL");


    let db = Database::new("forum.db")?;

	println!("Checking for new posts.");


    match get_forum() {
        Ok(body) => {
            let document = scraper::Html::parse_document(&body);

            let forum_post_selector = scraper::Selector::parse("#articlesGrid > .articleItem").unwrap();
            let forum_posts = document.select(&forum_post_selector);

            let posts = parse_posts(forum_posts);

            let existing_posts = db.get_posts()?;
			let existing_urls: HashSet<String> = existing_posts
				.into_iter()
				.filter_map(|p| p.url)
				.collect();

            for post in posts {
				if let Some(url) = &post.url {
					if !existing_urls.contains(url) {
						db.insert_post(&post)?;
						let _ = handle_new_post(&post);
					}
				}
			}
        }
        Err(err) => {
            println!("Failed to get forum: {err}");
        }
    }

    println!("All done. Exiting.");

    Ok(())
}
