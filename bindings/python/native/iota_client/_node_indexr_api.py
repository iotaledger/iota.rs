from iota_client.common import send_message_routine, call_client_method


class NodeIndexerAPI():

    @send_message_routine
    def output_ids(self, query_parameters):
        """Fetch output IDs.
        """
        return call_client_method('OutputIds', {
            'query_parameters': query_parameters
        })

    @send_message_routine
    def aliases_output_ids(self, query_parameters):
        """Fetch aliases output IDs.
        """
        return call_client_method('AliasesOutputIds', {
            'query_parameters': query_parameters
        })

    @send_message_routine
    def alias_output_id(self, alias_id):
        """Fetch alias output ID.
        """
        return call_client_method('AliasOutputId', {
            'alias_id': alias_id
        })

    @send_message_routine
    def nfts_output_ids(self, query_parameters):
        """Fetch NFTs output IDs.
        """
        return call_client_method('NftsOutputIds', {
            'query_parameters': query_parameters
        })

    @send_message_routine
    def nft_output_id(self, nft_id):
        """Fetch NFT output ID.
        """
        return call_client_method('NftOutputId', {
            'nft_id': nft_id
        })

    @send_message_routine
    def foundries_output_ids(self, query_parameters):
        """Fetch Foundries Output IDs.
        """
        return call_client_method('FoundriesOutputIds', {
            'query_parameters': query_parameters
        })

    @send_message_routine
    def foundry_output_id(self, foundry_id):
        """Fetch Foundry Output ID.
        """
        return call_client_method('FoundryOutputId', {
            'foundry_id': foundry_id
        })
