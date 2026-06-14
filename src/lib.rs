
use reqwest;
use url::Url;

pub enum Currency {
    USD,
    EUR,
    GBP
}

#[derive(Debug)]
pub enum FetchError {
    General,
    InsecureProtocol,
    HttpError(String),
}

pub async fn fetch_currency_data(currencies_list: Vec<Currency>) -> String {
    unimplemented!();
}

pub async fn fetch_data(url: String) -> Result<String, FetchError> {
    
    let parsed_url = Url::parse(&url[..]).unwrap();

    if parsed_url.scheme() != "https" {
        return Err(FetchError::InsecureProtocol);
    }
    
    let result = reqwest::get(&url[..]).await;
    
    if result.is_err() {
        println!("There was an error!");
        return Err(FetchError::General);
    }

    let contents = result.unwrap().text().await;

    match contents {
        Ok(c) => {
            println!("{}", c);
            return Ok(String::from("Ok"));
        },
        Err(e) => {
            println!("There was an error"); 
            return Err(FetchError::General);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn test_reqwst() {

    }
}
