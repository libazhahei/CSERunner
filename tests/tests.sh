#!/bin/bash

CSERUNNER_ROOT=~/.cserunner
BUILD_PATH=../cserunner/target

echo "Building cserunner"
cd ../cserunner
cargo build --release > /dev/null
if [ $? -ne 0 ]; then
    echo "Build failed"
    exit 1
fi
cd ../tests

CSERUNNER_PATH=$BUILD_PATH/release/cserunner

restore_cserunner_root() {
    rm -rf $CSERUNNER_ROOT
    if [ -d "$CSERUNNER_ROOT.bak" ]; then
        mv $CSERUNNER_ROOT.bak $CSERUNNER_ROOT
    fi
}

trap restore_cserunner_root EXIT

if [ -d "$CSERUNNER_ROOT" ]; then
    echo "Backup existing cserunner root"
    mv $CSERUNNER_ROOT $CSERUNNER_ROOT.bak
fi


echo "Testing Init module"
./init.sh $CSERUNNER_PATH