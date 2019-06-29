use crate::account::traits::{Account, Plugin};
use crate::account::iota_account::IotaAccount;

#[derive(Clone, Default)]
pub struct AccountPlugin<T: Account> {
    pub name: String,
    pub account: T
}

impl <T: Account> Plugin<T> for AccountPlugin<T> {
    fn account(&self) -> &T {
        &self.account
    }
    fn name(&self) -> &str {
        &self.name
    }
}