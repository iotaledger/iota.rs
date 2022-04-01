from iota_client._base_api import BaseAPI


class NodeIndexerAPI(BaseAPI):

    def output_ids(self, query_parameters):
        """Fetch output IDs.
        """
        return self.call_client_method('OutputIds', {
            'query_parameters': query_parameters
        })

    def aliases_output_ids(self, query_parameters):
        """Fetch aliases output IDs.
        """
        return self.call_client_method('AliasesOutputIds', {
            'query_parameters': query_parameters
        })

    def alias_output_id(self, alias_id):
        """Fetch alias output ID.
        """
        return self.call_client_method('AliasOutputId', {
            'alias_id': alias_id
        })

    def nfts_output_ids(self, query_parameters):
        """Fetch NFTs output IDs.
        """
        return self.call_client_method('NftsOutputIds', {
            'query_parameters': query_parameters
        })

    def nft_output_id(self, nft_id):
        """Fetch NFT output ID.
        """
        return self.call_client_method('NftOutputId', {
            'nft_id': nft_id
        })

    def foundries_output_ids(self, query_parameters):
        """Fetch Foundries Output IDs.
        """
        return self.call_client_method('FoundriesOutputIds', {
            'query_parameters': query_parameters
        })

    def foundry_output_id(self, foundry_id):
        """Fetch Foundry Output ID.
        """
        return self.call_client_method('FoundryOutputId', {
            'foundry_id': foundry_id
        })
