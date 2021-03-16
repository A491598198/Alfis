use crate::{Chain, Bus, Keystore, Settings, ExternalZones};
use crate::event::Event;
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

pub struct Context {
    pub settings: Settings,
    pub keystore: Keystore,
    pub chain: Chain,
    pub x_zones: ExternalZones,
    pub bus: Bus<Event>,
}

impl Context {
    /// Creating an essential context to work with
    pub fn new(settings: Settings, keystore: Keystore, chain: Chain) -> Context {
        Context { settings, keystore, chain, x_zones: ExternalZones::new(), bus: Bus::new() }
    }

    /// Load keystore and return Context
    pub fn load_keystore<S: Into<String>>(mut self, name: S, password: S) -> Context {
        let filename = &name.into();
        match Keystore::from_file(filename, &password.into()) {
            None => {
                warn!("Error loading keystore '{}'!", filename);
            },
            Some(keystore) => {
                self.keystore = keystore;
            },
        }
        self
    }

    pub fn get_keystore(&self) -> Keystore {
        self.keystore.clone()
    }

    pub fn set_keystore(&mut self, keystore: Keystore) {
        self.keystore = keystore;
    }

    pub fn get_chain(&self) -> &Chain {
        &self.chain
    }
}