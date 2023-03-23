mod service;
use abi::{FilterResponse, Reservation};
use futures::Stream;
use reservation::ReservationManager;
use std::pin::Pin;
use tonic::Status;

pub struct RsvpService {
    manager: ReservationManager,
}

type ReservationStream = Pin<Box<dyn Stream<Item = Result<Reservation, Status>> + Send + 'static>>;
type FilterResponseStream =
    Pin<Box<dyn Stream<Item = Result<FilterResponse, Status>> + Send + 'static>>;
