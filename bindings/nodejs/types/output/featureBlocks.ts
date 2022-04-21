import type { AddressDto } from '../address';

export type FeatureBlockDto =
    | SenderFeatureBlockDto
    | IssuerFeatureBlockDto
    | MetadataFeatureBlockDto
    | TagFeatureBlockDto;

export interface SenderFeatureBlockDto {
    type: number;
    address: AddressDto;
}

export interface IssuerFeatureBlockDto {
    type: number;
    address: AddressDto;
}

export interface MetadataFeatureBlockDto {
    type: number;
    data: string;
}

export interface TagFeatureBlockDto {
    type: number;
    tag: string;
}
