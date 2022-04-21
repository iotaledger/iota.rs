import type { AddressDto } from '../address';

export type UnlockConditionDto =
    | AddressUnlockConditionDto
    | StorageDepositReturnUnlockConditionDto
    | TimelockUnlockConditionDto
    | ExpirationUnlockConditionDto
    | StateControllerAddressUnlockConditionDto
    | GovernorAddressUnlockConditionDto
    | ImmutableAliasAddressUnlockConditionDto;

export interface AddressUnlockConditionDto {
    type: number;
    address: AddressDto;
}

export interface StorageDepositReturnUnlockConditionDto {
    type: number;
    returnAddress: AddressDto;
    amount: string;
}

export interface TimelockUnlockConditionDto {
    type: number;
    milestoneIndex: number;
    timestamp: number;
}

export interface ExpirationUnlockConditionDto {
    type: number;
    returnAddress: AddressDto;
    milestoneIndex: number;
    timestamp: number;
}

export interface StateControllerAddressUnlockConditionDto {
    type: number;
    address: AddressDto;
}

export interface GovernorAddressUnlockConditionDto {
    type: number;
    address: AddressDto;
}

export interface ImmutableAliasAddressUnlockConditionDto {
    type: number;
    address: AddressDto;
}
