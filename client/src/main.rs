use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let response = client.get("http://localhost:3000/questions")
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    let response = client.get("http://localhost:3000/questions")
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    let response = client.post("http://localhost:3000/question")
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);



/* 
    	    // Make a GET request to the APOD API
    let response = reqwest::get("https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY")
        .await?
        .json::<serde_json::Value>()
        .await?;
 
    // Extract the URL of the picture of the day
    let picture_url = response["url"].as_str().unwrap_or("");
 
    // Perform any necessary edits on the picture of the day
    // ...
 
    // Print the edited picture URL
    println!("Edited picture of the day: {}", picture_url);
*/

    Ok(())
}

