### Windows Service Center API wrapper
The following tool is a helper to access the security providers in Windows Security Center. This tool is used with a device script as a [extension](https://github.com/appgate/sdp-extensions) feature in Appgate SDP.

For download see [latest release](https://github.com/appgate/sdp-wscapi/releases/latest).

### Providers are:
* Firewall products
* Anti virus products
* Anti spyware products

### Platform support
* Binary should run on Windows 10 and upwards, x64 only
* Desktop only

Windows is moving to a [Windows Defender Advanced Threat Protection platform (name and product might have changed already)](https://docs.microsoft.com/en-us/windows/security/threat-protection/microsoft-defender-atp/microsoft-defender-advanced-threat-protection) so this is will be superseeded and the program rendered unusable. The program will let you know when running it.

### Usage
```
$ ./agwsc.exe
Usage: agwsc.exe [-av | -as | -fw]
   av: Query Antivirus programs
   as: Query Antispyware programs
   fw: Query Firewall programs
```

You can specify multiple flags

### Example output multiple flags
```
$ ./agwsc.exe -fw -av -as |jq
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
      "remediation_path": "%ProgramFiles%\\Windows Defender\\MSASCui.exe",
      "product_state_timestamp":"Mon, 09 Jan 2023 19:19:12 GMT"
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
| `product_state_timestamp`      | A timestamp                      |





