#! /bin/bash

NAME=rdb-redis

shutdown() {
    kill %1
    docker stop "$NAME"
    docker rm "$NAME"
}

# ensure that the container is stopped when the script ends
trap shutdown EXIT

# run redis-server
docker run --name "$NAME" -p "6379:6379" -d redis

# monitor logs
docker logs -f "$NAME" &

# Wait for user input
echo "Press any key to stop the Redis server..."
read -n 1 -s
