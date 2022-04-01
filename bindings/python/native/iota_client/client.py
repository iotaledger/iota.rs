import iota_client
from iota_client._node_core_api import NodeCoreAPI
from iota_client._node_indexr_api import NodeIndexerAPI
from iota_client._high_level_api import HighLevelAPI
from iota_client._utils import Utils
from json import dumps


class IotaClient(NodeCoreAPI, NodeIndexerAPI, HighLevelAPI, Utils):
    def __init__(self, client_config=None):
        """Initialize the IOTA Client.
        """
        if client_config:
            client_config = dumps(client_config)

        # Create the message handler
        self.handle = iota_client.create_message_handler(client_config)

    def get_handle(self):
        return self.handle

    def generate_addresses(self, signer, options):
        """Generate addresses.
        """
        return self.call_client_method('GenerateAddresses', {
            'signer': signer,
            'options': options
        })

    def generate_message(self, signer=None, options=None):
        """Generate client message.
        """
        return self.call_client_method('GenerateMessage', {
            'signer': signer,
            'options': options
        })

    def get_node(self):
        """Get a node candidate from the synced node pool.
        """
        return self.call_client_method('GetNode')

    def get_network_info(self):
        """Gets the network related information such as network_id and min_pow_score.
        """
        return self.call_client_method('GetNetworkInfo')

    def get_network_id(self):
        """Gets the network id of the node we're connecting to.
        """
        return self.call_client_method('GetNetworkId')

    def get_protocol_version(self):
        """Gets the protocol version of the node we're connecting to.
        """
        return self.call_client_method('GetProtocolVersion')

    def get_bech32_hrp(self):
        """Returns the bech32_hrp.
        """
        return self.call_client_method('GetBech32Hrp')

    def get_min_pow_score(self):
        """Returns the min pow score.
        """
        return self.call_client_method('GetMinPoWScore')

    def get_tips_interval(self):
        """Returns the tips interval.
        """
        return self.call_client_method('GetTipsInterval')

    def get_local_pow(self):
        """Returns if local pow should be used or not.
        """
        return self.call_client_method('GetLocalPoW')

    def get_rent_structure(self):
        """Get rent structure for the UTXO ledger.
        """
        return self.call_client_method('GetRentStructure')

    def get_fall_back_to_local_pow(self):
        """Get fallback to local proof of work timeout.
        """
        return self.call_client_method('GetFallbackToLocalPoW')

    def unsynced_nodes(self):
        """Returns the unsynced nodes.
        """
        return self.call_client_method('UnsyncedNodes')
