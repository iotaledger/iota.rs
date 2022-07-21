# Interface: IAliasOutputBuilderOptions

Options for building an Alias Output

## Hierarchy

- [`IBasicOutputBuilderOptions`](IBasicOutputBuilderOptions.md)

  ↳ **`IAliasOutputBuilderOptions`**

## Table of contents

### Properties

- [amount](IAliasOutputBuilderOptions.md#amount)
- [nativeTokens](IAliasOutputBuilderOptions.md#nativetokens)
- [unlockConditions](IAliasOutputBuilderOptions.md#unlockconditions)
- [features](IAliasOutputBuilderOptions.md#features)

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
