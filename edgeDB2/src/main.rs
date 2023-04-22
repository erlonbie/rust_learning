use edgedb_derive::Queryable;
// use edgedb_protocol::value::Value;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
// use edgedb_tokio::Client;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[edgedb(json)]
struct Movie {
    title: String,
    id: Uuid,
    release_year: i32,
}

#[tokio::main]
async fn main() {
    let conn = edgedb_tokio::create_client().await.unwrap();

    {
    // let query: String = conn
    //     .query_required_single("select {'This is a query fetching a string'}", &())
    //     .await
    //     .unwrap();
    // println!("First result is: {}", query);

    // let query2: Value = conn
    //     .query_required_single("insert Movie {title := 'Matrix 0', release_year := 1992};", &())
    //     .await
    //     .unwrap();
    // println!("{query2:#?}");

    // let query3: Vec<Value> = conn
    //     .query("select Movie { title, release_year };", &())
    //     .await
    //     .unwrap();
    // println!("{query3:#?}");

    // let query4: Vec<Value> = conn
    //     .query("insert Account {username := 'Erlon'};", &())
    //     .await
    //     .unwrap();
    // println!("{query4:#?}");

    // let query5: Vec<Value> = conn
    //     .query("select Account { username };", &())
    //     .await
    //     .unwrap();
    //
    // println!("{query5:#?}");
    //
    // get_client(&conn).await;

    }

    let query6: Movie = conn
        .query_required_single(
            "select <json>(insert Movie {
            title := 'Some movie 6',
            release_year := 2000
        }) {
            title,
            release_year,
            id,
            message := 'Hello world'
        };",
            &(),
        )
        .await
        .unwrap();

    // if let Value::Object { shape, fields } = query6 {
    //     println!("{fields:#?}");
    //     // println!("Movie title: {}", fields[shape.get_field("title").unwrap()]);
    // }

    // let as_string = query6.to_string();
    // let as_movie = serde_json::from_str::<Movie>(&as_string).unwrap();
    println!("{query6:#?}");



}

// async fn get_client(client: &Client) {
//     let query: Vec<Value> = client
//         .query("select Person { name };", &())
//         .await
//         .unwrap();
//     println!("{query:#?}");
// }
