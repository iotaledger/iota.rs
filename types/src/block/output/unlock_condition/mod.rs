// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod address;
mod expiration;
mod governor_address;
mod immutable_alias_address;
mod state_controller_address;
mod storage_deposit_return;
mod timelock;

use alloc::{boxed::Box, vec::Vec};

use bitflags::bitflags;
use derive_more::{Deref, From};
use iterator_sorted::is_unique_sorted;
use packable::{
    bounded::BoundedU8,
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::BoxedSlicePrefix,
    unpacker::Unpacker,
    Packable,
};

pub use self::{
    address::AddressUnlockCondition, expiration::ExpirationUnlockCondition,
    governor_address::GovernorAddressUnlockCondition, immutable_alias_address::ImmutableAliasAddressUnlockCondition,
    state_controller_address::StateControllerAddressUnlockCondition,
    storage_deposit_return::StorageDepositReturnUnlockCondition, timelock::TimelockUnlockCondition,
};
use crate::{
    block::{address::Address, protocol::ProtocolParameters, Error},
    create_bitflags,
};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "data")
)]
pub enum UnlockCondition {
    /// An address unlock condition.
    Address(AddressUnlockCondition),
    /// A storage deposit return unlock condition.
    StorageDepositReturn(StorageDepositReturnUnlockCondition),
    /// A timelock unlock condition.
    Timelock(TimelockUnlockCondition),
    /// An expiration unlock condition.
    Expiration(ExpirationUnlockCondition),
    /// A state controller address unlock condition.
    StateControllerAddress(StateControllerAddressUnlockCondition),
    /// A governor address unlock condition.
    GovernorAddress(GovernorAddressUnlockCondition),
    /// An immutable alias address unlock condition.
    ImmutableAliasAddress(ImmutableAliasAddressUnlockCondition),
}

impl UnlockCondition {
    /// Return the output kind of an `Output`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Address(_) => AddressUnlockCondition::KIND,
            Self::StorageDepositReturn(_) => StorageDepositReturnUnlockCondition::KIND,
            Self::Timelock(_) => TimelockUnlockCondition::KIND,
            Self::Expiration(_) => ExpirationUnlockCondition::KIND,
            Self::StateControllerAddress(_) => StateControllerAddressUnlockCondition::KIND,
            Self::GovernorAddress(_) => GovernorAddressUnlockCondition::KIND,
            Self::ImmutableAliasAddress(_) => ImmutableAliasAddressUnlockCondition::KIND,
        }
    }

    /// Returns the [`UnlockConditionFlags`] for the given [`UnlockCondition`].
    pub(crate) fn flag(&self) -> UnlockConditionFlags {
        match self {
            Self::Address(_) => UnlockConditionFlags::ADDRESS,
            Self::StorageDepositReturn(_) => UnlockConditionFlags::STORAGE_DEPOSIT_RETURN,
            Self::Timelock(_) => UnlockConditionFlags::TIMELOCK,
            Self::Expiration(_) => UnlockConditionFlags::EXPIRATION,
            Self::StateControllerAddress(_) => UnlockConditionFlags::STATE_CONTROLLER_ADDRESS,
            Self::GovernorAddress(_) => UnlockConditionFlags::GOVERNOR_ADDRESS,
            Self::ImmutableAliasAddress(_) => UnlockConditionFlags::IMMUTABLE_ALIAS_ADDRESS,
        }
    }
}

create_bitflags!(
    /// A bitflags-based representation of the set of active [`UnlockCondition`]s.
    pub UnlockConditionFlags,
    u16,
    [
        (ADDRESS, AddressUnlockCondition),
        (STORAGE_DEPOSIT_RETURN, StorageDepositReturnUnlockCondition),
        (TIMELOCK, TimelockUnlockCondition),
        (EXPIRATION, ExpirationUnlockCondition),
        (STATE_CONTROLLER_ADDRESS, StateControllerAddressUnlockCondition),
        (GOVERNOR_ADDRESS, GovernorAddressUnlockCondition),
        (IMMUTABLE_ALIAS_ADDRESS, ImmutableAliasAddressUnlockCondition),
    ]
);

impl Packable for UnlockCondition {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            Self::Address(unlock_condition) => {
                AddressUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::StorageDepositReturn(unlock_condition) => {
                StorageDepositReturnUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::Timelock(unlock_condition) => {
                TimelockUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::Expiration(unlock_condition) => {
                ExpirationUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::StateControllerAddress(unlock_condition) => {
                StateControllerAddressUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::GovernorAddress(unlock_condition) => {
                GovernorAddressUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::ImmutableAliasAddress(unlock_condition) => {
                ImmutableAliasAddressUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
        }?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            AddressUnlockCondition::KIND => {
                Self::from(AddressUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            StorageDepositReturnUnlockCondition::KIND => {
                Self::from(StorageDepositReturnUnlockCondition::unpack::<_, VERIFY>(unpacker, visitor).coerce()?)
            }
            TimelockUnlockCondition::KIND => {
                Self::from(TimelockUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            ExpirationUnlockCondition::KIND => {
                Self::from(ExpirationUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            StateControllerAddressUnlockCondition::KIND => {
                Self::from(StateControllerAddressUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            GovernorAddressUnlockCondition::KIND => {
                Self::from(GovernorAddressUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            ImmutableAliasAddressUnlockCondition::KIND => {
                Self::from(ImmutableAliasAddressUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            k => return Err(Error::InvalidOutputKind(k)).map_err(UnpackError::Packable),
        })
    }
}

pub(crate) type UnlockConditionCount = BoundedU8<0, { UnlockConditions::COUNT_MAX }>;

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidUnlockConditionCount(p.into())))]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct UnlockConditions(
    #[packable(verify_with = verify_unique_sorted_packable)] BoxedSlicePrefix<UnlockCondition, UnlockConditionCount>,
);

impl TryFrom<Vec<UnlockCondition>> for UnlockConditions {
    type Error = Error;

    #[inline(always)]
    fn try_from(unlock_conditions: Vec<UnlockCondition>) -> Result<Self, Self::Error> {
        Self::new(unlock_conditions)
    }
}

impl IntoIterator for UnlockConditions {
    type Item = UnlockCondition;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[UnlockCondition]>>::into(self.0)).into_iter()
    }
}

impl UnlockConditions {
    ///
    pub const COUNT_MAX: u8 = 7;

    /// Creates a new [`UnlockConditions`].
    pub fn new(unlock_conditions: Vec<UnlockCondition>) -> Result<Self, Error> {
        let mut unlock_conditions =
            BoxedSlicePrefix::<UnlockCondition, UnlockConditionCount>::try_from(unlock_conditions.into_boxed_slice())
                .map_err(Error::InvalidUnlockConditionCount)?;

        unlock_conditions.sort_by_key(UnlockCondition::kind);
        // Sort is obviously fine now but uniqueness still needs to be checked.
        verify_unique_sorted::<true>(&unlock_conditions)?;

        Ok(Self(unlock_conditions))
    }

    /// Gets a reference to an [`UnlockCondition`] from an unlock condition kind, if any.
    #[inline(always)]
    pub fn get(&self, key: u8) -> Option<&UnlockCondition> {
        self.0
            .binary_search_by_key(&key, UnlockCondition::kind)
            // PANIC: indexation is fine since the index has been found.
            .map(|index| &self.0[index])
            .ok()
    }

    /// Gets a reference to an [`AddressUnlockCondition`], if any.
    #[inline(always)]
    pub fn address(&self) -> Option<&AddressUnlockCondition> {
        if let Some(UnlockCondition::Address(address)) = self.get(AddressUnlockCondition::KIND) {
            Some(address)
        } else {
            None
        }
    }

    /// Gets a reference to a [`StorageDepositReturnUnlockCondition`], if any.
    #[inline(always)]
    pub fn storage_deposit_return(&self) -> Option<&StorageDepositReturnUnlockCondition> {
        if let Some(UnlockCondition::StorageDepositReturn(storage_deposit_return)) =
            self.get(StorageDepositReturnUnlockCondition::KIND)
        {
            Some(storage_deposit_return)
        } else {
            None
        }
    }

    /// Gets a reference to a [`TimelockUnlockCondition`], if any.
    #[inline(always)]
    pub fn timelock(&self) -> Option<&TimelockUnlockCondition> {
        if let Some(UnlockCondition::Timelock(timelock)) = self.get(TimelockUnlockCondition::KIND) {
            Some(timelock)
        } else {
            None
        }
    }

    /// Gets a reference to an [`ExpirationUnlockCondition`], if any.
    #[inline(always)]
    pub fn expiration(&self) -> Option<&ExpirationUnlockCondition> {
        if let Some(UnlockCondition::Expiration(expiration)) = self.get(ExpirationUnlockCondition::KIND) {
            Some(expiration)
        } else {
            None
        }
    }

    /// Gets a reference to a [`StateControllerAddressUnlockCondition`], if any.
    #[inline(always)]
    pub fn state_controller_address(&self) -> Option<&StateControllerAddressUnlockCondition> {
        if let Some(UnlockCondition::StateControllerAddress(state_controller_address)) =
            self.get(StateControllerAddressUnlockCondition::KIND)
        {
            Some(state_controller_address)
        } else {
            None
        }
    }

    /// Gets a reference to a [`GovernorAddressUnlockCondition`], if any.
    #[inline(always)]
    pub fn governor_address(&self) -> Option<&GovernorAddressUnlockCondition> {
        if let Some(UnlockCondition::GovernorAddress(governor_address)) = self.get(GovernorAddressUnlockCondition::KIND)
        {
            Some(governor_address)
        } else {
            None
        }
    }

    /// Gets a reference to an [`ImmutableAliasAddressUnlockCondition`], if any.
    #[inline(always)]
    pub fn immutable_alias_address(&self) -> Option<&ImmutableAliasAddressUnlockCondition> {
        if let Some(UnlockCondition::ImmutableAliasAddress(immutable_alias_address)) =
            self.get(ImmutableAliasAddressUnlockCondition::KIND)
        {
            Some(immutable_alias_address)
        } else {
            None
        }
    }

    /// Returns the address to be unlocked.
    #[inline(always)]
    pub fn locked_address<'a>(&'a self, address: &'a Address, milestone_timestamp: u32) -> &'a Address {
        self.expiration()
            .and_then(|e| e.return_address_expired(milestone_timestamp))
            .unwrap_or(address)
    }

    /// Returns whether a time lock exists and is still relevant.
    #[inline(always)]
    pub fn is_time_locked(&self, milestone_timestamp: u32) -> bool {
        self.timelock()
            .map_or(false, |timelock| milestone_timestamp < timelock.timestamp())
    }

    /// Returns whether an expiration exists and is expired.
    #[inline(always)]
    pub fn is_expired(&self, milestone_timestamp: u32) -> bool {
        self.expiration()
            .map_or(false, |expiration| milestone_timestamp >= expiration.timestamp())
    }
}

#[inline]
fn verify_unique_sorted<const VERIFY: bool>(unlock_conditions: &[UnlockCondition]) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(unlock_conditions.iter().map(UnlockCondition::kind)) {
        Err(Error::UnlockConditionsNotUniqueSorted)
    } else {
        Ok(())
    }
}

#[inline]
fn verify_unique_sorted_packable<const VERIFY: bool>(
    unlock_conditions: &[UnlockCondition],
    _: &ProtocolParameters,
) -> Result<(), Error> {
    verify_unique_sorted::<VERIFY>(unlock_conditions)
}

pub(crate) fn verify_allowed_unlock_conditions(
    unlock_conditions: &UnlockConditions,
    allowed_unlock_conditions: UnlockConditionFlags,
) -> Result<(), Error> {
    for (index, unlock_condition) in unlock_conditions.iter().enumerate() {
        if !allowed_unlock_conditions.contains(unlock_condition.flag()) {
            return Err(Error::UnallowedUnlockCondition {
                index,
                kind: unlock_condition.kind(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_flags_present() {
        assert_eq!(
            UnlockConditionFlags::ALL_FLAGS,
            &[
                UnlockConditionFlags::ADDRESS,
                UnlockConditionFlags::STORAGE_DEPOSIT_RETURN,
                UnlockConditionFlags::TIMELOCK,
                UnlockConditionFlags::EXPIRATION,
                UnlockConditionFlags::STATE_CONTROLLER_ADDRESS,
                UnlockConditionFlags::GOVERNOR_ADDRESS,
                UnlockConditionFlags::IMMUTABLE_ALIAS_ADDRESS
            ]
        );
    }
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize, Serializer};
    use serde_json::Value;

    pub use self::{
        address::dto::AddressUnlockConditionDto, expiration::dto::ExpirationUnlockConditionDto,
        governor_address::dto::GovernorAddressUnlockConditionDto,
        immutable_alias_address::dto::ImmutableAliasAddressUnlockConditionDto,
        state_controller_address::dto::StateControllerAddressUnlockConditionDto,
        storage_deposit_return::dto::StorageDepositReturnUnlockConditionDto, timelock::dto::TimelockUnlockConditionDto,
    };
    use super::*;
    use crate::block::error::dto::DtoError;

    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum UnlockConditionDto {
        /// An address unlock condition.
        Address(AddressUnlockConditionDto),
        /// A storage deposit return unlock condition.
        StorageDepositReturn(StorageDepositReturnUnlockConditionDto),
        /// A timelock unlock condition.
        Timelock(TimelockUnlockConditionDto),
        /// An expiration unlock condition.
        Expiration(ExpirationUnlockConditionDto),
        /// A state controller address unlock condition.
        StateControllerAddress(StateControllerAddressUnlockConditionDto),
        /// A governor address unlock condition.
        GovernorAddress(GovernorAddressUnlockConditionDto),
        /// An immutable alias address unlock condition.
        ImmutableAliasAddress(ImmutableAliasAddressUnlockConditionDto),
    }

    impl<'de> Deserialize<'de> for UnlockConditionDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid unlock condition type"))?
                    as u8
                {
                    AddressUnlockCondition::KIND => {
                        Self::Address(AddressUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize address unlock condition: {e}"))
                        })?)
                    }
                    StorageDepositReturnUnlockCondition::KIND => Self::StorageDepositReturn(
                        StorageDepositReturnUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!(
                                "cannot deserialize storage deposit unlock condition: {e}"
                            ))
                        })?,
                    ),
                    TimelockUnlockCondition::KIND => {
                        Self::Timelock(TimelockUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize timelock unlock condition: {e}"))
                        })?)
                    }
                    ExpirationUnlockCondition::KIND => {
                        Self::Expiration(ExpirationUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize expiration unlock condition: {e}"))
                        })?)
                    }
                    StateControllerAddressUnlockCondition::KIND => Self::StateControllerAddress(
                        StateControllerAddressUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!(
                                "cannot deserialize state controller unlock condition: {e}"
                            ))
                        })?,
                    ),
                    GovernorAddressUnlockCondition::KIND => {
                        Self::GovernorAddress(GovernorAddressUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize governor unlock condition: {e}"))
                        })?)
                    }
                    ImmutableAliasAddressUnlockCondition::KIND => Self::ImmutableAliasAddress(
                        ImmutableAliasAddressUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!(
                                "cannot deserialize immutable alias address unlock condition: {e}"
                            ))
                        })?,
                    ),
                    _ => return Err(serde::de::Error::custom("invalid unlock condition type")),
                },
            )
        }
    }

    impl Serialize for UnlockConditionDto {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            #[derive(Serialize)]
            #[serde(untagged)]
            enum UnlockConditionDto_<'a> {
                T1(&'a AddressUnlockConditionDto),
                T2(&'a StorageDepositReturnUnlockConditionDto),
                T3(&'a TimelockUnlockConditionDto),
                T4(&'a ExpirationUnlockConditionDto),
                T5(&'a StateControllerAddressUnlockConditionDto),
                T6(&'a GovernorAddressUnlockConditionDto),
                T7(&'a ImmutableAliasAddressUnlockConditionDto),
            }
            #[derive(Serialize)]
            struct TypedUnlockCondition<'a> {
                #[serde(flatten)]
                unlock_condition: UnlockConditionDto_<'a>,
            }
            let unlock_condition = match self {
                Self::Address(o) => TypedUnlockCondition {
                    unlock_condition: UnlockConditionDto_::T1(o),
                },
                Self::StorageDepositReturn(o) => TypedUnlockCondition {
                    unlock_condition: UnlockConditionDto_::T2(o),
                },
                Self::Timelock(o) => TypedUnlockCondition {
                    unlock_condition: UnlockConditionDto_::T3(o),
                },
                Self::Expiration(o) => TypedUnlockCondition {
                    unlock_condition: UnlockConditionDto_::T4(o),
                },
                Self::StateControllerAddress(o) => TypedUnlockCondition {
                    unlock_condition: UnlockConditionDto_::T5(o),
                },
                Self::GovernorAddress(o) => TypedUnlockCondition {
                    unlock_condition: UnlockConditionDto_::T6(o),
                },
                Self::ImmutableAliasAddress(o) => TypedUnlockCondition {
                    unlock_condition: UnlockConditionDto_::T7(o),
                },
            };
            unlock_condition.serialize(serializer)
        }
    }

    impl From<&UnlockCondition> for UnlockConditionDto {
        fn from(value: &UnlockCondition) -> Self {
            match value {
                UnlockCondition::Address(v) => Self::Address(AddressUnlockConditionDto::from(v)),
                UnlockCondition::StorageDepositReturn(v) => {
                    Self::StorageDepositReturn(StorageDepositReturnUnlockConditionDto::from(v))
                }
                UnlockCondition::Timelock(v) => Self::Timelock(TimelockUnlockConditionDto::from(v)),
                UnlockCondition::Expiration(v) => Self::Expiration(ExpirationUnlockConditionDto::from(v)),
                UnlockCondition::StateControllerAddress(v) => {
                    Self::StateControllerAddress(StateControllerAddressUnlockConditionDto::from(v))
                }
                UnlockCondition::GovernorAddress(v) => {
                    Self::GovernorAddress(GovernorAddressUnlockConditionDto::from(v))
                }
                UnlockCondition::ImmutableAliasAddress(v) => {
                    Self::ImmutableAliasAddress(ImmutableAliasAddressUnlockConditionDto::from(v))
                }
            }
        }
    }

    impl UnlockCondition {
        pub fn try_from_dto(value: &UnlockConditionDto, token_supply: u64) -> Result<Self, DtoError> {
            Ok(match value {
                UnlockConditionDto::Address(v) => Self::Address(AddressUnlockCondition::try_from(v)?),
                UnlockConditionDto::StorageDepositReturn(v) => {
                    Self::StorageDepositReturn(StorageDepositReturnUnlockCondition::try_from_dto(v, token_supply)?)
                }
                UnlockConditionDto::Timelock(v) => Self::Timelock(TimelockUnlockCondition::try_from(v)?),
                UnlockConditionDto::Expiration(v) => Self::Expiration(ExpirationUnlockCondition::try_from(v)?),
                UnlockConditionDto::StateControllerAddress(v) => {
                    Self::StateControllerAddress(StateControllerAddressUnlockCondition::try_from(v)?)
                }
                UnlockConditionDto::GovernorAddress(v) => {
                    Self::GovernorAddress(GovernorAddressUnlockCondition::try_from(v)?)
                }
                UnlockConditionDto::ImmutableAliasAddress(v) => {
                    Self::ImmutableAliasAddress(ImmutableAliasAddressUnlockCondition::try_from(v)?)
                }
            })
        }

        pub fn try_from_dto_unverified(value: &UnlockConditionDto) -> Result<Self, DtoError> {
            Ok(match value {
                UnlockConditionDto::Address(v) => Self::Address(AddressUnlockCondition::try_from(v)?),
                UnlockConditionDto::StorageDepositReturn(v) => {
                    Self::StorageDepositReturn(StorageDepositReturnUnlockCondition::try_from_dto_unverified(v)?)
                }
                UnlockConditionDto::Timelock(v) => Self::Timelock(TimelockUnlockCondition::try_from(v)?),
                UnlockConditionDto::Expiration(v) => Self::Expiration(ExpirationUnlockCondition::try_from(v)?),
                UnlockConditionDto::StateControllerAddress(v) => {
                    Self::StateControllerAddress(StateControllerAddressUnlockCondition::try_from(v)?)
                }
                UnlockConditionDto::GovernorAddress(v) => {
                    Self::GovernorAddress(GovernorAddressUnlockCondition::try_from(v)?)
                }
                UnlockConditionDto::ImmutableAliasAddress(v) => {
                    Self::ImmutableAliasAddress(ImmutableAliasAddressUnlockCondition::try_from(v)?)
                }
            })
        }
    }

    impl UnlockConditionDto {
        /// Return the unlock condition kind of a `UnlockConditionDto`.
        pub fn kind(&self) -> u8 {
            match self {
                Self::Address(_) => AddressUnlockCondition::KIND,
                Self::StorageDepositReturn(_) => StorageDepositReturnUnlockCondition::KIND,
                Self::Timelock(_) => TimelockUnlockCondition::KIND,
                Self::Expiration(_) => ExpirationUnlockCondition::KIND,
                Self::StateControllerAddress(_) => StateControllerAddressUnlockCondition::KIND,
                Self::GovernorAddress(_) => GovernorAddressUnlockCondition::KIND,
                Self::ImmutableAliasAddress(_) => ImmutableAliasAddressUnlockCondition::KIND,
            }
        }
    }
}
