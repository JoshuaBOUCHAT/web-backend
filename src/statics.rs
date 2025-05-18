use std::{
    fs::OpenOptions,
    io::{BufWriter, Write, stderr},
    sync::{LazyLock, Mutex},
};

use diesel::{
    SqliteConnection,
    r2d2::{self, ConnectionManager},
};
use tera::Tera;

pub static TERA: LazyLock<Tera> = LazyLock::new(|| {
    let tera = Tera::new("html/**/*.html").expect("Failed to load templates");
    println!(
        "Loaded templates: {:?}",
        tera.get_template_names().collect::<Vec<_>>()
    );
    tera
});
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub static DB_POOL: LazyLock<DbPool> = LazyLock::new(|| {
    let manager = ConnectionManager::<SqliteConnection>::new("database.db");
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});
pub static LOG_FILE: LazyLock<Mutex<BufWriter<Box<dyn Write + Send + Sync>>>> =
    LazyLock::new(|| {
        if let Err(err) = dotenvy::dotenv() {
            eprintln!("error while loading the dotenv file:\n{err}");
        }
        let Ok(path) = std::env::var("LOG_URL") else {
            return Mutex::new(BufWriter::new(Box::new(stderr())));
        };
        if let Ok(file) = OpenOptions::new().append(true).create(true).open(&path) {
            return Mutex::new(BufWriter::new(Box::new(file)));
        }
        panic!("File path of the log file invalide! :{}", &path);
    });

pub enum AppState {
    Prod,
    Dev,
}

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(|| {
    if let Err(err) = dotenvy::dotenv() {
        eprintln!("error while loading the dotenv file:\n{err}");
    }
    let Ok(state_string) = std::env::var("APP_STATE") else {
        panic!("APP_STATE do not existe or is not valide. Valide state are: prod, dev");
    };
    match state_string.as_str() {
        "dev" => AppState::Dev,
        "prod" => AppState::Prod,
        invalide => {
            panic!(
                "Invalide app state:{} the only possiblities are: dev, prod",
                invalide
            )
        }
    }
});
