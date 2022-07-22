// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import org.apache.commons.codec.binary.Hex;

/*
An incorrect seed conversion from Java to Rust in February 2022 resulted in incorrectly derived addresses. See https://github.com/iotaledger/iota.rs/pull/800 for more details.
This secret manager gives access to the funds located on the incorrectly derived addresses.
 */
public class WrongSeedConversionSecretManager extends SeedSecretManager {
    
    public WrongSeedConversionSecretManager(String hexSeed) {
        // Remove hex prefix and add it later again
        super("0x"+Hex.encodeHexString(hexSeed.replace("0x", "").getBytes()));
    }

}