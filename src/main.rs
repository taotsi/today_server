#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // let cors = warp::cors()
    //   .allow_origin("https://hyper.rs")
    //   .allow_methods(vec!["GET", "POST", "DELETE"]);

    // let route = warp::any()
    //   .map(warp::reply)
    //   .with(cors);
    let hello = warp::path!("api" / "hello")
      .map(|| format!("greeting from backend server"));

    warp::serve(hello)
      .run(([127, 0, 0, 1], 3030))
      .await;
}
