#!/bin/bash
source $HOME/.cargo/env

if [ -z "$(ls /source)" ]; then
    echo "Please mount the git repo at /source"
    echo "docker run --rm -v $(pwd):/source blog-maker"
    exit 1
fi

BUILD_FOLDER="/build-arena"

git clone /source $BUILD_FOLDER
cd $BUILD_FOLDER
chmod +x make_package.sh
./make_package.sh
cp $BUILD_FOLDER/dpbriggs-blog-*.zip /source
