use chrono::NaiveDateTime;
use regex::Regex;
use reqwest::blocking::Client;
use scraper::html::Select;

use crate::constants;

#[derive(Debug)]
pub struct ForumPostMeta {
    pub poster: Option<String>,
    pub date: Option<NaiveDateTime>
}

#[derive(Debug)]
pub struct ForumPost {
    pub url: Option<String>,
    pub title: Option<String>,
    pub meta: ForumPostMeta, 
}

pub fn get_forum() -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .user_agent(constants::REQWEST_USER_AGENT)
        .build()?;

    return client.get(constants::FORUM_URL)
        .send()?
        .text()
}

fn parse_meta(raw: &str) -> Option<(String, NaiveDateTime)> {
	let text = raw
		.trim()
		.replace('\n', " ")
		.replace('\t', " ")
		.split_whitespace()
		.collect::<Vec<_>>()
		.join(" ");

	let re = Regex::new(r"by\s+(?P<author>\S+)\s*:\s*(?P<date>.+)").unwrap();

	let caps = re.captures(&text)?;
	let author = caps["author"].to_string();
	let date_str = &caps["date"];

	let datetime = NaiveDateTime::parse_from_str(date_str, "%b %d, %Y at %I:%M %p").ok()?;

	Some((author, datetime))
}


pub fn parse_posts(html_posts: Select<'_, '_>) -> Vec<ForumPost> {
    let mut posts: Vec<ForumPost> = Vec::new();

    for html_post in html_posts {
        // scraping logic to retrieve the info
        // of interest
        let url = html_post
            .select(&scraper::Selector::parse(".subHeading > a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
      
        let title = html_post
            .select(&scraper::Selector::parse(".subHeading > a").unwrap())
            .next()
            .map(|a| a.text().collect::<String>());

        let html_meta_text = html_post
            .select(&scraper::Selector::parse(".primaryContent > .metaData > .dateData").unwrap())
            .next()
            .map(|el| el.text().collect::<String>());

        let html_meta_parsed = parse_meta(&html_meta_text.clone().unwrap()).unwrap();


        let meta = ForumPostMeta {
            poster: Some(html_meta_parsed.0),
            date: Some(html_meta_parsed.1),
        };

        // instantiate a new product
        // with the scraped data and add it to the list
        let post = ForumPost {
            url,
            title,
            meta,
        };
        posts.push(post);
    }

    posts
}