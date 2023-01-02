use serde::{Deserialize, Serialize};
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use std::{
    fs,
    path::PathBuf,
};

const ECHANGE_RATE_FILE: &str = "/var/cache/boj/echange_rates.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct ExchangeRates
{
    meta: ExchangeRateResponseMeta,
    data: ExchangeRateResponseData,
}

#[derive(Serialize, Deserialize, Clone)]

struct ExchangeRateResponseData
{
    BTC: ExchangeRateResponseDataInfo,
    CAD: ExchangeRateResponseDataInfo,
    EUR: ExchangeRateResponseDataInfo,
    USD: ExchangeRateResponseDataInfo,
}

#[derive(Serialize, Deserialize, Clone)]
struct ExchangeRateResponseMeta
{
    last_updated_at: String,
}

// Echange rates are floating point numbers that represent
// value relative to USD. USD will always be 1.0

#[derive(Serialize, Deserialize, Clone)]
struct ExchangeRateResponseDataInfo
{
    code: String,
    value: f64,
}

impl ExchangeRates
{
    /// Makes an http reqest using the api_key and saves this JSON
    /// data to `ECHANGE_RATE_FILE`
    pub async fn fetch() -> Result<(), String>
    {
        // Get the api_key
        let api_key = crate::configs::CONFIG.keys.exchange_rate_api_key.clone();


        // Construct request URL
        let url = format!(
            "https://api.currencyapi.com/v3/latest?apikey={}&currencies=EUR%2CUSD%2CCAD%2CBTC",
            api_key
        );

        // Get the response
        let resp = reqwest::get(url).await.unwrap();

        let pfile = PathBuf::from(ECHANGE_RATE_FILE);

        // If the parent directory of the file doesn't exit, we create
        // it.
        if !pfile.parent().unwrap().exists()
        {
            match fs::create_dir_all(pfile.parent().unwrap())
            {
                Ok(_) => (),
                Err(x) =>
                {
                    return Err(format!(
                        "Couldn't create the parent directory '{}': {} ",
                        pfile.parent().unwrap().display(),
                        x
                    ))
                }
            }
        }

        // Write the data to a predetermined file
        match fs::write(
            pfile.clone(),
            &match resp.bytes().await
            {
                Ok(x) => x,
                Err(x) => return Err(format!("Couldn't get response as bytes: {}", x)),
            },
        )
        {
            Ok(_) => (),
            Err(x) =>
            {
                return Err(format!(
                    "Couldn't save reponse to '{}': {}",
                    pfile.display(),
                    x
                ))
            }
        }

        Ok(())
    }

    /// Reads and parses data from `ECHANGE_RATE_FILE`
    fn read() -> Result<ExchangeRates, String>
    {
        let pfile = PathBuf::from(ECHANGE_RATE_FILE);

        if !pfile.exists()
        {
            return Err(format!(
                "Couldn't read from '{}': '{}' Doesn't exist. Fetch it first.",
                pfile.display(),
                pfile.display()
            ));
        }

        let file_contents = match fs::read_to_string(pfile.clone())
        {
            Ok(x) => x,
            Err(x) => return Err(format!("Couldn't read from '{}': {}", pfile.display(), x)),
        };

        // Parse json from cached file
        match serenity::json::prelude::from_str(&file_contents)
        {
            Ok(x) => Ok(x),
            Err(x) => Err(format!(
                "Couldn't parse json from '{}': {}",
                pfile.display(),
                x
            )),
        }
    }

}

enum Currency
{
    BTC(f64),
    CAD(f64),
    EUR(f64),
    USD(f64),
}

impl Currency
{
    fn to_usd(self, rates: ExchangeRates) -> Currency
    {
        match self
        {
            Self::BTC(x) => Currency::USD(rates.data.BTC.value * x),
            Self::CAD(x) => Currency::USD(rates.data.CAD.value * x),
            Self::EUR(x) => Currency::USD(rates.data.EUR.value * x),
            _ => self,
        }
    }

    fn to_btc(self, rates: ExchangeRates) -> Currency
    {
        Self::BTC(f64::from(self.to_usd(rates.clone())) / rates.data.BTC.value)
    }

    fn to_eur(self, rates: ExchangeRates) -> Currency
    {
        Self::EUR(f64::from(self.to_usd(rates.clone())) / rates.data.EUR.value)
    }

    fn to_cad(self, rates: ExchangeRates) -> Currency
    {
        Self::CAD(f64::from(self.to_usd(rates.clone())) / rates.data.CAD.value)
    }
}

// Allow easy converting to f64
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

pub fn run(input: String, target: String) -> String
{
    let rates = match ExchangeRates::read()
    {
        Ok(x) => x,
        Err(x) => return format!("Error: {}", x),
    };

    // Get the input and target values as lowercase
    let mut input = input.to_lowercase();
    let target = target.to_lowercase();

    // Determine the input currency
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
    else
    {
        return "Error: Invalid input currency".to_string();
    }
    

    // Try to parse the currency
    let parsed = match input.parse()
    {
        Ok(x) => x,
        Err(x) => return format!("Error: {}", x),
    };

    // Store everything in the appropriate `Currency` variant
    let input_currency = match input_type
    {
        "BTC" => Currency::BTC(parsed),
        "USD" => Currency::USD(parsed),
        "EUR" => Currency::EUR(parsed),
        "CAD" => Currency::CAD(parsed),
        &_ => return "Error: Invalid input currency".to_string(),
    };

    // Convert each currency based upon the target currency
    let result = match &*target
    {
        "usd" => input_currency.to_usd(rates),
        "cad" => input_currency.to_cad(rates),
        "btc" => input_currency.to_btc(rates),
        "eur" => input_currency.to_eur(rates),
        _ => return "Error: Invalid target".to_string(),
    };

    // Return the formatted result
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
