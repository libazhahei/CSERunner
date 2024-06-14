#!/bin/bash

EXEC=$1
# Test 1: test init creates cserunner root directory
$EXEC init < "y" > /dev/null
# Check if there is a cserunner root directory exist
if [ ! -d "$CSERUNNER_ROOT" ]; then
    echo "Test 1 failed"
    echo "There is no cserunner root directory"
    echo "Current home directory:"
    ls ~
    exit 1
fi
rm -rf $CSERUNNER_ROOT


# Test 2: test init creates workspace directory
$EXEC init < "y" > /dev/null
# Check if there is a workspace directory exist in cserunner root
if [ $(ls -l $CSERUNNER_ROOT | grep -c "^d.*workspace_*") -ne 1 ]; then
    echo "Test 2 failed"
    echo "There is no workspace directory in cserunner root"
    exit 1
fi
# Check if there is a workspace_list json file exist in cserunner root
if [! -f "$CSERUNNER_ROOT/workspace_list.json" ]; then
    echo "Test 2 failed"
    echo "There is no workspace_list.json file in cserunner root"
    exit 1
fi
rm -rf $CSERUNNER_ROOT

# Test 3: test init with arguments
$EXEC init ../example_config.json < "y" > /dev/null
# Check if there is a workspace directory exist in cserunner root
if [ $(ls -l $CSERUNNER_ROOT | grep -c "^d.*workspace_*") -ne 1 ]; then
    echo "Test 3 failed"
    echo "There is no workspace directory in cserunner root"
    exit 1
fi
# Check if there is a workspace_list json file exist in cserunner root
if [! -f "$CSERUNNER_ROOT/workspace_list.json" ]; then
    echo "Test 3 failed"
    echo "There is no workspace_list.json file in cserunner root"
    exit 1
fi
rm -rf $CSERUNNER_ROOT

# Test 4: Check if there is a config.json file in workspace directory
$EXEC init < "y" > /dev/null
# get the workspace directory
WORKSPACE=$(ls $CSERUNNER_ROOT | grep "^workspace_*")
# Check if there is a config.json file in workspace directory
if [! -f "$CSERUNNER_ROOT/$WORKSPACE/config.json" ]; then
    echo "Test 4 failed"
    echo "There is no config.json file in workspace directory"
    exit 1
fi
if [! -f "$CSERUNNER_ROOT/$WORKSPACE/config.json.lock" ]; then
    echo "Test 4 failed"
    echo "There is no config.json file in workspace directory"
    exit 1
fi
rm -rf $CSERUNNER_ROOT

# Test 5: Cannot create workspace directory if there exists the same directory
$EXEC init < "y" > /dev/null
$EXEC init < "y" > /dev/null
# Check if error code is 1
if [ $? -ne 1 ]; then
    echo "Test 5 failed"
    echo "Cannot create workspace in the same directory"
    exit 1
fi
rm -rf $CSERUNNER_ROOT

# Test 6: Test init funcationality currectly
$EXEC init < "y" > /dev/null
$EXEC  > /dev/null
CURRENT_DIR=$(pwd)
# Check if error code is 1
if [ $? -ne 1 ]; then
    echo "Test 6 failed"
    echo "Success init but failed to load workspace in the same directory"
    exit 1
fi
mkdir temp
cd temp
$EXEC  > /dev/null
# Check if error code is 1
if [ $? -ne 1 ]; then
    echo "Test 6 failed"
    echo "Success init but failed to load workspace in the sub directory"
    cd $CURRENT_DIR
    rm -rf temp
    exit 1
fi
cd $CURRENT_DIR
rm -rf temp
cd ..
$EXEC  > /dev/null
# Check if error code is 1
if [ $? -eq 1 ]; then
    echo "Test 6 failed"
    echo "Success init but you cannot load workspace in the parent/differnt directory"
    cd $CURRENT_DIR
    rm -rf temp
    exit 1
fi
cd $CURRENT_DIR
rm -rf $CSERUNNER_ROOT

# Test success
echo "All tests passed"


