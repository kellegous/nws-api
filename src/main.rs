use clap::Parser;
use std::error::Error;
use weather_api::nws::{Client, Location};

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long, default_value_t=String::from("https://gihtub.com/kellegous/nws-api"))]
    auth: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let location = Location::new(34.408433, -77.57307);
    let client = Client::new(&args.auth);
    let grid = client.get_grid(&location)?;
    let stations = grid.get_stations()?;

    println!("{}", location);
    for station in stations {
        println!(
            "{} {:0.1} km",
            station.id(),
            Location::distance_between(&location, station.location()).in_kilometers()
        );
    }

    Ok(())
}
