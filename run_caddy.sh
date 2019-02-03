#!/bin/bash

source ~/.caddy_env
caddy -c ~/dpbriggs-blog/caddy/Caddyfile -l ~/caddy-log.log
