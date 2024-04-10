extern crate call_api;
use call_api::wheater_calls::WheaterApi;
use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;

// use reqwest;

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
    city: String,
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
        Commands::City(city) => {
            let result_from_request = wheater_functions.search_by_city(&city.city).await;
            println!("{:?}", result_from_request);
        }

        Commands::Coords(coords) => {
            let result_from_request = wheater_functions
                .search_by_coords(coords.lat, coords.lon)
                .await;
            println!("{:?}", result_from_request);
        }
    }
}
