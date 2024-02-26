use reqwest::Error;
use crate::star_wars::person::Person;

pub async fn get_person(id: u8) -> Result<Person, Error> {
    let url = format!("https://swapi.dev/api/people/{}", id);
    let http_response = reqwest::get(url).await?;
    let response = http_response.json::<Person>().await?;

    Ok(response)
}