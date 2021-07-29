# Start geth
geth --dev --dev.period 3 --http --http.api eth,net,web3 --ws --ws.api eth,net,web3 >>/dev/null 2>&1 &
pid=$!
sleep 3

# Run delegate example
cargo run --bin delegate_example

# kill geth
kill "$pid"
sleep 5
