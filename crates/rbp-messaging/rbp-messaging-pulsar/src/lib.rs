use std::collections::HashMap;

use pulsar::{Executor, Pulsar, Producer};

pub mod error;

pub struct PulsarStreamChannel<Exe: Executor> {
    pub client: Pulsar<Exe>,
    pub producers: HashMap<String, Producer<Exe>>
}


