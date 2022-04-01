from iota_client._base_api import BaseAPI


class HighLevelAPI(BaseAPI):

    def get_outputs(self, output_ids):
        """Fetch OutputResponse from provided OutputIds (requests are sent in parallel).
        """
        return self.call_client_method('GetOutputs', {
            'output_ids': output_ids
        })

    def try_get_outputs(self, output_ids):
        """Try to get OutputResponse from provided OutputIds.
           Requests are sent in parallel and errors are ignored, can be useful for spent outputs.
        """
        return self.call_client_method('TryGetOutputs', {
            'output_ids': output_ids
        })

    def find_messages(self, message_ids):
        """Find all messages by provided message IDs.
        """
        return self.call_client_method('FindMessages', {
            'message_ids': message_ids
        })

    def retry(self, message_id):
        """Retries (promotes or reattaches) a message for provided message id. Message should only be
           retried only if they are valid and haven't been confirmed for a while.
        """
        return self.call_client_method('Retry', {'message_id': message_id})

    def retry_until_included(self, message_id, interval=None, max_attempts=None):
        """Retries (promotes or reattaches) a message for provided message id until it's included (referenced by a
           milestone). Default interval is 5 seconds and max attempts is 40. Returns the included message at first
           position and additional reattached messages.
        """
        return self.call_client_method('RetryUntilIncluded', {
            'message_id': message_id,
            'interval': interval,
            'max_attempts': max_attempts
        })

    def consolidate_funds(self, signer, account_index, address_range):
        """Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
           Returns the address to which the funds got consolidated, if any were available.
        """
        return self.call_client_method('ConsolidateFunds', {
            'signer': signer,
            'account_index': account_index,
            'address_range': address_range
        })

    def find_inputs(self, addresses, amount):
        """Function to find inputs from addresses for a provided amount (useful for offline signing)
        """
        return self.call_client_method('FindInputs', {
            'addresses': addresses,
            'amount': amount
        })

    def find_outputs(self, outputs, addresses):
        """Find all outputs based on the requests criteria. This method will try to query multiple nodes if
           the request amount exceeds individual node limit.
        """
        return self.call_client_method('FindOutputs', {
            'outputs': outputs,
            'addresses': addresses
        })

    def reattach(self, message_id):
        """Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
           confirmed for a while.
        """
        return self.call_client_method('Reattach', {
            'message_id': message_id
        })

    def reattach_unchecked(self, message_id):
        """Reattach a message without checking if it should be reattached.
        """
        return self.call_client_method('ReattachUnchecked', {
            'message_id': message_id
        })

    def promote(self, message_id):
        """Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
           method should error out and should not allow unnecessary promotions.
        """
        return self.call_client_method('Promote', {
            'message_id': message_id
        })

    def promote_unchecked(self, message_id):
        """Promote a message without checking if it should be promoted.
        """
        return self.call_client_method('PromoteUnchecked', {
            'message_id': message_id
        })
