use influxdb::ReadQuery;
use mqtt2influx_core::utils::{generate_random_number, generate_random_token};
use mqtt2influx_core::{Event, EventSink, InfluxDbConnectionParameters, InfluxDbCredentials, InfluxDbSink};

lazy_static::lazy_static! {
    static ref INFLUX_URL: String = {
        dotenv::dotenv().ok();
        std::env::var("TEST_INFLUX_URL").unwrap_or_else(|_| "http://127.0.0.1:8086".to_string())
    };

   static ref INFLUX_DB: String = {
       dotenv::dotenv().ok();
       std::env::var("TEST_INFLUX_DB").unwrap_or_else(|_| "test_db".to_string())
   };

   static ref INFLUX_USER: String = {
       dotenv::dotenv().ok();
       std::env::var("TEST_INFLUX_USER").unwrap_or_else(|_| "user".to_string())
   };

   static ref INFLUX_PASSWORD: String = {
       dotenv::dotenv().ok();
       std::env::var("TEST_INFLUX_PASSWORD").unwrap_or_else(|_| "password".to_string())
   };
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct InfluxResults {
    results: Vec<InfluxResult>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct InfluxResult {
    series: Vec<InfluxSerie>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct InfluxSerie {
    name: String,
    values: Vec<Vec<serde_json::Value>>,
}

#[tokio::test]
async fn sink_sends_event() {
    let sink = InfluxDbSink::new(InfluxDbConnectionParameters {
        server: &INFLUX_URL,
        db: &INFLUX_DB,
        credentials: Some(InfluxDbCredentials {
            username: &INFLUX_USER,
            password: &INFLUX_PASSWORD,
        }),
    })
    .await
    .expect("Error creating sink");

    let name = generate_random_token(10);

    let event = Event {
        device_name: name.clone(),
        battery: generate_random_number(1, 100) as u8,
        humidity: generate_random_number(1, 100) as f32,
        temperature: generate_random_number(1, 100) as f32,
        voltage: generate_random_number(1, 100) as u16,
        linkquality: generate_random_number(1, 100) as u8,
    };
    sink.sink(event.clone()).await.expect("Should be able to sink");

    let client = influxdb::Client::new(INFLUX_URL.as_str(), INFLUX_DB.as_str());
    let table = mqtt2influx_core::sink::influx::READINGS_TABLE;

    let query = format!(
        "SELECT device_name,temperature FROM {table} WHERE battery={battery} AND humidity={humidity} AND temperature={temperature};",
        table = table,
        battery = event.battery,
        humidity = event.humidity,
        temperature = event.temperature
    );
    let q = ReadQuery::new(query);

    let res = client.query(&q).await.expect("Should be able to query");
    let results: InfluxResults = serde_json::from_str(&res).expect("Error parsing influx response");
    assert_eq!(results.results.len(), 1, "Should return 1 result");

    let series = &results.results[0].series;
    assert_eq!(series.len(), 1, "Should return 1 series");

    let serie = &series[0];
    assert_eq!(serie.name, table, "Series should match");

    let values = &serie.values;
    assert_eq!(values.len(), 1, "Should contain only 1 value");

    let value = &values[0];
    assert_eq!(value.len(), 3, "Value should contain 2 entries");

    let device_name_value = &value[1];
    match device_name_value {
        serde_json::Value::String(device_name) => {
            assert_eq!(device_name, &name, "Name should match");
        }
        _ => panic!("Value should be a String"),
    }
}
