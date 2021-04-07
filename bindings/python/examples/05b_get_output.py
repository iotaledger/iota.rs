import iota_client
client = iota_client.Client()

print(
    client.get_output("a22cba0667c922cbb1f8bdcaf970b2a881ccd6e88e2fcce50374de2aac7c37720000")
)