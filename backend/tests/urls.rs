use common::spawn_app;

mod common;

fn get_client() -> reqwest::Client {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to construct request client")
}

#[tokio::test]
async fn posting_url_and_requesting_returns_the_same_url() {
    let url = "https://www.rust-lang.org/";

    let app = spawn_app().await;
    let client = get_client();

    let post_response = client
        .post(&format!("{}", &app.address))
        .json(url)
        .send()
        .await
        .expect("Failed to post url");

    assert_eq!(reqwest::StatusCode::CREATED, post_response.status());

    let slug = post_response
        .json::<String>()
        .await
        .expect("Failed to read body.");

    let get_response = client
        .get(&format!("{}/{}", &app.address, &slug))
        .send()
        .await
        .expect("Failed to get url");

    assert_eq!(
        reqwest::StatusCode::MOVED_PERMANENTLY,
        get_response.status()
    );

    let headers = get_response.headers();

    let location = headers
        .get(reqwest::header::LOCATION)
        .map(|h| h.to_str().map_err(|e| e.to_string()));

    assert_eq!(Some(Ok(url)), location);
}

#[tokio::test]
async fn posting_the_same_url_twice_creates_the_same_slug() {
    let url = "https://haskell.pl-a.net/";

    let app = spawn_app().await;
    let client = get_client();

    let slug_1 = client
        .post(&format!("{}", &app.address))
        .json(url)
        .send()
        .await
        .expect("Failed to post url")
        .json::<String>()
        .await
        .expect("Failed to read body.");

    let slug_2 = client
        .post(&format!("{}", &app.address))
        .json(url)
        .send()
        .await
        .expect("Failed to post url")
        .json::<String>()
        .await
        .expect("Failed to read body.");

    assert_eq!(slug_1, slug_2);
}

#[tokio::test]
async fn requesting_a_slug_that_doesnt_exist_gives_a_404() {
    let app = spawn_app().await;
    let client = get_client();

    let get_response = client
        .get(&format!("{}/aabbcc", &app.address))
        .send()
        .await
        .expect("Failed to get url");

    assert_eq!(reqwest::StatusCode::NOT_FOUND, get_response.status());
}

#[tokio::test]
async fn requesting_an_invalid_slug_gives_a_400() {
    let app = spawn_app().await;
    let client = get_client();

    let get_response = client
        .get(&format!("{}/%40%23(0a)", &app.address))
        .send()
        .await
        .expect("Failed to get url");

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, get_response.status());
}

#[tokio::test]
async fn posting_url_an_invalid_url_gives_a_400() {
    let url = "not a url";

    let app = spawn_app().await;
    let client = get_client();

    let post_response = client
        .post(&format!("{}", &app.address))
        .json(url)
        .send()
        .await
        .expect("Failed to post url");

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, post_response.status());
}
