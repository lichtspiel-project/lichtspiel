#!/usr/bin/env bash

set -Ee                               # fail script, if single command fails
set -u                                # unset variables are treated as errors
set -f                                # no filename extension
# set -x                                # verbose mode
set -o pipefail                       # if a subcommand from a pipe fails, the whole pipe fails
trap cleanup SIGINT SIGTERM ERR EXIT  # execute cleanup function at following signals

SOURCE_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)  # location of script
WORKING_DIR=$(pwd -P)  # working directory

cleanup() {
    trap - SIGINT SIGTERM ERR EXIT
    # script cleanup here
    echo "🧹 Time for cleanup"
}

echo "😊 This script is located at '${SOURCE_DIR}'."
echo "😊 This script was exectued from '${WORKING_DIR}'."


curl -OL https://downloads.sourceforge.net/project/pracrand/PractRand_0.93.zip
unzip -q PractRand_0.93.zip
g++ -std=c++14 -c $(find src -name '*.cpp') -O3 -Iinclude -pthread
ar rcs libPractRand.a $(find . -name '*.o')
g++ -std=c++14 -o RNG_test tools/RNG_test.cpp libPractRand.a -O3 -Iinclude -pthread
mv RNG_test /usr/local/bin