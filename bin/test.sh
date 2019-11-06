#!/bin/bash

set -e

autocannon -c 1000 -p 1000 -d 10 -m "GET" -b "💣" http://localhost:3000