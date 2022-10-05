from iota_client._base_api import BaseAPI


class NodeIndexerAPI(BaseAPI):

    def basic_output_ids(self, query_parameters):
        """Fetch basic output IDs.
        """
        return self.send_message('basicOutputIds', {
            'queryParameters': query_parameters
        })

    def alias_output_ids(self, query_parameters):
        """Fetch alias output IDs.
        """
        return self.send_message('aliasOutputIds', {
            'queryParameters': query_parameters
        })

    def alias_output_id(self, alias_id):
        """Fetch alias output ID.
        """
        return self.send_message('aliasOutputId', {
            'aliasId': alias_id
        })

    def nft_output_ids(self, query_parameters):
        """Fetch NFT output IDs.
        """
        return self.send_message('nftOutputIds', {
            'queryParameters': query_parameters
        })

    def nft_output_id(self, nft_id):
        """Fetch NFT output ID.
        """
        return self.send_message('nftOutputId', {
            'nftId': nft_id
        })

    def foundry_output_ids(self, query_parameters):
        """Fetch foundry Output IDs.
        """
        return self.send_message('foundryOutputIds', {
            'queryParameters': query_parameters
        })

    def foundry_output_id(self, foundry_id):
        """Fetch foundry Output ID.
        """
        return self.send_message('foundryOutputId', {
            'foundryId': foundry_id
        })
