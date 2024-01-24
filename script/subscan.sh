#!/bin/sh

set -e

if [ $# -lt 1 ]; then
    echo "This script builds a wasm contract for Subscan"
    echo ""
    echo "Usage: $0 <build arch>"
    echo ""
    echo "E.g. $0 amd64"
    exit 1
fi

echo "Building for Subscan..."
set -x
ARCH=$1
ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && cd .. && pwd )
OUT_DIR=$ROOT_DIR/$ARCH
SRC_DIR=$ROOT_DIR/contracts/metadogo
set +x
mkdir -p $OUT_DIR
docker run \
	--mount type=bind,source="$OUT_DIR",target="/target" \
	--mount type=bind,source="$SRC_DIR",target="/builds/contract" \
	quay.io/subscan-explorer/wasm-compile-build:$ARCH-stable-1.70.0-v3.2.0 \
	cargo contract build --release
cd $SRC_DIR
python3 $ROOT_DIR/script/convert.py --manifest Cargo.toml > $OUT_DIR/metadogo_standard_input.json
echo "Subscan standard_input.json file:"
echo "  - $OUT_DIR/metadogo_standard_input.json"
