#!/bin/sh

addr=127.0.0.1
port=8128
dir="./target"

python3 \
	-m http.server \
	--bind "${addr}" \
	--directory "${dir}" \
	${port}
