use serde::{Deserialize, Serialize};
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use std::{fs, path::PathBuf};

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
    // Euro
    EUR: ExchangeRateResponseDataInfo,
    /// U.S. Dollar
    USD: ExchangeRateResponseDataInfo,
    /// Canadian Dollar
    CAD: ExchangeRateResponseDataInfo,
    /// Russian Ruble
    RUB: ExchangeRateResponseDataInfo,
    /// YEN
    JPY: ExchangeRateResponseDataInfo,
    /// Austrialian Dollar
    AUD: ExchangeRateResponseDataInfo,
    /// Armenian Dram
    AMD: ExchangeRateResponseDataInfo,
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
            "https://api.currencyapi.com/v3/latest?apikey={}&currencies=EUR%2CUSD%2CCAD%2CRUB%2CJPY%2CAUD%2CAMD",
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
    EUR(f64),
    USD(f64),
    CAD(f64),
    RUB(f64),
    JPY(f64),
    AUD(f64),
    AMD(f64),
}

impl Currency
{
    fn to_eur(self, rates: ExchangeRates) -> Currency
    {
        Self::EUR(f64::from(self.to_usd(rates.clone())) * rates.data.EUR.value)
    }

    fn to_usd(self, rates: ExchangeRates) -> Currency
    {
        // We convert each currency to USD by dividing it by it's exchange rate
        // based on USD
        match self
        {
            Self::EUR(x) => Currency::USD(rates.data.EUR.value / x),
            Self::USD(_) => self,
            Self::CAD(x) => Currency::USD(rates.data.CAD.value / x),
            Self::RUB(x) => Currency::USD(rates.data.RUB.value / x),
            Self::JPY(x) => Currency::USD(rates.data.JPY.value / x),
            Self::AUD(x) => Currency::USD(rates.data.AUD.value / x),
            Self::AMD(x) => Currency::USD(rates.data.AUD.value / x),
        }
    }

    fn to_cad(self, rates: ExchangeRates) -> Currency
    {
        Self::CAD(f64::from(self.to_usd(rates.clone())) * rates.data.CAD.value)
    }

    fn to_rub(self, rates: ExchangeRates) -> Currency
    {
        Self::RUB(f64::from(self.to_usd(rates.clone())) * rates.data.RUB.value)
    }
    fn to_jpy(self, rates: ExchangeRates) -> Currency
    {
        Self::JPY(f64::from(self.to_usd(rates.clone())) * rates.data.JPY.value)
    }
    fn to_aud(self, rates: ExchangeRates) -> Currency
    {
        Self::AUD(f64::from(self.to_usd(rates.clone())) * rates.data.AUD.value)
    }
    fn to_amd(self, rates: ExchangeRates) -> Currency
    {
        Self::AMD(f64::from(self.to_usd(rates.clone())) * rates.data.AMD.value)
    }
}

// Allow easy converting to f64
impl From<Currency> for f64
{
    fn from(item: Currency) -> Self
    {
        match item
        {
            Currency::EUR(x) => x,
            Currency::USD(x) => x,
            Currency::CAD(x) => x,
            Currency::RUB(x) => x,
            Currency::JPY(x) => x,
            Currency::AUD(x) => x,
            Currency::AMD(x) => x,
        }
    }
}

fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str
{
    if s.ends_with(p)
    {
        &s[..s.len() - p.len()]
    }
    else
    {
        s
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
        input = remove_suffix(&input, "usd")
            .strip_prefix('$')
            .unwrap_or(&input)
            .to_string();
        input_type = "USD";
    }
    else if input.starts_with('€') || input.ends_with("eur") || input.ends_with("euro")
    {
        input = remove_suffix(remove_suffix(&input, "eur"), "euro")
            .strip_prefix('€')
            .unwrap_or(&input)
            .to_string();
        input_type = "EUR";
        println!("{input}");
    }
    else if input.ends_with("cad")
    {
        input = remove_suffix(&input, "cad").to_string();
        input_type = "CAD";
    }
    else if input.ends_with("rub") || input.ends_with("ruble") || input.ends_with("rubles")
    {
        input = remove_suffix(
            remove_suffix(remove_suffix(&input, "rub"), "ruble"),
            "rubles",
        )
        .to_string();
        input_type = "RUB";
    }
    else if input.ends_with("jpy") || input.ends_with("yen")
    {
        input = remove_suffix(remove_suffix(&input, "jpy"), "yen")
            .to_string();

        input_type = "JPY";
    }
    else if input.ends_with("aud")
    {
        input = remove_suffix(&input, "aud").to_string();
        input_type = "AUD";
    }
    else if input.ends_with("amd") || input.ends_with("dram")
    {
        input = remove_suffix(remove_suffix(&input, "amd"), "dram")
            .to_string();

        input_type = "AMD";
    }
    else
    {
        return "Error: Invalid input currency".to_string();
    }

    // Try to parse the currency
    let parsed = match input.parse()
    {
        Ok(x) => x,
        Err(x) => return format!("Error: {}: '{}'", x, input),
    };

    // Store everything in the appropriate `Currency` variant
    let input_currency = match input_type
    {
        "EUR" => Currency::EUR(parsed),
        "USD" => Currency::USD(parsed),
        "CAD" => Currency::CAD(parsed),
        "RUB" => Currency::RUB(parsed),
        "JPY" => Currency::JPY(parsed),
        "AUD" => Currency::AUD(parsed),
        "AMD" => Currency::AMD(parsed),
        &_ => return format!("Error: Invalid input currency type: '{input_type}'"),
    };

    // Convert each currency based upon the target currency
    let result = match &*target
    {
        "eur" | "euro" => input_currency.to_eur(rates),
        "usd" | "dollar" => input_currency.to_usd(rates),
        "cad" => input_currency.to_cad(rates),
        "rub" | "ruble" => input_currency.to_rub(rates),
        "jpy" | "yen" => input_currency.to_jpy(rates),
        "aud" => input_currency.to_aud(rates),
        "amd" | "dram" => input_currency.to_amd(rates),
        _ => return format!("Error: Invalid target: '{target}'"),
    };

    // Return the formatted result
    format!("{:.2}{}", f64::from(result), target)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("currency")
        .description(
            "Convert from one Currency to another. Supports USD, CAD, EUR, YEN, RUB, AUD, and AMD",
        )
        .create_option(|option| {
            option
                .name("value")
                .description("Original value (e.g. '$45' [USD], '18.33 AMD' [Armenian Dram].")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("target")
                .description("The currency to target. (e.g 'EUR' [Euro], 'CAD' [Canadian Dollar]).")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
