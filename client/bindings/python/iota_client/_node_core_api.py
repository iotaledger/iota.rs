from iota_client._base_api import BaseAPI


class NodeCoreAPI(BaseAPI):

    def get_node_health(self, url):
        """ Get node health.
        """
        return self.send_message('getNodeHealth', {
            'url': url
        })

    def get_node_info(self, url, auth=None):
        """Get node info.
        """
        return self.send_message('getNodeInfo', {
            'url': url,
            'auth': auth
        })

    def get_info(self):
        """Returns the node information together with the url of the used node.
        """
        return self.send_message('getInfo')

    def get_peers(self):
        """Get peers.
        """
        return self.send_message('getPeers')

    def get_tips(self):
        """Get tips.
        """
        return self.send_message('getTips')

    def post_block(self, block):
        """Post block.
        """
        return self.send_message('postBlockJson', {
            'block': block
        })

    def get_block_data(self, block_id):
        """Post block.
        """
        return self.send_message('getBlock', {
            'blockId': block_id
        })

    def get_block_metadata(self, block_id):
        """Get block metadata with block_id.
        """
        return self.send_message('getBlockMetadata', {
            'blockId': block_id
        })

    def get_block_raw(self, block_id):
        """Get block raw.
        """
        return self.send_message('getBlockRaw', {
            'blockId': block_id
        })

    def post_block_raw(self, block_bytes):
        """Post block raw.
        """
        return self.send_message('postBlockRaw', {
            'blockBytes': block_bytes
        })

    def get_output(self, output_id):
        """Get output.
        """
        return self.send_message('getOutput', {
            'outputId': output_id
        })

    def get_output_metadata(self, output_id):
        """Get output metadata.
        """
        return self.send_message('getOutputMetadata', {
            'outputId': output_id
        })

    def get_milestone_by_id(self, milestone_id):
        """Get the milestone by the given milestone id.
        """
        return self.send_message('getMilestoneById', {
            'milestoneId': milestone_id
        })

    def get_milestone_by_id_raw(self, milestone_id):
        """Get the raw milestone by the given milestone id.
        """
        return self.send_message('getMilestoneByIdRaw', {
            'milestoneId': milestone_id
        })

    def get_milestone_by_index(self, index):
        """Get the milestone by the given index.
        """
        return self.send_message('getMilestoneByIndex', {
            'index': index
        })

    def get_milestone_by_index_raw(self, index):
        """Get the milestone by the given index.
        """
        return self.send_message('getMilestoneByIndexRaw', {
            'index': index
        })

    def get_utxo_changes_by_id(self, milestone_id):
        """Get the UTXO changes by the given milestone id.
        """
        return self.send_message('getUtxoChangesById', {
            'milestoneId': milestone_id
        })

    def get_utxo_changes_by_index(self, index):
        """Get the UTXO changes by the given milestone index.
        """
        return self.send_message('getUtxoChangesByIndex', {
            'index': index
        })

    def get_receipts(self):
        """Get all receipts.
        """
        return self.send_message('getReceipts')

    def get_receipts_migrated_at(self, milestone_index):
        """Get the receipts by the given milestone index.
        """
        return self.send_message('getReceiptsMigratedAt', {
            'milestoneIndex': milestone_index
        })

    def get_treasury(self):
        """Get the treasury output.
        """
        return self.send_message('getTreasury')

    def get_included_block(self, transaction_id):
        """Returns the included block of the transaction.
        """
        return self.send_message('getIncludedBlock', {
            'transactionId': transaction_id
        })

    def get_included_block_metadata(self, transaction_id):
        """Returns the metadata of the included block of the transaction.
        """
        return self.send_message('getIncludedBlockMetadata', {
            'transactionId': transaction_id
        })
