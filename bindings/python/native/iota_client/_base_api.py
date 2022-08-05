from iota_client.common import send_message_routine


class BaseAPI():

    @send_message_routine
    def send_message(self, name, data=None):
        message = {
            'name': name,
            'data': data
        }

        return message
