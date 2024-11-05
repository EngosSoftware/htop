#!/usr/bin/env bash

###############################################################################
#                                                                             #
# Dependencies:                                                               #
#                                                                             #
# $ sudo dnf install lcov                                                     #
# $ rustup component add llvm-tools-preview                                   #
# $ cargo install grcov                                                       #
# $ cargo install htop                                                        #
#                                                                             #
###############################################################################

WORKING_DIRECTORY=$(pwd)
CARGO_NAME=$(grep -oE '^name = "[^"]+"' Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[^"]+')
CARGO_VERSION=$(grep -oE '^version = "[^"]+"' Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[^"]+')
BINARY_PATH="$WORKING_DIRECTORY"/target/debug
BBT_DIRECTORY="$WORKING_DIRECTORY"/bbt

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export LLVM_PROFILE_FILE="$WORKING_DIRECTORY/target/profraw/dsntk-%p-%m.profraw"

# run all unit tests
cargo +nightly test

# build binary before running black-box tests
cargo +nightly build

# run black-box
echo "$BBT_DIRECTORY"
if [[ -d "$BBT_DIRECTORY" ]]
then
  export PATH="$BINARY_PATH:$PATH"
  cd "$BBT_DIRECTORY" || exit 1
  ./run.sh ./tests
  cd "$WORKING_DIRECTORY" || exit 1
fi

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage

# generate coverage info
grcov . \
      --llvm \
      -s . \
      --binary-path "$BINARY_PATH" \
      -t lcov \
      --branch \
      --ignore-not-existing \
      --ignore "*cargo*" --ignore "*tests*" --ignore "*headless_chrome*" \
      -o ./target/lcov/lcov.info

# generate coverage report
genhtml -t "$CARGO_NAME v$CARGO_VERSION" -q -o ./target/coverage ./target/lcov/lcov.info

# generate coverage report in PDF format
if [ "$PDF_REPORT" != "" ]; then
  echo ""
  echo "Generating PDF report..."
  htop -b -l -p A4 --margin=4mm single ./target/coverage/index.html ./target/coverage/coverage.pdf
fi

# display the final message
echo ""
echo "Open coverage report:"
echo "  HTML file://$WORKING_DIRECTORY/target/coverage/index.html"
if [ "$PDF_REPORT" != "" ]; then
  echo "   PDF file://$WORKING_DIRECTORY/target/coverage/coverage.pdf"
fi
echo ""
