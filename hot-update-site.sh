#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cargo build --release


chmod +x $DIR/run_site.sh

source ~/.caddy-env
sudo pkill -USR1 caddy
$DIR/run_site.sh
