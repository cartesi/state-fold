#!/bin/bash

get_package_identifier() {
    local query=${1:-""}
    cloudsmith list packages --output-format json --query "$query" cartesi/main 2> /dev/null | jq '.data[0].slug' | sed -e 's/^"//' -e 's/"$//'
}

query=${1:-""}
total_time_limit=${2:-180}
identifier=""
package_status=""
total_time=0
package_sync_complete=1
package_sync_failed=0
sleep_time=10

while [[ $total_time -lt $total_time_limit ]]; do
    if [[ -z "$identifier" ]] || [[ "$identifier" == "null" ]]; then
        identifier=$(get_package_identifier "$query")
        if [[ -z "$identifier" ]] || [[ "$identifier" == "null" ]]; then
          echo "Waiting for package .. (query: $query)" > /dev/stderr
          total_time=$((total_time+$sleep_time))
          sleep $sleep_time
          continue
        fi
    fi

    package_status=$(cloudsmith status "cartesi/main/$identifier" 2> /dev/null)

    echo "$package_status" | grep --quiet 'Completed'
    package_sync_complete=$?

    echo "$package_status" | grep --quiet 'Failed'
    package_sync_failed=$?

    if [[ $package_sync_complete -eq 0 ]] || [[ $package_sync_failed -eq 0 ]]; then
        break
    fi

    echo "Waiting for package status ... (identifier: $identifier)" > /dev/stderr
    total_time=$((total_time+$sleep_time))
    sleep $sleep_time
done

if [[ $total_time -gt $total_time_limit ]]; then
    echo "Timed out after waiting $total_time seconds for package to sync" > /dev/stderr
    exit 1
fi

if [[ $package_sync_complete -ne 0 ]]; then
  echo "Package failed to sync after $total_time seconds" > /dev/stderr
  exit 1
fi

echo "Package synced successfully after $total_time seconds" > /dev/stderr
