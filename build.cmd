:: Builds the Flycatcher CLI in debug mode.
@echo off

:: Build the executable.
cargo build

:: Make the lib directory.
set LIB_DIR="target\debug\lib\"
if not exist %LIB_DIR% (
    mkdir %LIB_DIR%
)

:: In the future, load the standard library into the %LIB_DIR%.