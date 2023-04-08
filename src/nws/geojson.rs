use super::{Client, Grid, Location, Station};

pub mod grid {
    use super::{Client, Grid};
    use serde_derive::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Response {
        properties: Properties,
    }

    impl Response {
        pub fn to_grid(&self, client: &Client) -> Grid {
            Grid {
                client: client.clone(),
                grid_id: self.properties.grid_id.clone(),
                grid_x: self.properties.grid_x,
                grid_y: self.properties.grid_y,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Properties {
        #[serde(rename = "gridId")]
        grid_id: String,
        #[serde(rename = "gridX")]
        grid_x: i64,
        #[serde(rename = "gridY")]
        grid_y: i64,
    }
}

pub mod stations {
    use serde_derive::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Point {
        coordinates: (f64, f64),
    }

    impl Point {
        fn to_location(&self) -> super::Location {
            let (lng, lat) = self.coordinates;
            super::Location::new(lat, lng)
        }
    }

    #[derive(Debug, Deserialize)]
    struct Properties {
        #[serde(rename = "stationIdentifier")]
        id: String,
        name: String,
        elevation: Elevation,
    }

    #[derive(Debug, Deserialize)]
    struct Elevation {
        value: f64,
        #[serde(rename = "unitCode")]
        unit: String,
    }

    #[derive(Debug, Deserialize)]
    struct Station {
        geometry: Point,
        properties: Properties,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        features: Vec<Station>,
    }

    impl Response {
        pub fn to_stations(&self) -> Vec<super::Station> {
            self.features
                .iter()
                .map(|s| {
                    super::Station::new(
                        s.properties.id.clone(),
                        s.properties.name.clone(),
                        s.properties.elevation.value,
                        s.geometry.to_location(),
                    )
                })
                .collect()
        }
    }
}
