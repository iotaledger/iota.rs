# Interface: IFoundryOutputBuilderOptions

Options for building a Foundry Output

## Hierarchy

- [`IBasicOutputBuilderOptions`](IBasicOutputBuilderOptions.md)

  ↳ **`IFoundryOutputBuilderOptions`**

## Table of contents

### Properties

- [amount](IFoundryOutputBuilderOptions.md#amount)
- [nativeTokens](IFoundryOutputBuilderOptions.md#nativetokens)
- [unlockConditions](IFoundryOutputBuilderOptions.md#unlockconditions)
- [features](IFoundryOutputBuilderOptions.md#features)
- [serialNumber](IFoundryOutputBuilderOptions.md#serialnumber)

## Properties

### amount

• `Optional` **amount**: `string`

If not provided, minimum storage deposit will be used

#### Inherited from

[IBasicOutputBuilderOptions](IBasicOutputBuilderOptions.md).[amount](IBasicOutputBuilderOptions.md#amount)

___

### nativeTokens

• `Optional` **nativeTokens**: `INativeToken`[]

The native tokens to be held by the output.

#### Inherited from

[IBasicOutputBuilderOptions](IBasicOutputBuilderOptions.md).[nativeTokens](IBasicOutputBuilderOptions.md#nativetokens)

___

### unlockConditions

• **unlockConditions**: `UnlockConditionTypes`[]

The unlock conditions for the output.

#### Inherited from

[IBasicOutputBuilderOptions](IBasicOutputBuilderOptions.md).[unlockConditions](IBasicOutputBuilderOptions.md#unlockconditions)

___

### features

• `Optional` **features**: `FeatureTypes`[]

Features to be contained by the output.

#### Inherited from

[IBasicOutputBuilderOptions](IBasicOutputBuilderOptions.md).[features](IBasicOutputBuilderOptions.md#features)

___

### serialNumber

• **serialNumber**: `number`

The serial number of the foundry with respect to the controlling alias.
