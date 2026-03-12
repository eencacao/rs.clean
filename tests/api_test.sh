#!/usr/bin/env bash
set -e

PROJ="$(cd "$(dirname "$0")/.." && pwd)"
PORT=18080
BASE="http://127.0.0.1:$PORT"
PASS=0; FAIL=0

check() {
    local label=$1 got=$2 want=$3
    if [ "$got" = "$want" ]; then
        echo "PASS $label"; PASS=$((PASS+1))
    else
        echo "FAIL $label: got=$got want=$want"; FAIL=$((FAIL+1))
    fi
}

cd "$PROJ"
cargo build -q 2>&1

PORT=$PORT "$PROJ/target/debug/rs_clean" &
SRV_PID=$!
sleep 0.5

ST=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/todos")
check "GET /todos empty" "$ST" "200"

ST=$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "$BASE/todos" \
  -H "Content-Type: application/json" \
  -d '{"title":"test"}')
check "POST /todos" "$ST" "201"

curl -s -X POST "$BASE/todos" \
  -H "Content-Type: application/json" \
  -d '{"title":"second"}' > /dev/null

ST=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/todos/1")
check "GET /todos/1" "$ST" "200"

ST=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/todos/99")
check "GET /todos/99 not found" "$ST" "404"

ST=$(curl -s -o /dev/null -w "%{http_code}" \
  -X PUT "$BASE/todos/1" \
  -H "Content-Type: application/json" \
  -d '{"title":"updated","completed":true}')
check "PUT /todos/1" "$ST" "200"

ST=$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "$BASE/todos" \
  -H "Content-Type: application/json" \
  -d '{}')
check "POST /todos no title" "$ST" "400"

ST=$(curl -s -o /dev/null -w "%{http_code}" \
  -X DELETE "$BASE/todos/1")
check "DELETE /todos/1" "$ST" "204"

ST=$(curl -s -o /dev/null -w "%{http_code}" \
  -X DELETE "$BASE/todos/1")
check "DELETE /todos/1 not found" "$ST" "404"

kill "$SRV_PID" 2>/dev/null
echo ""
echo "Results: $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
