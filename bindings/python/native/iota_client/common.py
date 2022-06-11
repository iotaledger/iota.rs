import iota_client
from json import dumps


def send_block_routine(func):
    """The routine of dump json string and call send_block()
    """
    def wrapper(*args, **kwargs):
        message = func(*args, **kwargs)
        message = dumps(message)

        # Send message to the Rust library
        response = iota_client.send_message(args[0].handle, message)
        return response
    return wrapper
