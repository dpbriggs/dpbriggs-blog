#!/bin/bash

source ~/.caddy_env
# caddy2: /usr/bin/caddy
# caddy: caddy
# dunno how logging works in caddy2
# /usr/bin/caddy run -config caddy/Caddyfile -log ~/caddy-log.log

# Don't forget:
# sudo setcap CAP_NET_BIND_SERVICE=+eip /usr/bin/caddy

/usr/bin/caddy run -config caddy/Caddyfile
