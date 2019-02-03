#!/bin/bash

ulimit -n 8192

source ~/.caddy_env
caddy -conf ~/dpbriggs-blog/caddy/Caddyfile -log ~/caddy-log.log
