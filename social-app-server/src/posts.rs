use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use async_std::sync::Mutex;
use ron::de::from_reader;
use ron::to_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{format, Debug};
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub const APPLICATION_JSON: &str = "application/json";
static FILE_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    timestamp: u64,
    user: String,
    content: String,
}

#[derive(Deserialize)]
pub struct PostRequest {
    user: String,
    content: String,
}

// TODO: Could make file writing and other operations async_std as well

pub async fn new_message(message_request: web::Json<PostRequest>) -> Result<HttpResponse, Error> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let PostRequest { user, content } = message_request.0;

    let post = Post {
        timestamp,
        user,
        content,
    };

    let _guard = FILE_LOCK.lock().await;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("chat_data.ron")
        .expect("Failed to open chat_data.ron file");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read chat_data.ron");

    let mut data: HashMap<String, Vec<Post>> = from_reader(file_contents.as_bytes())
        .unwrap_or_else(|_| HashMap::<String, Vec<Post>>::new());
    let users_posts = data.entry(post.user.clone()).or_insert_with(Vec::new);

    users_posts.push(post);
    let ron_string =
        to_string(&data).expect(format!("Failed to convert data to string {:?}", data).as_str());

    file.set_len(0)
        .expect("Could not clear chat_data! But could read");
    file.write_all(ron_string.as_bytes())
        .expect(format!("Failed to write to data_ron: {:?}", ron_string).as_str());

    Ok(HttpResponse::Ok().json(format!("{{timestamp: {}}}", timestamp)))
}
