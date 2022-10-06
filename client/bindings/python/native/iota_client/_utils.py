from iota_client._base_api import BaseAPI


class Utils(BaseAPI):

    def bech32_to_hex(self, bech32):
        """Transforms bech32 to hex.
        """
        return self.send_message('bech32ToHex', {
            'bech32': bech32
        })

    def hex_to_bech32(self, hex, bech32_hrp):
        """Transforms a hex encoded address to a bech32 encoded address.
        """
        return self.send_message('hexToBech32', {
            'hex': hex,
            'bech32Hrp': bech32_hrp
        })

    def hex_public_key_to_beh32_address(self, hex, bech32_hrp=None):
        """Transforms a hex encoded public key to a bech32 encoded address.
        """
        return self.send_message('hexPublicKeyToBech32Address', {
            'hex': hex,
            'bech32Hrp': bech32_hrp
        })

    def parse_bech32_address(self, address):
        """Returns a valid Address parsed from a String.
        """
        return self.send_message('parseBech32Address', {
            'address': address
        })

    def is_address_valid(self, address):
        """Checks if a String is a valid bech32 encoded address.
        """
        return self.send_message('isAddressValid', {
            'address': address
        })

    def generate_mnemonic(self):
        """Generates a new mnemonic.
        """
        return self.send_message('generateMnemonic')

    def mnemonic_to_hex_seed(self, mnemonic):
        """Returns a hex encoded seed for a mnemonic.
        """
        return self.send_message('mnemonicToHexSeed', {
            'mnemonic': mnemonic
        })

    def block_id(self, block):
        """ Returns a block ID (Blake2b256 hash of block bytes) from a block.
        """
        return self.send_message('blockId', {
            'block': block
        })
