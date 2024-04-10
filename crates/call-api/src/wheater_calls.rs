///! Manage request from openweather api
///! Author: yako(https://github.com/koikiss-dev)
use core::panic;
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// whater api class for requests functions
pub struct WheaterApi {
    client: Client,
    url_main: String,
    api_key_query: Vec<(String, String)>,
}
pub struct Example {
    id: i32,
}

/// esta es la estructura de respuesta

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultRequest {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    pub id: i64,
    pub country: String,
}

impl WheaterApi {
    pub fn new(api_key: String) -> WheaterApi {
        WheaterApi {
            client: Client::new(),
            url_main: String::from("https://api.openweathermap.org/data/2.5/weather"),
            api_key_query: vec![(String::from("appid"), api_key.clone())],
        }
    }

    //consultar por nombre de ciudad

    pub async fn search_by_city(
        &mut self,
        city_name: &str,
    ) -> Result<ResultRequest, Box<dyn Error>> {
        let mut query = vec![("q".to_string(), city_name.to_string())];
        query.extend(self.api_key_query.clone());

        let request = self
            .client
            .get(self.url_main.to_string())
            .query(&query)
            .send()
            .await?;
        let message_error = format!("{}{}", "Error en: ", city_name);
        self.check_request(request, &message_error).await
    }

    //consultar por coordenadas
    pub async fn search_by_coords(
        &mut self,
        lat: f64,
        lon: f64,
    ) -> Result<ResultRequest, Box<dyn Error>> {
        let mut query = vec![
            ("lat".to_string(), lat.to_string()),
            ("lon".to_string(), lon.to_string()),
        ];
        query.extend(self.api_key_query.clone());

        //request
        let request = self
            .client
            .get(self.url_main.to_string())
            .query(&query)
            .send()
            .await?;

        let message_error = format!("{}{}{}", "Error en: ", lat, lon);
        self.check_request(request, &message_error).await
        //todo!()
    }

    // para confirmar si la funcion fue consultada con exito
    async fn check_request(
        &mut self,
        request: Response,
        error_message: &str,
    ) -> Result<ResultRequest, Box<dyn Error>> {
        match request.status() {
            StatusCode::OK => {
                let response = request.json::<ResultRequest>().await?;
                Ok(response)
            }
            _ => {
                panic!("{:?}", error_message)
            }
        }
    }
}
