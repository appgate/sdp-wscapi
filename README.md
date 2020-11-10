### Windows Service Center API wrapper
The following tool is a helper to access the security providers in Windows Security Center. This tool is used with a device script as a [extension](https://github.com/appgate/sdp-extensions) feature in Appgate SDP. 

For download see [latest release](https://github.com/appgate/sdp-wscapi/releases/latest).

### Providers are:
* Firewall products
* Anti virus products
* Anti spyware products

### The following platforms are supported
* Windows 8 and upwards until The Windows 10 Creators Update (also known as version 1703 and codenamed "Redstone 2")
* Desktop only x64

### Usage
```
$ ./agwscapi.exe
Usage: wscapi.exe [-av | -as | -fw]
   av: Query Antivirus programs
   as: Query Antispyware programs
   fw: Query Firewall programs
Exe build date: Mar 30 2020
Exe build time: 21:06:56
```

You can specify multiple flags 

### Example output multiple flags
```
$ ./agwscapi.exe -fw -av -as |jq
{
  "Antispyware": [
    {
      "product_name": "Windows Firewall",
      "product_state": "Off",
      "remediation_path": "NULL"
    },
    {
      "product_name": "Windows Defender",
      "product_state": "Off",
      "product_status": "Up-to-date",
      "remediation_path": "%ProgramFiles%\\Windows Defender\\MSASCui.exe"
    },
    {
      "product_name": "CylancePROTECT",
      "product_state": "On",
      "product_status": "Up-to-date",
      "remediation_path": "C:\\Program Files\\Cylance\\Desktop\\CylanceSvc.exe"
    },
    {
      "product_name": "Windows Defender",
      "product_state": "Off",
      "product_status": "Up-to-date",
      "remediation_path": "%ProgramFiles%\\Windows Defender\\MSASCui.exe"
    },
    {
      "product_name": "CylancePROTECT",
      "product_state": "On",
      "product_status": "Up-to-date",
      "remediation_path": "C:\\Program Files\\Cylance\\Desktop\\CylanceSvc.exe"
    }
  ],
  "Antivirus": [
    {
      "product_name": "Windows Firewall",
      "product_state": "Off",
      "remediation_path": "NULL"
    },
    {
      "product_name": "Windows Defender",
      "product_state": "Off",
      "product_status": "Up-to-date",
      "remediation_path": "%ProgramFiles%\\Windows Defender\\MSASCui.exe"
    },
    {
      "product_name": "CylancePROTECT",
      "product_state": "On",
      "product_status": "Up-to-date",
      "remediation_path": "C:\\Program Files\\Cylance\\Desktop\\CylanceSvc.exe"
    }
  ],
  "Firewall": [
    {
      "product_name": "Windows Firewall",
      "product_state": "Off",
      "remediation_path": "NULL"
    }
  ]
}
```

## Defined keywords
The following are possible output states (keywords) which you can expect from the wscapi.exe:

| field name                     | possible values                  |
|--------------------------------|----------------------------------|
| provider type                  | Firewall, Antispyware, Antivirus |
| `product_state`                | On, Off, Snoozed, Expired        |
| `product_status` for AV and AS | Up-to-date, Out-of-date          |





