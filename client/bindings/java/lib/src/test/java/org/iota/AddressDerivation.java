package org.iota;

import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.Range;
import org.iota.types.secret.SeedSecretManager;
import org.junit.jupiter.api.Test;

public class AddressDerivation extends ApiTest {

    @Test
    public void testAddressDerivation() throws ClientException, InitializeClientException {
        // The hex seed that is affected by the seed conversion bug.
        String hexSeed = "0x4e4f4e5345435552455f5553455f4f465f444556454c4f504d454e545f534545445f31";

        // Test the hex seed with the wrong + valid seed secret manager.
        org.iota.types.secret.WrongSeedConversionSecretManager wrongSecretManager = new org.iota.types.secret.WrongSeedConversionSecretManager(hexSeed);
        SeedSecretManager correctSecretManager = new SeedSecretManager(hexSeed);

        // Generate the first address.
        String wrongAddress = client.hexToBech32(client.bech32ToHex(client.generateAddresses(wrongSecretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)).withCoinType(4218))[0]), "atoi");
        String correctAddress = client.hexToBech32(client.bech32ToHex(client.generateAddresses(correctSecretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)).withCoinType(4218))[0]), "atoi");

        if (wrongAddress.equals("atoi1qzzj3wa2c0m0mpe6s2v004037sjhyk7zgr7hj3umwgnanr9xy6c92qyz3c8") && correctAddress.equals("atoi1qp5dzudmpxxz7xxlzez8w5ttefeanhpf9rju48ds5y2ellp6aauuztf0dyd")) {
            System.out.println("success");
        }
    }
}
