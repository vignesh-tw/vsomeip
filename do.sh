#!/bin/bash

command=$1

echo $command

function cmakeBuild() {
    echo "running cmake build"
    ( mkdir build ; cd build ; cmake .. ; make)
}

case $command in
    "cmake-build")
        cmakeBuild
        ;;
    *)
        echo "command $command do not exist"
        ;;
esac

