use yooso::Yooso;

#[yooso::launch]
async fn yooso() -> Yooso {
    Yooso::build().await
}
