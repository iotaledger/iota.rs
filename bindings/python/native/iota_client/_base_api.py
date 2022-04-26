from iota_client.common import send_message_routine


class BaseAPI():

    @send_message_routine
    def call_client_method(self, name, data=None):
        message = {
            'cmd': 'CallClientMethod',
            'payload': {
                'name': name
            }
        }
        if data:
            message['payload']['data'] = data
        return message
