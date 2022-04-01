from iota_client._base_api import BaseAPI


class NodeCoreAPI(BaseAPI):

    def get_node_health(self, url):
        """ Get node health.
        """
        return self.call_client_method('GetNodeHealth', {
            'url': url
        })

    def get_health(self):
        """Get node health.
        """
        return self.call_client_method('GetHealth')

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

    def post_message(self, message):
        """Post message.
        """
        return self.call_client_method('PostMessageJson', {
            'message': message
        })

    def get_message_data(self, message_id):
        """Post message.
        """
        return self.call_client_method('GetMessageData', {
            'message_id': message_id
        })

    def get_message_metadata(self, message_id):
        """Get message metadata with message_id.
        """
        return self.call_client_method('GetMessageMetadata', {
            'message_id': message_id
        })

    def get_message_raw(self, message_id):
        """Get message raw.
        """
        return self.call_client_method('GetMessageRaw', {
            'message_id': message_id
        })

    def get_message_children(self, message_id):
        """Get message children.
        """
        return self.call_client_method('GetMessageChildren', {
            'message_id': message_id
        })

    def get_output(self, output_id):
        """Get output.
        """
        return self.call_client_method('GetOutput', {
            'output_id': output_id
        })

    def get_milestone(self, index):
        """Get the milestone by the given index.
        """
        return self.call_client_method('GetMilestone', {
            'index': index
        })

    def get_milestone_uxto_changes(self, index):
        """Get the milestone utxo changes by the given index.
        """
        return self.call_client_method('GetMilestoneUtxoChanges', {
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

    def get_included_message(self, transaction_id):
        """Returns the included message of the transaction.
        """
        return self.call_client_method('GetIncludedMessage', {
            'transaction_id': transaction_id
        })
