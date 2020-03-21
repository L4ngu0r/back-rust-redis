use redis::{AsyncCommands};
use redis::aio::Connection;

use std::collections::HashMap;
use std::convert::Infallible;
use std::time::{SystemTime, UNIX_EPOCH};

use warp::Filter;
use warp::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Todo {
    id: String,
    name: String,
    description: String
}

async fn connect() -> redis::RedisResult<Connection> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let con = client.get_async_connection().await?;
    Ok(con)
}

async fn get_all_hash(con: &mut Connection) -> redis::RedisResult<HashMap<String, String>> {
    con.hgetall("todo:1234").await
}

async fn get_all_keys(con: &mut Connection) -> redis::RedisResult<Vec<String>> {
    con.hkeys("todos:1234").await
}

async fn list_all() -> Result<impl warp::Reply, Infallible> {
    let mut con = connect().await.unwrap();
    let hashes = get_all_hash(&mut con).await.unwrap();
    println!("{:?}", &hashes);
    Ok(warp::reply::json(&hashes))
}

async fn add_todo(todo: Todo) -> Result<impl warp::Reply, Infallible> {
    let mut con = connect().await.unwrap();
    con.hset::<String, String, String, ()>(todo.id, todo.name, todo.description).await.unwrap();
    Ok(StatusCode::CREATED)
}

async fn delete_todo(id: String) -> Result<impl warp::Reply, Infallible> {
    let mut con = connect().await.unwrap();
    con.del::<String, ()>(id).await.unwrap();
    Ok(StatusCode::OK)
}

async fn update_todo(todo: Todo) -> Result<impl warp::Reply, Infallible> {
    let mut con = connect().await.unwrap();
    con.hset::<String, String, String, ()>(todo.id, todo.name, todo.description).await.unwrap();
    Ok(StatusCode::OK)
}

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    /* let hash_index: String = ["todo", "date"].join(":");
    println!("{}", hash_index);

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let in_ms = since_the_epoch.as_millis();
    println!("{}", in_ms);
    // con.zadd("todo:id", 1234.0, "todo:1")?; */

    let all = warp::path("all")
        .and(warp::get())
        .and_then(list_all);

    let add = warp::post()
        .and(warp::path("add"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(|todo: Todo| add_todo(todo));

    let delete = warp::delete()
        .and(warp::path("del"))
        .and(warp::path::param::<String>())
        .and_then(|id| delete_todo(id));

    let put = warp::put()
        .and(warp::path("update"))
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .and_then(|id: String, todo: Todo| update_todo(todo));

    let routes = warp::any().and(all.or(add).or(delete).or(put));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[tokio::test]
async fn routes() {
    let all = warp::path("all")
        .and(warp::get())
        .and_then(list_all);

    let all_req = || warp::test::request().path("/all");

    assert!(all_req().matches(&all).await);

    let del = warp::delete()
        .and(warp::path("del"))
        .and(warp::path::param::<String>())
        .and_then(|id| delete_todo(id));

    let req = || warp::test::request().method("DELETE").path("/del/1");

    assert!(req().matches(&del).await);
}
