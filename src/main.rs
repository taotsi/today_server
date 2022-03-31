#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let cors = warp::cors()
      .allow_origin("https://hyper.rs")
      .allow_methods(vec!["GET", "POST", "DELETE"]);

    // let route = warp::any()
    //   .map(warp::reply)
    //   .with(cors);
    let routes = warp::any()
      .map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}