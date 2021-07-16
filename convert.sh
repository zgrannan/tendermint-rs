#!/usr/bin/env bash
set -euo pipefail

FILENAME="$1"

if ! grep "extern crate prusti_contracts" "$1" > /dev/null; then
    sed -i '1s;^;extern crate prusti_contracts\;\n;' "$FILENAME"
fi

function comment {
    sed -i "s/^\s*$1/\/\/ \0/" "$FILENAME"
}
function delete {
    sed -i "s/$1//" "$FILENAME"
}

comment "use serde::"

sed -i 's/derive(Serialize, Deserialize, /derive(/' "$1"
delete ', Serialize, Deserialize'
sed -i 's/r#type.try_into/0.try_into/' "$1"

comment "r#type: "
delete ', ::prost::Message'
delete ', ::prost::Oneof'
delete ', ::prost::Enumeration'
delete ', Debug'

comment '#\[derive(::serde.*'
comment '#\[prost.*'
comment '#\[serde.*'
comment 'pub r#type.*'
