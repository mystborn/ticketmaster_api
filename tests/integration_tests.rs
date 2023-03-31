use ticketmaster::discovery::{TicketmasterDiscoveryClient, EventSearchQuery};

#[tokio::test]
async fn it_gets_events_from_houston_returns_json() {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = TicketmasterDiscoveryClient::new(api_key);

    let mut query = EventSearchQuery::default();
    query.set_city(Some(vec!["Houston".to_owned()]));
    query.set_size(Some(1));

    let events = client.event_search_as_json(&query).await;

    assert!(events.is_ok())

    // println!("{:?}", events);
}

#[tokio::test]
async fn it_gets_events_from_houston_returns_events() {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = TicketmasterDiscoveryClient::new(api_key);

    let mut query = EventSearchQuery::default();
    query.set_city(Some(vec!["Houston".to_owned()]));
    query.set_size(Some(1));

    let events = client.event_search(&query).await;

    println!("{:#?}", events);
}

#[test]
fn it_serializes_query_defaults() {
    let client = TicketmasterDiscoveryClient::new("".to_string());
    let mut query = EventSearchQuery::default();

    query.set_id(Some("mystborn".to_owned()));

    let json = client.test_query_serialize(&query).unwrap();

    assert_eq!("id=mystborn", json);
}

#[test]
fn it_append_api_key() {
    let mut url = "https://app.ticketmaster.com/discovery/v2/test=true".to_owned();

    let client = TicketmasterDiscoveryClient::new("6978".to_string());

    client.append_api_key(&mut url);

    assert_eq!(url, "https://app.ticketmaster.com/discovery/v2/test=true&apikey=6978");
}

#[test]
fn it_append_api_key_slash() {
    let mut url = "https://app.ticketmaster.com/discovery/v2/".to_owned();

    let client = TicketmasterDiscoveryClient::new("6978".to_string());

    client.append_api_key(&mut url);

    assert_eq!(url, "https://app.ticketmaster.com/discovery/v2/apikey=6978");
}

#[test]
fn it_append_api_key_question() {
    let mut url = "https://app.ticketmaster.com/discovery/v2/events.json?".to_owned();

    let client = TicketmasterDiscoveryClient::new("6978".to_string());

    client.append_api_key(&mut url);

    assert_eq!(url, "https://app.ticketmaster.com/discovery/v2/events.json?apikey=6978");
}

#[test]
fn it_append_api_key_and() {
    let mut url = "https://app.ticketmaster.com/discovery/v2/test=true&".to_owned();

    let client = TicketmasterDiscoveryClient::new("6978".to_string());

    client.append_api_key(&mut url);

    assert_eq!(url, "https://app.ticketmaster.com/discovery/v2/test=true&apikey=6978");
}