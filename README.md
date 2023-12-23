# geonames-rs

Rust client library for [geonames.org](https://www.geonames.org/) API.

## Usage

```rust
use geonames::ApiClient;
use geonames::ApiEndpoint;
use geonames::GeoNamesApi;
use geonames::OceanResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() {
    let username = "YOUR_GEONAMES_USERNAME";
    let client = ApiClient::new(GeoNamesApi::Ocean, username, None);

    let mut params = HashMap::new();
    params.insert("lat", "40.7834");
    params.insert("lng", "-43.96635");

    let result: OceanResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    println!("{:?}", result);
}
```

## Configuration

Use the new function to create a new client:

```rust
let api = GeoNamesApi::Ocean;
let username = "YOUR_GEONAMES_USERNAME";
let token = Some("YOUR_COMMERCIAL_TOKEN");

let client = ApiClient::new(api, username, token);
```

## Testing

```
API_USER=your_username cargo test
```

## Features

An overview of the overall Web Services exposed from [geonames.org](https://www.geonames.org/) can be found here.
</br>
The library supports the following web services in JSON format:

- astergdem
- children
- cities
- contains
- countryCode
- countryInfo
- countrySubdivision
- earthquakes
- findNearby
- findNearbyPlaceName
- findNearbyPostalCodes
- findNearbyStreetsOSM
- findNearByWeather
- findNearbyWikipedia
- findNearestIntersectionOsm
- findNearbyPOIsOSM
- address
- geoCodeAddress
- streetNameLookup
- get
- gtopo30
- hierarchy
- neighbours
- ocean
- postalCodeCountryInfo
- postalCodeLookup
- postalCodeSearch
- search
- siblings
- srtm1
- srtm3
- timezone
- weather
- weatherIcao
- wikipediaBoundingBox
- wikipediaSearch

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
