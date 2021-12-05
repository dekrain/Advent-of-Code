#!/usr/bin/sh
if [ -z "$CXX" ]; then
	CXX=c++
fi
"$CXX" -std=c++17 -o AoC *.cpp
