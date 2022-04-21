// TODO: what is this type for?
export interface Address {
    address: string;
}

export type AddressDto = Ed25519AddressDto | AliasAddressDto | NftAddressDto;

export interface Ed25519AddressDto {
    type: number;
    pubKeyHash: string;
}

export interface AliasAddressDto {
    type: number;
    aliasId: string;
}

export interface NftAddressDto {
    type: number;
    nftId: string;
}
