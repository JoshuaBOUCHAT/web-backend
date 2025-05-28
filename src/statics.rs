use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufWriter, Write, stderr},
    sync::{LazyLock, Mutex},
};

use diesel::{
    SqliteConnection,
    r2d2::{self, ConnectionManager},
};
use lettre::{SmtpTransport, message::Mailbox, transport::smtp::authentication::Credentials};
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
pub static ENV: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let env = include_str!("../.env");
    env.lines()
        .map(|line| {
            line.split_once('=')
                .expect("error when parsing the .env file")
        })
        .collect()
});

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
pub static APP_URL: LazyLock<String> =
    LazyLock::new(|| ENV.get("APP_URL").expect("APP_URL non définie").to_string());

pub static APP_MAIL: LazyLock<String> =
    LazyLock::new(|| ENV.get("APP_MAIL").expect("APP_MAIL not set").to_string());

pub static MAILER: LazyLock<Mutex<SmtpTransport>> = LazyLock::new(|| {
    if let Err(err) = dotenvy::dotenv() {
        eprintln!("error while loading the dotenv file:\n{err}");
    };
    let password = ENV
        .get("MAIL_PASWD")
        .expect("MAIL_PASWD n'est pas défini")
        .to_string();
    let creds = Credentials::new(APP_MAIL.clone(), password);

    // Create a connection to our email provider
    // In this case, we are using Namecheap's Private Email
    // You can use any email provider you want
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    Mutex::new(mailer)
});
pub static APP_MAIL_BOX: LazyLock<Mailbox> =
    LazyLock::new(|| ENV.get("APP_MAIL").unwrap().parse().unwrap());
