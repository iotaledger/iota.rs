import iota_client
from json import dumps


def send_message_routine(func):
    """The routine of dump json string and call send_message()
    """
    def wrapper(*args, **kwargs):
        message_type = func(*args, **kwargs)
        message_type = dumps(message_type)

        # Send message to the Rust library
        response = iota_client.send_message(args[0].handle, message_type)
        return response
    return wrapper
