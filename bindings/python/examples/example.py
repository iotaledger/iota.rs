# import iota
# some_file.py
# from ...lib import iota
# import importlib
# iota_client = importlib.import_module('iota_client', '../lib')
import iota_client
t = iota_client.Client(node="http://0.0.0.0:14265", node_sync_disabled=True)
print(t.get_health())
print(dict(t.get_info()))
print(t.get_tips())
# import iota_client
# import sys
# insert at position 1 in the path, as 0 is the path of this file.
# sys.path.insert(1, '../lib')

# file.function()
# # Create your client instance
# client = iota_client.Client("http://0.0.0.0:14265")
# print(client.get_address_balances())
# print(client.ttt(token=5))
# # # Send your token
# # message_id = client.send(seed="256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2",
# #                          path="m/",
# #                          address="6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92",
# #                          value=200)
# # print(f'Message ID: {message_id}')

# # # Check the balance
# # balance = client.get_address_balances(
# #     "6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92")

# # print(f'Balance: {balance}')
# print(iota_client.sum_as_string(2, 4))
# [pyclass]
# struct TestObject {
#     num: i32,
#     debug: bool,
# }
# t = iota_client.TestObject()

# t = iota_client.Client("http://0.0.0.0:14265")
# print(t)
