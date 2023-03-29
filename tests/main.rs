use robloxapi;
use tokio;

const COOKIE: &str = "";
#[tokio::main]
async fn main() {
    // let params = web::Query::<DevProduct>::from_query(req.query_string()).unwrap();
    let mut client = robloxapi::Client::new();
    client.set_cookie(COOKIE).await;

    let user = client.user(242872495).await;
    // dbg!(user);

    // println!("{}", client.current_user().await);

    let mut game = client.game(3272915504).await.expect("Error");
    // dbg!(&game);
    println!("{:?}", game.servers().await);

    let dev_product = game.create_dev_product("17", 17).await;

    println!("{}", dev_product.unwrap());
}
