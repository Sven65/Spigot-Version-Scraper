use chrono::NaiveDateTime;
use sqlite::{Connection, State};

use crate::forum::{ForumPost, ForumPostMeta};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> sqlite::Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> sqlite::Result<()> {
        self.conn.execute(
            "
                CREATE TABLE IF NOT EXISTS posts (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    url TEXT,
                    title TEXT,
                    poster TEXT,
                    date TEXT
                );
            ",
        )?;
        Ok(())
    }

    pub fn insert_post(&self, post: &ForumPost) -> sqlite::Result<()> {
		let mut stmt = self.conn.prepare(
			"INSERT INTO posts (url, title, poster, date) VALUES (?, ?, ?, ?)",
		)?;

		stmt.bind((1, post.url.as_deref().unwrap_or_default()))?;
		stmt.bind((2, post.title.as_deref().unwrap_or_default()))?;
		stmt.bind((3, post.meta.poster.as_deref().unwrap_or_default()))?;

		let date_str = post.meta.date.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string());
		stmt.bind((4, date_str.as_deref().unwrap_or_default()))?;

		stmt.next()?; // executes
		Ok(())
	}

	pub fn get_posts(&self) -> sqlite::Result<Vec<ForumPost>> {
		let mut stmt = self.conn.prepare(
			"SELECT url, title, poster, date FROM posts ORDER BY id DESC",
		)?;
		let mut posts: Vec<ForumPost> = Vec::new();

		while let State::Row = stmt.next()? {
			let url = stmt.read::<String, &str>("url").ok();
			let title = stmt.read::<String, &str>("title").ok();
			let poster = stmt.read::<String, &str>("poster").ok();
			let date_str = stmt.read::<String, &str>("date").ok();

			let date = date_str
				.and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok());

			let meta = ForumPostMeta { poster, date };

			posts.push(ForumPost { url, title, meta });
		}

		Ok(posts)
	}
}