# Start geth
geth --dev --dev.period 3 --http --http.api eth,net,web3 --ws --ws.api eth,net,web3 >>/dev/null 2>&1 &
pid=$!
sleep 3

mode="Array"

usage() { echo "Usage: $0 [-m <string>]" 1>&2; exit 1; }

while getopts m: flag
do
    case "${flag}" in
        m) mode=${OPTARG};;
	*) usage;;
    esac
done

# Run delegate example
cargo run --bin delegate_example $mode

# kill geth
kill "$pid"
sleep 5
