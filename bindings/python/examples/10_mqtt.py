import iota_client
import json
import os
import queue
import time

# The node mqtt url
node_url = 'https://chrysalis-nodes.iota.org/'

# The queue to store received events
q = queue.Queue()

# The MQTT broker options
broker_options = {
    'automatic_disconnect': True,
    'timeout': 30,
    'use_ws': True,
    'port': 443,
    'max_reconnection_attempts': 5,
}

client = iota_client.Client(
    nodes_name_password=[[node_url]], mqtt_broker_options=broker_options)

# The queue to store received events
q = queue.Queue()

# The MQTT broker options
broker_options = {
    'automatic_disconnect': True,
    'timeout': 5,
    'use_ws': True,
    'port': 443,
    'max_reconnection_attempts': 5,
}


def worker(topics):
    """The worker to process the queued events.
    """
    received_events = 0
    while received_events < 10:
        item = q.get(True)
        event = json.loads(item)
        print(f'Received Event: {event}')
        message_id = client.get_message_id(str(event['payload']))
        print(f'Received message_id: {message_id}')
        received_events += 1
        q.task_done()


def on_mqtt_event(event):
    """Put the received event to queue.
    """
    q.put(event)


if __name__ == '__main__':
    client.subscribe_topics(['messages'], on_mqtt_event)
    worker(['messages'])
    client.disconnect()
    q.queue.clear()
