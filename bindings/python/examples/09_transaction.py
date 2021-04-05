import iota_client
client = iota_client.Client()

message = client.message(
    seed='b3d7092195c36d47133ff786d4b0a1ef2ee6a0052f6e87b6dc337935c70c531e',
    outputs=[
        {
            'address': 'atoi1qqydc70mpjdvl8l2wyseaseqwzhmedzzxrn4l9g2c8wdcsmhldz0ulwjxpz',
            'amount': 1_000_000
        }
    ]
)
print(message)