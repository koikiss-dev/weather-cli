extern crate call_api;
use call_api::wheater_calls::{ResultRequest, WheaterApi};
use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use std::error::Error;
use tabled::builder::Builder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Buscar datos del clima por el nombre de la ciudad
    City(SearchByCity),
    /// Buscar datos del clima por coordenadas (lat y lon)
    Coords(SearchByCoords),
}

#[derive(Args)]
struct SearchByCity {
    /// Nombre de la ciudad, si va con espacios escribirla con doble comilla -> ""
    #[arg(short, long)]
    name: String,
}

#[derive(Args)]
#[command(allow_hyphen_values = true)]
struct SearchByCoords {
    /// Coordenada -> latitud -> valor flotante
    #[arg(long)]
    lat: f64,

    /// Coordenada -> longitud -> valor flotante
    #[arg(long)]
    lon: f64,
}
#[tokio::main]
async fn main() {
    dotenv().ok(); //load the enviroment variables
    let cli = Cli::parse();
    let api_key = std::env::var("appid").expect("api token for wheather consults");
    let mut wheater_functions = WheaterApi::new(api_key);

    match cli.cmd {
        Commands::City(name) => {
            let result_from_request = wheater_functions.search_by_city(&name.name).await;
            handle_response(result_from_request).await
        }

        Commands::Coords(coords) => {
            let result_from_request = wheater_functions
                .search_by_coords(coords.lat, coords.lon)
                .await;

            handle_response(result_from_request).await
        }
    }
}

/* esto se puede gracias a que la api de weatherapi
regresa los mismos valores para las consultas de clima*/
async fn handle_response(result: Result<ResultRequest, Box<dyn Error>>) {
    let mut builder = Builder::default();
    builder.push_record([
        "Name",
        "Code",
        "Latitude",
        "Longitude",
        "Clouds",
        "Temperature",
        "Pressure",
        "Humidity",
    ]);
    match result {
        Ok(n) => {
            builder.push_record([
                n.name.to_string(),
                n.cod.to_string(),
                n.coord.lat.to_string(),
                n.coord.lon.to_string(),
                n.clouds.all.to_string(),
                format!(
                    "Normal: {} \nMin: {} \nMax: {}",
                    n.main.temp.to_string(),
                    n.main.temp_min.to_string(),
                    n.main.temp_max.to_string()
                ),
                n.main.pressure.to_string(),
                n.main.humidity.to_string(),
            ]);
            println!("{}", builder.build());
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
