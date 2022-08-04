from iota_client._base_api import BaseAPI


class NodeIndexerAPI(BaseAPI):

    def basic_output_ids(self, query_parameters):
        """Fetch basic output IDs.
        """
        return self.send_message('BasicOutputIds', {
            'queryParameters': query_parameters
        })

    def alias_output_ids(self, query_parameters):
        """Fetch alias output IDs.
        """
        return self.send_message('AliasOutputIds', {
            'queryParameters': query_parameters
        })

    def alias_output_id(self, alias_id):
        """Fetch alias output ID.
        """
        return self.send_message('AliasOutputId', {
            'aliasId': alias_id
        })

    def nft_output_ids(self, query_parameters):
        """Fetch NFT output IDs.
        """
        return self.send_message('NftOutputIds', {
            'queryParameters': query_parameters
        })

    def nft_output_id(self, nft_id):
        """Fetch NFT output ID.
        """
        return self.send_message('NftOutputId', {
            'nftId': nft_id
        })

    def foundry_output_ids(self, query_parameters):
        """Fetch foundry Output IDs.
        """
        return self.send_message('FoundryOutputIds', {
            'queryParameters': query_parameters
        })

    def foundry_output_id(self, foundry_id):
        """Fetch foundry Output ID.
        """
        return self.send_message('FoundryOutputId', {
            'foundryId': foundry_id
        })
