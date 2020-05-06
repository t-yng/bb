#!/bin/sh

BASEDIR=$(dirname "$0");
cp "$BASEDIR/../bin/bb" "/usr/local/bin/";
echo "bb was installed."
