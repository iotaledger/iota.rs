from iota_client.common import send_message_routine, call_client_method


class NodeCoreAPI():

    @send_message_routine
    def get_node_health(self, url):
        """ Get node health.
        """
        return call_client_method('GetNodeHealth', {
            'url': url
        })

    @send_message_routine
    def get_health(self):
        """Get node health.
        """
        return call_client_method('GetHealth')

    @send_message_routine
    def get_node_info(self, url, auth=None):
        """Get node info.
        """
        return call_client_method('GetNodeInfo', {
            'url': url,
            'auth': auth
        })

    @send_message_routine
    def get_info(self):
        """Returns the node information together with the url of the used node.
        """
        return call_client_method('GetInfo')

    @send_message_routine
    def get_peers(self):
        """Get peers.
        """
        return call_client_method('GetPeers')

    @send_message_routine
    def get_tips(self):
        """Get tips.
        """
        return call_client_method('GetTips')

    @send_message_routine
    def post_message(self, message):
        """Post message.
        """
        return call_client_method('PostMessageJson', {
            'message': message
        })

    @send_message_routine
    def get_message_data(self, message_id):
        """Post message.
        """
        return call_client_method('GetMessageData', {
            'message_id': message_id
        })

    @send_message_routine
    def get_message_metadata(self, message_id):
        """Get message metadata with message_id.
        """
        return call_client_method('GetMessageMetadata', {
            'message_id': message_id
        })

    @send_message_routine
    def get_message_raw(self, message_id):
        """Get message raw.
        """
        return call_client_method('GetMessageRaw', {
            'message_id': message_id
        })

    @send_message_routine
    def get_message_children(self, message_id):
        """Get message children.
        """
        return call_client_method('GetMessageChildren', {
            'message_id': message_id
        })

    @send_message_routine
    def get_output(self, output_id):
        """Get output.
        """
        return call_client_method('GetOutput', {
            'output_id': output_id
        })

    @send_message_routine
    def get_milestone(self, index):
        """Get the milestone by the given index.
        """
        return call_client_method('GetMilestone', {
            'index': index
        })

    @send_message_routine
    def get_milestone_uxto_changes(self, index):
        """Get the milestone utxo changes by the given index.
        """
        return call_client_method('GetMilestoneUtxoChanges', {
            'index': index
        })

    @send_message_routine
    def get_receipts(self):
        """Get all receipts.
        """
        return call_client_method('GetReceipts')

    @send_message_routine
    def get_receipts_migrated_at(self, milestone_index):
        """Get the receipts by the given milestone index.
        """
        return call_client_method('GetReceiptsMigratedAt', {
            'milestone_index': milestone_index
        })

    @send_message_routine
    def get_treasury(self):
        """Get the treasury output.
        """
        return call_client_method('GetTreasury')

    @send_message_routine
    def get_included_message(self, transaction_id):
        """Returns the included message of the transaction.
        """
        return call_client_method('GetIncludedMessage', {
            'transaction_id': transaction_id
        })
