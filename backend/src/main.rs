use backend::run_backend;
use backend::models::Apod;

#[tokio::main]
async fn main() {

    //testing leftover from making sure I could translate from nasa api to apod struct
    /* 
    let apod_url = "https://api.nasa.gov/planetary/apod?api_key=HbP7U12I4K6CKbozeINP0PogXXL0fbiabLZ7jVjf";
    let response = reqwest::get(apod_url)
    .await
    .expect("Failed to fetch APOD data")
    .json::<Apod>()
    .await
    .expect("Failed to parse JSON");


    println!("Fetched APOD success: {:#?}", response);

    assert_eq!(response.media_type, "image");
    assert!(!response.title.is_empty());
    */

    run_backend().await;
}
