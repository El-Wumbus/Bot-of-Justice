use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::{command::CommandOptionType}, builder::CreateApplicationCommand};
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
struct EchangeRateResponse
{
    meta: EchangeRateResponseMeta,
    data: EchangeRateResponseData,
}

impl EchangeRateResponse
{
    async fn fetch(api_key: String) -> PathBuf
    {
        let url = format!(
            "https://api.currencyapi.com/v3/latest?apikey={}&currencies=EUR%2CUSD%2CCAD%2CBTC",
            api_key
        );
        let storage_dirs = AppDirs::new(Some("boj"), false).unwrap();
        dbg!(&storage_dirs);
        let cache_dir = storage_dirs.cache_dir;
        let save_file = cache_dir.join("exchange_rates.json");
        let resp = reqwest::get(url).await.unwrap();

        let mut f = File::create(save_file.clone()).unwrap();
        f.write_all(&resp.bytes().await.unwrap()).unwrap();
        f.flush().unwrap();
        save_file
    }

    fn parse(path: PathBuf) -> EchangeRateResponse
    {
        let data = &*read_to_string(path).unwrap();
        let exchange: EchangeRateResponse = serde_json::from_str(data).unwrap();
        exchange
    }
}

#[derive(Serialize, Deserialize)]

struct EchangeRateResponseData
{
    BTC: EchangeRateResponseDataInfo,
    CAD: EchangeRateResponseDataInfo,
    EUR: EchangeRateResponseDataInfo,
    USD: EchangeRateResponseDataInfo,
}

#[derive(Serialize, Deserialize)]
struct EchangeRateResponseMeta
{
    last_updated_at: String,
}

// Echange rates are floating point numbers that represent
// value relative to USD. USD will always be 1.0

#[derive(Clone)]
pub struct EchangeRates
{
    pub BTC: f64,
    pub CAD: f64,
    pub EUR: f64,
    pub USD: f64,
}

impl EchangeRates
{
    pub async fn from_api(api_key: String) -> EchangeRates
    {
        let response = EchangeRateResponse::parse(EchangeRateResponse::fetch(api_key).await);
        EchangeRates {
            BTC: response.data.BTC.value,
            CAD: response.data.CAD.value,
            EUR: response.data.EUR.value,
            USD: response.data.USD.value,
        }
    }

    pub async fn from_saved() -> EchangeRates
    {
        let storage_dirs = AppDirs::new(Some("boj"), false).unwrap();
        let save_file = storage_dirs.cache_dir.join("exchange_rates.json");
        if !save_file.exists()
        {
            // crate::pull_api().await;
        }

        let response = EchangeRateResponse::parse(save_file);
        EchangeRates {
            BTC: response.data.BTC.value,
            CAD: response.data.CAD.value,
            EUR: response.data.EUR.value,
            USD: response.data.USD.value,
        }

    }
}

#[derive(Serialize, Deserialize)]
struct EchangeRateResponseDataInfo
{
    code: String,
    value: f64,
}

pub enum Currency
{
    BTC(f64),
    CAD(f64),
    EUR(f64),
    USD(f64),
}

impl From<Currency> for f64
{
    fn from(item: Currency) -> Self
    {
        match item
        {
            Currency::BTC(x) => x,
            Currency::CAD(x) => x,
            Currency::EUR(x) => x,
            Currency::USD(x) => x,
        }
    }
}

impl Currency
{
    pub fn to_usd(self, rates: EchangeRates) -> Currency
    {
        match self
        {
            Self::BTC(x) => Currency::USD(rates.BTC * x),
            Self::CAD(x) => Currency::USD(rates.CAD * x),
            Self::EUR(x) => Currency::USD(rates.EUR * x),
            _ => self,
        }
    }

    pub fn to_btc(self, rates: EchangeRates) -> Currency
    {
        Self::BTC(f64::from(self.to_usd(rates.clone())) / rates.BTC.clone())
    }

    pub fn to_eur(self, rates: EchangeRates) -> Currency
    {
        Self::EUR(f64::from(self.to_usd(rates.clone())) / rates.EUR.clone())
    }

    pub fn to_cad(self, rates: EchangeRates) -> Currency
    {
        Self::CAD(f64::from(self.to_usd(rates.clone())) / rates.CAD.clone())
    }
}

pub fn run(input: String, target: String, rates:EchangeRates) -> String
{
    let mut input = input.to_lowercase();
    let target = target.to_lowercase();
    let input_type: &str;
    if input.starts_with('$') || input.ends_with("usd")
    {
        input = input
            .trim()
            .strip_suffix("usd")
            .unwrap_or(&input)
            .strip_prefix('$')
            .unwrap_or(&input)
            .to_string();
        input_type = "USD";
    }
    else if input.starts_with('₿') || input.ends_with("btc")
    {
        input = input
            .trim()
            .strip_suffix("btc")
            .unwrap_or(&input)
            .strip_prefix('₿')
            .unwrap_or(&input)
            .to_string();
        input_type = "BTC";
    }
    else if input.starts_with('€') || input.ends_with("eur")
    {
        input = input
            .trim()
            .strip_suffix("eur")
            .unwrap_or(&input)
            .strip_prefix('€')
            .unwrap_or(&input)
            .to_string();

        input_type = "EUR";
    }
    else if input.ends_with("cad")
    {
        input = input
            .trim()
            .strip_suffix("cad")
            .unwrap_or(&input)
            .to_string();
        input_type = "CAD";
    }
    else {
        return "Error: Invalid input currency".to_string();
    }
    let parsed = input.parse().unwrap_or(0.0);
    let input_currency = match input_type
    {
        "BTC" => Currency::BTC(parsed),
        "USD" => Currency::USD(parsed),
        "EUR" => Currency::EUR(parsed),
        "CAD" => Currency::CAD(parsed),
        &_=> return "Error: Invalid input currency".to_string(),
    };

    let result = match &*target {
        "usd" => input_currency.to_usd(rates),
        "cad" => input_currency.to_cad(rates),
        "btc" => input_currency.to_btc(rates),
        "eur" => input_currency.to_eur(rates),
        _ => return "Error: Invalid target".to_string(),
    };

    format!("{:.1}{}", f64::from(result), target)
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("currency")
        .description("Convert from one Currency to another. Supports USD, CAD, EUR, and BTC")
        .create_option(|option| {
            option
                .name("value")
                .description("Original value (e.g. '$45' [USD], '18.33 CAD' [CAD].")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("target")
                .description("The currency to target. (e.g 'EUR' [Euro], 'BTC' [Bitcoin]).")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
