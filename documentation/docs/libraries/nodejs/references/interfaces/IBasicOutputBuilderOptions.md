# Interface: IBasicOutputBuilderOptions

Options for building a Basic Output

## Hierarchy

- **`IBasicOutputBuilderOptions`**

  ↳ [`IAliasOutputBuilderOptions`](IAliasOutputBuilderOptions.md)

  ↳ [`IFoundryOutputBuilderOptions`](IFoundryOutputBuilderOptions.md)

  ↳ [`INftOutputBuilderOptions`](INftOutputBuilderOptions.md)

## Table of contents

### Properties

- [amount](IBasicOutputBuilderOptions.md#amount)
- [nativeTokens](IBasicOutputBuilderOptions.md#nativetokens)
- [unlockConditions](IBasicOutputBuilderOptions.md#unlockconditions)
- [features](IBasicOutputBuilderOptions.md#features)

## Properties

### amount

• `Optional` **amount**: `string`

If not provided, minimum storage deposit will be used

___

### nativeTokens

• `Optional` **nativeTokens**: `INativeToken`[]

The native tokens to be held by the output.

___

### unlockConditions

• **unlockConditions**: `UnlockConditionTypes`[]

The unlock conditions for the output.

___

### features

• `Optional` **features**: `FeatureTypes`[]

Features to be contained by the output.
