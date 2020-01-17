#!/bin/bash

ulimit -n 8192

source ~/.caddy_env
caddy -conf caddy/Caddyfile -log ~/caddy-log.log
