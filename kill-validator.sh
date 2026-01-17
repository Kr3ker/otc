#!/bin/bash

# Kill solana validator on port 8899
PID=$(lsof -t -i :8899)
if [ -n "$PID" ]; then
  kill -9 $PID
  echo "Killed PID $PID on port 8899"
else
  echo "No process on port 8899"
fi

# Stop and remove arcium docker containers
CONTAINERS=$(docker ps -a -q --filter "name=arx" 2>/dev/null)
if [ -n "$CONTAINERS" ]; then
  docker stop $CONTAINERS 2>/dev/null
  docker rm $CONTAINERS 2>/dev/null
  echo "Stopped and removed arcium containers"
else
  echo "No arcium containers found"
fi
