import iota_client
import json
from json import dumps


def send_message_routine(func):
    """The routine of dump json string and call send_message()
    """
    def wrapper(*args, **kwargs):
        message = func(*args, **kwargs)
        message = dumps(message)

        # Send message to the Rust library
        response = iota_client.send_message(args[0].handle, message)
        return json.loads(response)['payload']
    return wrapper
