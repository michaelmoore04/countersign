use ::rand::Rng;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use clap::Parser;

const DEFAULT_LENGTH: usize = 32;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Use alphas
   #[arg(short, long, default_value_t=false)]
   alpha: bool,

   /// Use numerics
   #[arg(short, long, default_value_t=false)]
   numeric: bool,

   /// Use special characters
   #[arg(short, long, default_value_t=false)]
   special: bool,

   /// Number of characters to use
   #[arg(short, long)]
   length: Option<usize>,

   /// Name to save password under
   #[arg(short, long)]
   entry: Option<String>,
}


fn get_random_char(charset: &[u8]) -> char {
    let idx = ::rand::thread_rng().gen_range(0..charset.len());
    charset[idx] as char
}

fn get_charset(args: &Args) -> Vec<u8> {
    let mut alpha: bool = args.alpha;
    let numeric: bool = args.numeric;
    let special: bool = args.special;

    if !numeric && !special {
        alpha = true;
    }

    let uppercase_chars: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase_chars: &str = "abcdefghijklmnopqrstuvwxyz";
    let numeric_chars: &str = "0123456789";
    let special_chars: &str = "!@#$%&*";
    
    let mut chars: String = "".to_owned();
    if alpha {
        chars.push_str(uppercase_chars);
        chars.push_str(lowercase_chars);
    }
    if numeric {
        chars.push_str(numeric_chars);
    }
    if special {
        chars.push_str(special_chars);
    }
    chars.into_bytes() as Vec<u8>
}

fn encrypt_password(pass: &String) -> (String, String) {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    let result = rng.fill(&mut salt);
    if result.is_err() {
        println!("Error with salt");
    }
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        pass.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));
    (HEXUPPER.encode(&salt), HEXUPPER.encode(&pbkdf2_hash))
}

#[async_std::main]
async fn main() {
    let args = Args::parse();
    let charset = get_charset(&args);
    let entry: String = args.entry.unwrap();
    let length: usize = if let Some(len) = args.length.as_ref() {
        *len
    } else {
        DEFAULT_LENGTH
    };
    let pass: String = (0..length)
        .map(|_| get_random_char(&charset))
        .collect();
    println!("{}", pass);

    let (salt, pbkdf2_hash) = encrypt_password(&pass);

    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap(); 
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    
    let qry = 
    "CREATE TABLE IF NOT EXISTS passwords
        (
            id                      INTEGER PRIMARY KEY AUTOINCREMENT,
            entry                   TEXT                NOT NULL,
            password                TEXT                NOT NULL,
            salt                    TEXT                NOT NULL,
            hash                    TEXT                NOT NULL,
            created_on              DATETIME DEFAULT (datetime('now','localtime')),
            updated_on              DATETIME DEFAULT (datetime('now','localtime'))
        );";
    let table_result = sqlx::query(&qry).execute(&instances).await;   
    println!("{:?}", table_result);

    let query = format!("
        INSERT INTO passwords (entry, password, salt, hash) VALUES ( '{}', '{}', '{}', '{}' );", entry.to_string(), pass.to_string(), salt, pbkdf2_hash);
    println!("{}", query); 
    let result = sqlx::query(&query).execute(&instances).await;
    instances.close().await;
    println!("{:?}", result);
}
