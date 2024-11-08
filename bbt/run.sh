#!/usr/bin/env bash

echo " "
echo "────────────────────────────────────"
echo " Testing: $(htop -V)"
echo "────────────────────────────────────"

RUN_FILE=run.sh
WORK_DIR=$(pwd)

PASSED=0
FAILED=0
FAILURE=0
PERCENT=0.7

#******************************************************************************
# Increments passed tests counter.
#******************************************************************************
function inc_passed () {
  PASSED=$((PASSED+1))
  echo -e "\e[32m OK\e[0m"
}

#******************************************************************************
# Increments failed tests counter.
#******************************************************************************
function inc_failed () {
  FAILED=$((FAILED+1))
  echo -e "\e[31m FAILED\e[0m"
}

#******************************************************************************
# Compares files named `expected` and `actual` using `diff` tool.
#******************************************************************************
function compare_using_diff () {
  diff expected actual -y >> /dev/null 2>&1
  RESULT=$?
  if [ $RESULT -ne 0 ]; then
    FAILURE=1
    echo ""
    echo -e "\e[31mEXPECTED\e[0m                                                                                                \e[32mACTUAL\e[0m"
    echo -e "\e[31m─────────────────────────────────────────────────────────────────────────────────────────────────\e[0m       \e[32m─────────────────────────────────────────────────────────────────────────────────────────────────\e[0m"
    diff expected actual -y --color=always -W 200
    echo ""
    echo ""
    echo "Full report:"
    diff expected actual --color=always
  fi
}

#******************************************************************************
# Compares files using `biff` tool.
#
# Parameters:
#
# $1 - name of the file with expected content
# $2 - name of the file with actual content
#
#******************************************************************************
function compare_using_biff () {
  biff -p $PERCENT "$1" "$2"
  RESULT=$?
  if [ $RESULT -ne 0 ]; then
    FAILURE=1
  fi
}

#******************************************************************************
# Iterate over subdirectories and execute tests.
#******************************************************************************
for dir in $(find "$1" -type d | sort); do
  if [ -f "$dir/$RUN_FILE" ]; then
    FAILURE=0
    cd "$dir" || exit 1
    if [ -f expected ]; then
      ./"$RUN_FILE" > actual
      compare_using_diff
    else
      ./"$RUN_FILE"
    fi
    if [ -f expected.1 ]; then
      for EXPECTED in expected.*; do
        ACTUAL=${EXPECTED/expected/actual}
        if [ -f "$ACTUAL" ]; then
          compare_using_biff "$EXPECTED" "$ACTUAL"
        else
          echo "File named '$ACTUAL' was not found."
          FAILURE=1
          break
        fi
      done
    fi
    echo -n "$dir/$RUN_FILE"
    if [ $FAILURE -eq 0 ]; then
      inc_passed
    else
      inc_failed
    fi
    if [ "$2" != "trace" ]; then
      rm -f actual*
    fi
    cd "$WORK_DIR" || exit 1
  fi
done

# Display test summary.
echo "────────────────────────────────────"
echo " $PASSED passed, $FAILED failed"
echo "────────────────────────────────────"
echo " "

exit 0
