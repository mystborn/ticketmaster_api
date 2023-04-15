//! A simple wrapper aroung the [Ticketmaster API](https://developer.ticketmaster.com/products-and-docs/apis/discovery-api/v2/).
//! Currently only supports the Discover API.
//! 
//! # Quick Start Example
//! 
//! ```rs
//! 
//! #[tokio::main]
//! async fn main() {
//!     let api_key = "xxx";
//! 
//!     // Create the Ticketmaster client
//!     let client = ticketmaster_api::TicketMasterDiscoverClient::new(api_key);
//! 
//!     // Search for events near Houston, TX, USA
//!     let mut query = EventSearchQuery::default();
//!     query.set_city(Some(vec!["Houston".to_owned()]));
//! 
//!     // Search for 5 events per page.
//!     query.set_size(5);
//! 
//!     // Make API request
//!     match client.event_search(&query) {
//!         Ok(events) => {
//!             println!("Successfully searched for events:");
//!             println!("{:?}", events");
//!         },
//!         Err(err) => {
//!             println!("Error searching for events:");
//!             println!("{:?}", err)
//!         }
//!     }
//!     let events = client.event_search(&query);
//! }
//! ```

pub mod discovery;
pub mod partner;