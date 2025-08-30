#!/bin/bash

if [ -f "testdata/lakemountain.original.png" ]; then
    mv testdata/lakemountain.original.png testdata/lakemountain.png
fi
if [ -f "testdata/lakemountain.original.jpg" ]; then
    mv testdata/lakemountain.original.jpg testdata/lakemountain.jpg
fi

cargo run testdata
