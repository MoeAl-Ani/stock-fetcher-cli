/// api to call https://query1.finance.yahoo.com/v7/finance/quote?=&
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

const YAHOO_URL: &str = "https://query1.finance.yahoo.com/v7/finance/quote?=&";

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum QuoteType {
    #[serde(rename = "CRYPTOCURRENCY")]
    CryptoCurrency,
    #[serde(rename = "MUTUALFUND")]
    MutualFund
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Symbol {
    #[serde(rename = "BTC-USD")]
    Btc
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Currency {
    EUR,
    USD,
}

#[derive(Debug, Deserialize, Serialize)]
struct Root {
    #[serde(rename = "quoteResponse")]
    quote_response: QuoteResponse
}

#[derive(Debug, Deserialize, Serialize)]
struct QuoteResponse {
    result: Vec<StockData>
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct StockData {
    #[serde(rename = "quoteType")]
    quote_type: QuoteType,
    symbol: Symbol,
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: f64,
    #[serde(rename = "regularMarketDayHigh")]
    regular_market_day_high: f64,
    #[serde(rename = "regularMarketDayLow")]
    regular_market_day_low: f64,
    currency: Currency,
}

impl StockData {
    pub fn new(
        quote_type: QuoteType,
        symbol: Symbol,
        regular_market_price: f64,
        regular_market_day_high: f64,
        regular_market_day_low:f64,
        currency: Currency) -> Self {
        StockData {
            quote_type,
            symbol,
            regular_market_price,
            regular_market_day_high,
            regular_market_day_low,
            currency
        }
    }
}

pub struct YahooApi {}

impl YahooApi {
    pub fn new() -> Self {
        YahooApi {}
    }
}

#[async_trait]
impl Api for YahooApi {
    async fn fetch(&self, name: &String, symbol: &String) -> Option<StockData> {
        let mut url = String::from(YAHOO_URL);
        url.push_str(format!("symbols={}", symbol).as_str());
        let res = reqwest::get(&url).await;
        return match res {
            Ok(res) => {
                let data = res.text().await.unwrap();
                let root_result: Root = serde_json::from_str(&data).unwrap();
                let stock = root_result.quote_response.result.get(0);
                let stock_data = stock.unwrap().clone();
                Some(stock_data)
            }
            Err(e) => {
                eprintln!("error = {}", e.to_string());
                None
            }
        };
    }

    fn print(&self) {
        unimplemented!()
    }
}

#[async_trait]
pub trait Api {
    async fn fetch(&self, name: &String, symbol: &String) -> Option<StockData>;
    fn print(&self);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_stock_data() {
        let stock_data = StockData::new(QuoteType::CryptoCurrency, Symbol::Btc, 3000.0, 1000.0, 100.0, Currency::USD);
        match stock_data.quote_type {
            QuoteType::CryptoCurrency => {
                println!("ok");
            }
            _ => {}
        }
    }

    #[test]
    fn test_create_stock_data_serialize() {
        let stock_data = StockData::new(QuoteType::CryptoCurrency, Symbol::Btc, 3000.0, 1000.0, 100.0, Currency::USD);

        let json = serde_json::to_string(&stock_data);
        match json {
            Ok(json) => {
                println!("json = {}", json)
            }
            Err(e) => {
                println!("error = {}", e);
                panic!()
            }
        }
    }

    #[test]
    fn test_create_stock_data_deserialize() {
        let stock_data = StockData::new(QuoteType::CryptoCurrency, Symbol::Btc, 3000.0, 1000.0, 100.0, Currency::USD);

        let json = serde_json::to_string(&stock_data);
        match json {
            Ok(json) => {
                //println!("json = {}", json);
                let stock: StockData = serde_json::from_str(&json).unwrap();
                match stock.quote_type {
                    QuoteType::CryptoCurrency => {
                        println!("ok");
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("error = {}", e);
                panic!()
            }
        }
    }

    #[tokio::test]
    async fn test_create_stock_data_fetch_api() {
        let api = YahooApi::new();
        let name = format!("");
        let value = format!("BTC-USD");
        let option = api.fetch(&name,&value).await;
        match option {
            None => { panic!() }
            Some(data) => {
                println!("stock = {:?}", data)
            }
        }
    }
}