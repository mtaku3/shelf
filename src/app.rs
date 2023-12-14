use std::{path::{Path, PathBuf}, env, fs::canonicalize};

use anyhow::Result;
use rusqlite::Connection;

pub struct Config {
    path: PathBuf,
    conn: Connection,
}

impl Config {
    pub fn new(path: Option<&Path>) -> Result<Self> {
        let default_path = dirs::home_dir().unwrap().join(".shelf");
        let path = path.unwrap_or(&default_path);
        let config = Self {
            path: path.to_path_buf(),
            conn: Self::open(path)?,
        };
        config.init_table()?;
        Ok(config)
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    fn open(path: &Path) -> Result<Connection> {
        Ok(Connection::open(path)?)
    }

    fn init_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS book (
                category TEXT NOT NULL,
                tag TEXT NOT NULL,
                path TEXT UNIQUE NOT NULL,
                PRIMARY KEY (category, tag)
            )", ()
        )?;
        Ok(())
    }

    pub fn store(&self, book: &Book) -> Result<()> {
        self.conn.execute(
            "INSERT INTO book (
                category,
                tag,
                path
            ) VALUES (?1, ?2, ?3)",
            (&book.category, &book.tag, &book.path)
        )?;
        Ok(())
    }

    pub fn visit(&self, category: Option<&str>) -> Result<Vec<Book>> {
        let mut stmt = match category.is_some() {
            true => self.conn.prepare("SELECT category, tag, path FROM book WHERE category = ?1 ORDER BY category, tag")?,
            false => self.conn.prepare("SELECT category, tag, path FROM book ORDER by category, tag")?,
        };
        let rows = match category.is_some() {
            true => stmt.query([category.unwrap()])?,
            false => stmt.query([])?,
        };
        let books_iter = rows.mapped(|row| {
            Ok(Book {
                category: row.get(0)?,
                tag: row.get(1)?,
                path: row.get(2)?,
            })
        });
       Ok(books_iter.map(|v| v.unwrap()).collect())
    }

    pub fn throw(&self, category: &str, tag: Option<&str>) -> Result<()> {
        if tag.is_some() {
            let mut stmt = self.conn.prepare("DELETE FROM book WHERE category = ?1 AND tag = ?2")?;
            stmt.execute([category, tag.unwrap()])?;
        } else {
            let mut stmt = self.conn.prepare("DELETE FROM book WHERE category = ?1")?;
            stmt.execute([category])?;
        }
        Ok(())
    }

    pub fn read(&self, category: &str, tag: &str) -> Result<Book> {
        let book = self.conn.query_row("SELECT category, tag, path FROM book WHERE category = ?1 AND tag = ?2", [category, tag], |row| {
            Ok(Book {
                category: row.get(0)?,
                tag: row.get(1)?,
                path: row.get(2)?,
            })
        })?;
        Ok(book)
    }
}

pub struct Book {
    category: String,
    tag: String,
    path: String,
}

impl Book {
    pub fn print(books: &[Book]) -> Result<()> {
        for book in books {
            println!("{}\t{}\t{}", book.category, book.tag, book.path);
        }
        Ok(())
    }
}
        

pub fn store(config: &Config, category: &str, tag: &str, path: Option<&str>) -> Result<Book> {
    let current_dir = env::current_dir()?;
    let path = match path.is_some() {
        true => PathBuf::from(path.unwrap()),
        false => current_dir,
    };
    path.try_exists()?;
    let book = Book {
        category: category.to_string(),
        tag: tag.to_string(),
        path: canonicalize(&path)?.into_os_string().into_string().unwrap(),
    };
    config.store(&book)?;
    Ok(book)
}

pub fn visit(config: &Config, category: Option<&str>) -> Result<Vec<Book>> {
    config.visit(category)
}

pub fn throw(config: &Config, category: &str, tag: Option<&str>) -> Result<()> {
    config.throw(category, tag)
}

pub fn read(config: &Config, category: &str, tag: &str) -> Result<Book> {
    config.read(category, tag)
}
