#!/bin/bash

# Run netcat to listen on port 50000 and read messages line by line
nc -lu -p 50000 | while IFS= read -r line; do
  echo "$line" | python -c 'from pyais.stream import UDPReceiver

host = "127.0.0.1"
port = 50000

for msg in UDPReceiver(host, port):
    print(msg.decode())
    # do something with it'
done


