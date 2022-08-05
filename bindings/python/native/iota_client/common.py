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

        json_response = json.loads(response)

        if "type" in json_response:
            if json_response["type"] == "Error":
                raise IotaClientError(json_response['payload'])

        if "payload" in json_response:
            return json_response['payload']
        else:
            return response
    return wrapper

class IotaClientError(Exception):
    """iota-client error"""
    pass
