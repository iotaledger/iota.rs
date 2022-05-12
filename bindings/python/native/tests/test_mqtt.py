# Copyright 2021 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import iota_client
import json
import os
import queue
import time

# Read the test vector
tv = dict()
with open('../../../tests/fixtures/test_vectors.json') as json_file:
    tv = json.load(json_file)
tv = tv['python']

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
    nodes_name_password=[[tv['MQTT_NODE_URL']]], mqtt_broker_options=broker_options)

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
        assert event['topic'] in topics
        received_events += 1
        q.task_done()


def on_mqtt_event(event):
    """Put the received event to queue.
    """
    q.put(event)


def test_mqtt():
    client.subscribe_topics(
        ['milestones/confirmed', 'messages'], on_mqtt_event)
    worker(['milestones/confirmed', 'messages'])
    client.disconnect()
    q.queue.clear()
    client.subscribe_topic('messages', on_mqtt_event)
    worker('messages')
    client.unsubscribe_topics(['messages'])
    client.subscribe_topics(
        ['milestones/confirmed', 'messages'], on_mqtt_event)
    worker(['milestones/confirmed', 'messages'])
    client.unsubscribe()
    client.disconnect()
