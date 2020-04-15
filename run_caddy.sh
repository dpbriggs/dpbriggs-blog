#!/bin/bash

source ~/.caddy_env
caddy -conf caddy/Caddyfile -log ~/caddy-log.log
