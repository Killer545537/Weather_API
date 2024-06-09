use std::fmt::{Display, Formatter};
use std::io;
use serde::Deserialize;
use colored::*;
use regex::Regex;

///This is to deserialize the JSON response
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    //The fields are keys of the JSON response
    weather: Vec<Weather>, //This is a vector of just one element (Necessary since they return in this format)
    main: Main, //Temperature, Humidity, Pressure
    wind: Wind, //Wind Speed
    name: String, //Name of the Place
}

#[derive(Deserialize, Debug)]
struct Weather {
    main: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    humidity: f32,
    pressure: f32,
}

///These are all the possible Weather Conditions
enum WeatherMain {
    Thunderstorm,
    Drizzle,
    Rain,
    Snow,
    Mist,
    Smoke,
    Haze,
    Dust,
    Fog,
    Sand,
    Ash,
    Squall,
    Tornado,
    Clear,
    Clouds,
}

impl Display for WeatherMain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherMain::Thunderstorm => write!(f, "⛈️"),
            WeatherMain::Drizzle => write!(f, "🌧️"),
            WeatherMain::Rain => write!(f, "☔"),
            WeatherMain::Snow => write!(f, "❄️"),
            WeatherMain::Mist => write!(f, "🌫️"),
            WeatherMain::Smoke => write!(f, "💨"),
            WeatherMain::Haze => write!(f, "🌫️"),
            WeatherMain::Dust => write!(f, "💨"),
            WeatherMain::Fog => write!(f, "🌫️"),
            WeatherMain::Sand => write!(f, "💨"),
            WeatherMain::Ash => write!(f, "💨"),
            WeatherMain::Squall => write!(f, "🌪️"),
            WeatherMain::Tornado => write!(f, "🌪️"),
            WeatherMain::Clear => write!(f, "☀️"),
            WeatherMain::Clouds => write!(f, "☁️"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_weather(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json()?;
    Ok(response_json)
}

enum Temperature {
    Cold,
    Cloudy,
    PartiallyCloudy,
    Sunny,
    Hot,
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Temperature::Cold => write!(f, "🥶️"),
            Temperature::Cloudy => write!(f, "🥱"),
            Temperature::PartiallyCloudy => write!(f, "😴️"),
            Temperature::Sunny => write!(f, "🤭"),
            Temperature::Hot => write!(f, "{}", "🥵️"),
        }
    }
}

impl Display for WeatherResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //Add an emoji to the temperature
        fn temp_to_emoji(temperature: f32) -> String {
            if temperature < 0.0 {
                format!("{}", Temperature::Cold)
            } else if temperature >= 0.0 && temperature < 10.0 {
                format!("{}", Temperature::Cloudy)
            } else if temperature >= 10.0 && temperature < 20.0 {
                format!("{}", Temperature::PartiallyCloudy)
            } else if temperature >= 20.0 && temperature < 30.0 {
                format!("{}", Temperature::Sunny)
            } else {
                format!("{}", Temperature::Hot)
            }
        }
        //Add an emoji to the Weather
        fn main_formatter(description: String) -> WeatherMain {
            match description.as_str() {
                "Thunderstorm" => WeatherMain::Thunderstorm,
                "Drizzle" => WeatherMain::Drizzle,
                "Rain" => WeatherMain::Rain,
                "Snow" => WeatherMain::Snow,
                "Mist" => WeatherMain::Mist,
                "Smoke" => WeatherMain::Smoke,
                "Haze" => WeatherMain::Haze,
                "Dust" => WeatherMain::Dust,
                "Fog" => WeatherMain::Fog,
                "Sand" => WeatherMain::Sand,
                "Ash" => WeatherMain::Ash,
                "Squall" => WeatherMain::Squall,
                "Tornado" => WeatherMain::Tornado,
                "Clear" => WeatherMain::Clear,
                "Clouds" => WeatherMain::Clouds,
                _ => panic!("Unknown weather description: {}", description),
            }
        }

        let description = &self.weather[0].main;
        let more_description = &self.weather[0].description;
        let description_color = main_formatter(description.to_string());

        let weather_string = format!(
            "Weather in {}: {} {}
            (More Description: {})
            > Temperature: {:.1}°C {} (but feels like {:.1}°C)
            > Humidity: {:.1}%
            > Pressure: {:.1} hPa
            > Wind Speed: {:.1} m/s"
            , self.name, description, description_color, more_description, self.main.temp, temp_to_emoji(self.main.temp), self.main.feels_like, self.main.humidity, self.main.pressure, self.wind.speed);

        //Giving color to the output
        let weather_string = match description_color {
            WeatherMain::Thunderstorm => weather_string.color("gray"),
            WeatherMain::Drizzle => weather_string.color("light blue"),
            WeatherMain::Rain => weather_string.color("blue"),
            WeatherMain::Snow => weather_string.color("white"),
            WeatherMain::Mist => weather_string.color("light gray"),
            WeatherMain::Smoke => weather_string.color("dark gray"),
            WeatherMain::Haze => weather_string.color("light gray"),
            WeatherMain::Dust => weather_string.color("brown"),
            WeatherMain::Fog => weather_string.color("light gray"),
            WeatherMain::Sand => weather_string.color("yellow"),
            WeatherMain::Ash => weather_string.color("dark gray"),
            WeatherMain::Squall => weather_string.color("dark gray"),
            WeatherMain::Tornado => weather_string.color("dark gray"),
            WeatherMain::Clear => weather_string.color("yellow"),
            WeatherMain::Clouds => weather_string.color("light gray"),
        };

        write!(f, "{}", weather_string)
    }
}

fn main() -> io::Result<()> {
    println!("{}", "Welcome to Weather Station".bright_yellow());
    loop {
        println!("{}", "Enter the name of the city".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city)?;
        let city = city.trim();

        println!("{}", "Enter the country code".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code)?;
        let country_code = country_code.trim();

        const API_KEY: &str = "233ace9ec0324fa41c57314c366e0cee";

        match get_weather(city, country_code, API_KEY) {
            Ok(response) => {
                println!("{}", response);
            }
            Err(err) => { eprintln!("Error: {err}"); }
        }

        println!("{}", "Do you want to search more?".bright_green());
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let re = Regex::new(r"(?i)(yes|ok)").unwrap();

        if !re.is_match(input) {
            break;
        }
    }

    Ok(())
}