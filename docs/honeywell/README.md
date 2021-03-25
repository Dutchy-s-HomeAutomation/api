# Honeywell MyTotalConnectComform API
NOTE: This is reverse engineerde from Honeywell's website

## Login
Path: `https://international.mytotalconnectcomfort.com/api/accountApi/login`  
Method: `POST`  
  
Query parameters: None

Body:
```jsonc
{
    "EmailAddress": "LOGIN_EMAIL_HERE",         //The Email address of the user to log in as
    "Password": "LOGIN_PASSWORD_HERE",          //The password of the user to log in as
    "IsServiceStatusReturned": true,
    "ApiActive": true,
    "ApiDown": false,
    "RedirectUri": "",
    "events": [],
    "formErrors": []
}
```

Returns:
```jsonc
{
    "Content": {
        "UserId": "USER_ID_HER",                //ID of the logged in user
        "DisplayName": "USER_FIRST_NAME_HERE",  //Logged in user's first name
        "UserName": "USER_EMAIL_HERE",          //Logged in user's E-Mail address
        "LatestEulaAccepted": null,             //Unkown
        "AccessToken": "",                      //Guessing these are only used when the endpoint is access as an 'api'?
        "RefreshToken": "",                     //Guessing these are only used when the endpoint is access as an 'api'?
        "Reauthenticated": false,               //Unkown
        "RedirectUri": null,                    //Unkown
        "AuthorizationCode": null,              //Guessing these are only used when the endpoint is access as an 'api'?
        "GrantType": null,                      //Guessing these are only used when the endpoint is access as an 'api'?
        "ExpiresIn": null,                      //Guessing these are only used when the endpoint is access as an 'api'?
        "ResourceUri": null                     //Guessing these are only used when the endpoint is access as an 'api'?
    },
    "Errors": null,
    "RedirectUrl": "https://international.mytotalconnectcomfort.com/Locations",
    "CurrentCulture": null
}
```
The useful bits are in the returned Cookies:
```
SessionCookie: <YOUR SESSION ID>
RefreshCookie: <YOUR REFRESH TOKEN>
```

## Getting the User's TCC Locations
Path: `https://international.mytotalconnectcomfort.com/api/locationsapi/getlocations`  
Method: `GET`  

Query parameters: None

Body: None

Cookies:
```
SessionCookie: <YOUR SESSION ID>
```

Response:
```jsonc
{
    "Content": {
        "Locations": [
            {
                "Name": "LOCATION_NAME",                        //Name of the location
                "Id": "LOCATION_ID",                            //ID of the Location
                "SystemDeviceId": null,                         //Uknown
                "TimeOffset": 0,                                //Unkown, guessing timezone offset
                "HasGateways": true,                            //True if the Location has an EvoHome Gateway
                "HasTempControlSystem": false,                  //Unkown
                "HasZones": false,                              //This one makes no sense, it's false yet we get zones
                "IsDefault": false,                             //True if this Location is the default for the logged in user
                "City": "LOCATION_CITY",                        //Location's city
                "Country": "LOCATION_COUTRY",                   //Location's country, fully spelled out
                "CountryId": null,                              //Country ID, format unkown
                "Postcode": null,                               //Postal code
                "StreetAddress": null,                          //Address street 
                "OwnerName": null,                              //Name of the owner, format unknown
                "TimeZoneId": null,                             //Time Zone ID. Format unkown
                "TimeZoneDisplayName": null,                    //Display name of the timezone, guessing like 'UTC', 'CET' etc 
                "HeatingSystemType": 1,                         //Unkown
                "Type": 1,                                      //Unkown
                "Current": false,                               //True if the zone is currently selected by the user (doesn't really have any use when used as an API)
                "IsOwner": true,                                //True if the currently logged-in user is the owner of the Location
                "QuickActionStatus": null,                      //Unkown
                "IsChecked": false,                             //Unkown
                "SystemModesConfiguration": null,               //Unkown
                "FanModeStatus": null,                          //Unkown
                "LocationViewType": 0,                          //Unkown
                "SupportsDaylightSaving": false,                //True if the Location supports (or uses?) daylight savings
                "UseDaylightSavingSwitch": false,               //Unkown
                "AllActiveFaults": null,                        //Guessing this would be an array of Faults
                "AlertCount": 0,                                //Amount of alerts
                "HasCommLostSystemOrGatewayAlert": false,       //True if the Location has lost communication with the Gateway, or if the Gateway has an alert
                "HasSecuritySystem": true,                      //True if the Location has a security system
                "SecuritySystemId": null,                       //ID of the security system, guessing this is null because the User isn't logged into the security system
                "LocationDate": null,                           //Unkown
                "ShouldShowAdvertisement": false,               //True if Ads should be shown
                "SubscriptionEndDate": null,                    //End date of the subscription, Format unkown
                "Zones": [
                    {
                        "Id": "ZONE_ID_HERE",                   //The ID of the Zone
                        "DeviceId": 0000000,                    //The ID of the device, not sure what this means
                        "Name": "ZONE_NAME_HERE",               //The name of the zone
                        "MacId": "ZONE_MAC_ID_HERE",            //Thermostat's MAC address, without columns (':')
                        "ThermostatModelType": "Evo",           //Thermostat type
                        "IsAlive": true,                        //True if the device is alive
                        "HasAlerts": false,                     //True if the device has any alerts
                        "HasCommLostAlert": false,              //True if the device has lost communication with the EvoHome system
                        "HasBatteryLowAlert": false,            //True if the device's battery is almost dead
                        "HasSensorFailureAlert": false,         //True if the sensor has failed
                        "Temperature": 20,                      //Current temperature
                        "MinHeatSetpoint": 5,                   //Minimum allowed temperature
                        "MaxHeatSetpoint": 24,                  //Maximum allowed temperature
                        "MaxCoolSetpoint": 0,                   //Unknown, I'm guessing airconditioning related
                        "MinCoolSetpoint": 0,                   //Unknown, I'm guessing airconditioning related
                        "TargetHeatTemperature": 20,            //Target temperature
                        "TargetCoolTemperature": null,          //Unknown, I'm guessing airconditioning related
                        "SetpointDeadband": null,               //Unknown
                        "ThermostatType": 0,                    //Unknown
                        "OverrideActive": false,                //True if manually overriden (by phisically turning up the tmperature on the device)
                        "HoldTemperaturePermanently": false,    //True if while setting a temperature 'Permanent' was selected
                        "SetPointStatus": 0,                    //Unkown
                        "NextHeatSetPointTime": null,           //Unkown
                        "NextHeatSetPointTimeFormatted": null,  //Unkown
                        "DomesticHotWaterOn": 0,                //Unkown
                        "DomesticHotWaterState": 0,             //Unkown
                        "CurrentFanSetting": null,              //Unkown
                        "FanSettingCanBeChanged": null,         //Unkown
                        "AllowedFanSettings": null,             //Unkown
                        "AllowedThermostatModes": [             //Unkown
                            3,
                            4
                        ],
                        "ThermostatUnits": "Celsius",           //Temperature unit used by the thermostat
                        "ThermostatVersion": "EvoTouch"         //Theromstat device version
                    },
                ]
            }
        ]
    }
}
```

## Set a Zone's temperature
Path: `https://international.mytotalconnectcomfort.com/api/ZonesApi/SetZoneTemperature`  
Method: `POST`  

Query parameters: None

Body:
```jsonc
{
    "zoneId": "3967755",                //The ID of the Zone you want  to change
    "heatTemperature": "19.0",          //The target temperature 
    "hotWaterStateIsOn": false,         //Unkown
    "isPermanent": true,                //True if the new Temperature should be permanent
    "setUntilHours": "19",              //The hour that the Zone should revert back to schedule, for this isPermament needs to be false
    "setUntilMinutes": "00",            //The minute that the Zone should revert back to schedule, for this isPermanent needs to be false
    "locationTimeOffsetMinutes": 60,    //Unkown, 60 seems to work
    "isFollowingSchedule": false        //Unkown
}
```

Cookies:
```
SessionCookie: <YOUR SESSION ID>
```

Response:
```jsonc
{
    "Errors": null,                         //Contains Errors if something went wrong. Format unkown
    "RedirectUrl": null,                    //Unkown
    "ReauthenticatedAccessToken": null,     //Unkown
    "ReauthenticatedRefreshToken":null      //Unkown
}
```