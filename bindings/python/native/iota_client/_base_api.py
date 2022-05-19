from iota_client.common import send_block_routine


class BaseAPI():

    @send_block_routine
    def call_client_method(self, name, data=None):
        block = {
            'cmd': 'CallClientMethod',
            'payload': {
                'name': name
            }
        }
        if data:
            block['payload']['data'] = data
        return block
