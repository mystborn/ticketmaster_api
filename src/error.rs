use std::{fmt, error::Error};

#[derive(Debug)]
pub struct TicketmasterError {
    message: Option<String>
}

impl fmt::Display for TicketmasterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{}", msg),
            None => write!(f, "Encountered an error with the ticketmaster api")
        }
    }
}

impl Error for TicketmasterError {}

impl TicketmasterError {
    pub fn new(message: Option<String>) -> TicketmasterError {
        TicketmasterError {
            message
        }
    }
}