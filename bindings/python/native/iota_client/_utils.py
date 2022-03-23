
from iota_client.common import send_message_routine, call_client_method


class Utils():

    @send_message_routine
    def bech32_to_hex(self, bech32):
        """Transforms bech32 to hex.
        """
        return call_client_method('Bech32ToHex', {
            'bech32': bech32
        })

    @send_message_routine
    def bech32_to_hex(self, hex, bech32_hrp):
        """Transforms a hex encoded address to a bech32 encoded address.
        """
        return call_client_method('HexToBech32', {
            'hex': hex,
            'bech32_hrp': bech32_hrp
        })

    @send_message_routine
    def hex_public_key_to_beh32_address(self, hex, bech32_hrp=None):
        """Transforms a hex encoded public key to a bech32 encoded address.
        """
        return call_client_method('HexPublicKeyToBech32Address', {
            'hex': hex,
            'bech32_hrp': bech32_hrp
        })

    @send_message_routine
    def parse_bech32_address(self, address):
        """Returns a valid Address parsed from a String.
        """
        return call_client_method('ParseBech32Address', {
            'address': address
        })

    @send_message_routine
    def is_address_valid(self, address):
        """Checks if a String is a valid bech32 encoded address.
        """
        return call_client_method('IsAddressValid', {
            'address': address
        })

    @send_message_routine
    def generate_mnemonic(self):
        """Generates a new mnemonic.
        """
        return call_client_method('GenerateMnemonic')

    @send_message_routine
    def mnemonic_to_hex_seed(self, mnemonic):
        """Returns a hex encoded seed for a mnemonic.
        """
        return call_client_method('MnemonicToHexSeed', {
            'mnemonic': mnemonic
        })
