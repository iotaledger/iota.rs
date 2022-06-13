from iota_client._base_api import BaseAPI


class NodeCoreAPI(BaseAPI):

    def get_node_health(self, url):
        """ Get node health.
        """
        return self.call_client_method('GetNodeHealth', {
            'url': url
        })

    def get_node_info(self, url, auth=None):
        """Get node info.
        """
        return self.call_client_method('GetNodeInfo', {
            'url': url,
            'auth': auth
        })

    def get_info(self):
        """Returns the node information together with the url of the used node.
        """
        return self.call_client_method('GetInfo')

    def get_peers(self):
        """Get peers.
        """
        return self.call_client_method('GetPeers')

    def get_tips(self):
        """Get tips.
        """
        return self.call_client_method('GetTips')

    def post_block(self, block):
        """Post block.
        """
        return self.call_client_method('PostBlockJson', {
            'block': block
        })

    def get_block_data(self, block_id):
        """Post block.
        """
        return self.call_client_method('GetBlock', {
            'block_id': block_id
        })

    def get_block_metadata(self, block_id):
        """Get block metadata with block_id.
        """
        return self.call_client_method('GetBlockMetadata', {
            'block_id': block_id
        })

    def get_block_raw(self, block_id):
        """Get block raw.
        """
        return self.call_client_method('GetBlockRaw', {
            'block_id': block_id
        })

    def get_output(self, output_id):
        """Get output.
        """
        return self.call_client_method('GetOutput', {
            'output_id': output_id
        })

    def get_output_metadata(self, output_id):
        """Get output metadata.
        """
        return self.call_client_method('GetOutputMetadata', {
            'output_id': output_id
        })

    def get_milestone_by_id(self, milestone_id):
        """Get the milestone by the given milestone id.
        """
        return self.call_client_method('GetMilestoneById', {
            'milestone_id': milestone_id
        })

    def get_milestone_by_id_raw(self, milestone_id):
        """Get the raw milestone by the given milestone id.
        """
        return self.call_client_method('GetMilestoneByIdRaw', {
            'milestone_id': milestone_id
        })

    def get_milestone_by_index(self, index):
        """Get the milestone by the given index.
        """
        return self.call_client_method('GetMilestoneByIndex', {
            'index': index
        })

    def get_milestone_by_index_raw(self, index):
        """Get the milestone by the given index.
        """
        return self.call_client_method('GetMilestoneByIndexRaw', {
            'index': index
        })

    def get_utxo_changes_by_id(self, milestone_id):
        """Get the UTXO changes by the given milestone id.
        """
        return self.call_client_method('GetUtxoChangesById', {
            'milestone_id': milestone_id
        })

    def get_utxo_changes_by_index(self, index):
        """Get the UTXO changes by the given milestone index.
        """
        return self.call_client_method('GetUtxoChangesByIndex', {
            'index': index
        })

    def get_receipts(self):
        """Get all receipts.
        """
        return self.call_client_method('GetReceipts')

    def get_receipts_migrated_at(self, milestone_index):
        """Get the receipts by the given milestone index.
        """
        return self.call_client_method('GetReceiptsMigratedAt', {
            'milestone_index': milestone_index
        })

    def get_treasury(self):
        """Get the treasury output.
        """
        return self.call_client_method('GetTreasury')

    def get_included_block(self, transaction_id):
        """Returns the included block of the transaction.
        """
        return self.call_client_method('GetIncludedBlock', {
            'transaction_id': transaction_id
        })
