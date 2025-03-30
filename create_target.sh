#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$(dirname $(realpath $0))"

function print_usage() {
	echo "Usage : create_target <SYSROOT> <TARGET> <CFLAGS>"
}

if [ $# != 3 ]; then
	print_usage
	exit 1
fi

readonly target=$1
readonly sysroot=$2
readonly cflags=$3

mkdir -p cmake

cat - << EOF >> "$SCRIPT_DIR"/cmake/target/"$target-clang.cmake"
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSROOT "$sysroot")
set(CMAKE_C_COMPILER_TARGET "$target")
set(CMAKE_CXX_COMPILER_TARGET "$target")
set(CMAKE_C_FLAGS_INIT "$cflags")
set(CMAKE_CXX_FLAGS_INIT "$cflags")
set(CMAKE_LINKER_TYPE LLD)
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)
EOF