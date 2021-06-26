#!/bin/bash

(cd ./backend; ./scripts/start-database.sh; cargo run) &
api=$!

(cd ./frontend; BROWSER=false npm run start) &
client=$!

docker run --rm --name shortest-url-dev-nginx --net="host" -v $PWD/nginx.conf:/etc/nginx/nginx.conf:ro nginx:alpine &
proxy=$!

wait $api $client $proxy
