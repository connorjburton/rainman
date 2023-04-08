use reqwest::blocking::get;
use serde::Deserialize;
use chrono::prelude::*;

#[derive(Debug, Deserialize)]
struct Hourly {
    weathercode: [u8; 24]
}

#[derive(Debug, Deserialize)]
struct Weather {
    hourly: Hourly
}

// https://open-meteo.com/en/docs
// WMO Weather intepretation codes
// anything about 51 is raining
const RAINY_WEATHER_CODE: u8 = 51;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = get("https://api.open-meteo.com/v1/forecast?latitude=51.51&longitude=-0.13&hourly=rain,weathercode&daily=weathercode&current_weather=true&forecast_days=1&timezone=Europe%2FLondon")?;
    let body = resp.text()?;
    let weather: Weather = serde_json::from_str(&body)?;
    let now = Local::now();
    let current_hour = now.hour() as usize;

    let hours_from_now = &weather.hourly.weathercode[current_hour..];
    for (i, weathercode) in hours_from_now.iter().enumerate() {
        if weathercode >= &RAINY_WEATHER_CODE {
            println!("It's going to rain at {}:00", current_hour + i);
            // send to discord, etc
        }
    }
    Ok(())
}
