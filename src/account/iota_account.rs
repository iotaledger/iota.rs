use super::structs::ConditionalDepositAddress;
use super::traits::{Account, Plugin};
use crate::Result;
use iota_model::Bundle;

#[derive(Clone, Default)]
pub struct IotaAccount<T: Account>{
    pub id: String,
    pub plugins: Vec<Box<dyn Plugin>>
}

impl Account for IotaAccount {
    fn id(&self) -> String {
        unimplemented!();
    }

    fn load(&self) -> Result<()>{
        unimplemented!();
    }

    fn start(&self) -> Result<()>{
        unimplemented!();
    }

    fn shutdown(&self) -> Result<()>{
        unimplemented!();
    }

    fn send(&self) -> Result<Bundle>{
        unimplemented!();
    }

    fn new_deposit_address(&self) -> Result<ConditionalDepositAddress>{
        unimplemented!();
    }

    fn usable_balance(&self) -> Result<u64>{
        unimplemented!();
    }

    fn total_balance(&self) -> Result<u64>{
        unimplemented!();
    }

    fn is_new(&self) -> bool{
        unimplemented!();
    }

    fn update_settings(&self) -> Result<()>{
        unimplemented!();
    }
}