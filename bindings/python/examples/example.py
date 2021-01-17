import iota_client
t = iota_client.Client(node="http://0.0.0.0:14265", node_sync_disabled=True)
print(t.get_health())
print(t.get_info()['name'])
print(dict(t.get_info()))
print(t.get_tips())
