from iota_client._base_api import BaseAPI


class Utils(BaseAPI):

    def bech32_to_hex(self, bech32):
        """Transforms bech32 to hex.
        """
        return self.call_client_method('Bech32ToHex', {
            'bech32': bech32
        })

    def bech32_to_hex(self, hex, bech32_hrp):
        """Transforms a hex encoded address to a bech32 encoded address.
        """
        return self.call_client_method('HexToBech32', {
            'hex': hex,
            'bech32_hrp': bech32_hrp
        })

    def hex_public_key_to_beh32_address(self, hex, bech32_hrp=None):
        """Transforms a hex encoded public key to a bech32 encoded address.
        """
        return self.call_client_method('HexPublicKeyToBech32Address', {
            'hex': hex,
            'bech32_hrp': bech32_hrp
        })

    def parse_bech32_address(self, address):
        """Returns a valid Address parsed from a String.
        """
        return self.call_client_method('ParseBech32Address', {
            'address': address
        })

    def is_address_valid(self, address):
        """Checks if a String is a valid bech32 encoded address.
        """
        return self.call_client_method('IsAddressValid', {
            'address': address
        })

    def generate_mnemonic(self):
        """Generates a new mnemonic.
        """
        return self.call_client_method('GenerateMnemonic')

    def mnemonic_to_hex_seed(self, mnemonic):
        """Returns a hex encoded seed for a mnemonic.
        """
        return self.call_client_method('MnemonicToHexSeed', {
            'mnemonic': mnemonic
        })

    def block_id(self, block):
        """ Returns a block ID (Blake2b256 hash of block bytes) from a block.
        """
        return self.call_client_method('BlockId', {
            'block': block
        })
