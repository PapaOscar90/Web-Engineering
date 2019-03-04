# API Design
> Group 21

<!-- markdown-toc start -->
**Table of Contents**

- [API Design](#api-design)
    - [Introduction](#introduction)
    - [Dataset](#dataset)
    - [Requirements](#requirements)
    - [Endpoints](#Endpoints)
    - [Summary](#summary)
- [Appendix](#appendix)
    - [JSON Example Data](#json-example-data)

<!-- markdown-toc end -->


## Introduction
The aim of this document is to detail the design of a RESTful Web API allowing access to the data of the [CORGIS Airlines Dataset](https://think.cs.vt.edu/corgis/json/airlines/airlines.html). The requirements of the API are detailed in the [specification](specification.pdf) that was provided as part of the Web Engineering course material. 

It is important to note that this document will continue to evolve in the future. As the specification changes with added or modified requirements, this document will also change.

## Dataset
The first step in developing an API to access data, is to understand the structure of the data being accessed. The [CORGIS](https://think.cs.vt.edu/corgis/) project supplies a collection of datasets in a variety of formats. For the purposes of this project, the JSON version of the airlines dataset is considered authoritative.

The dataset consists of an array of objects, each of which represent one record of data. Each record specifies:
- an airport
```json
"airport": {
    "code": "ATL",
    "name": "Atlanta, GA: Hartsfield-Jackson Atlanta International"
}
```
- a carrier
```json
"carrier": {
    "code": "AA",
    "name": "American Airlines Inc."
}
```
- a time
```json
"time": {
    "label": "2003/6",
    "year": 2003,
    "month": 6
}
```

For this intersection the record also specifies additional statistics.
```json
"statistics": {
    "flights": {
        "cancelled": 5,
        "on time": 561,
        "total": 752,
        "delayed": 186,
        "diverted": 0
    },
    "# of delays": {
        "late aircraft": 18,
        "weather": 28,
        "security": 2,
        "national aviation system": 105,
        "carrier": 34
    },
    "minutes delayed": {
        "late aircraft": 1269,
        "weather": 1722,
        "carrier": 1367,
        "security": 139,
        "total": 8314,
        "national aviation system": 3817
    }
}
```

From considering the structure of the data, several points become apparent
1. As it stands the data does not enforce a hierarchical relationship between `airport`s, `carrier`s, `time`s, or `statistics`. 
2. The data does not provide enough information to accurately link flight's from one carrier between two airports.

## Requirements
The specification that was provided details the minimum requires for the API.
- All endpoints should support representing resources and receiving data in both JSON and CSV.
- All airports in the dataset should be retrievable.
- All carriers should be retrievable.
  - This should filterable by specifying a particular airport.
- All statistics for a carrier should be retrievable.
  - This should be filterable by specifying an airport or a month.
- New statistics for a carrier should be able to be added.
- Specific statistics for a carrier should be able to be modified.
- Specific statistics for a carrier should be able to be deleted.
- The number of on-time flights for a carrier should be retrievable
  - This should be filterable by specifying an airport or a month.
- The number of delayed flights for a carrier should be retrievable
  - This should be filterable by specifying an airport or a month.
- The number of cancelled flights for a carrier should be retrievable
  - This should be filterable by specifying an airport or a month.
- The number of minutes of delay for a carrier should be retrievable.
  - This should be filterable by specifying a reason (allowing for the filtration of carrier-specific reasons), an airport, or a month.
- Descriptive statistics for carrier-specific delays averaged between two airports.
  - This should be filterable by carrier.


## Endpoints
The requirement of supporting communication in JSON and CSV will be met by using the `Content-Type` header. A user of the API will specify that their request is in `application/json` or `text/csv`, and the API will respond accordingly. If the `Content-Type` is not specified, JSON is considered the default. As this requirement does not directly influence the underlying endpoint design, each endpoint should be considered to implicitly support both JSON and CSV.

All endpoints should be assumed to respond with status code 200 on success unless specified otherwise.

---
## `/airports`
This route supports retrieving all airports in the dataset.
##### GET
Return all airports within the dataset.
###### Sample result (JSON)
*NOTE* some results have been elided by `...`.
```json
[
    {
        "code": "ATL",
        "name": "Atlanta, GA: Hartsfield-Jackson Atlanta International"
    },
    {
        "code": "BOS",
        "name": "Boston, MA: Logan International"
    },
    {
        "code": "BWI",
        "name": "Baltimore, MD: Baltimore/Washington International Thurgood Marshall"
    },
    ...,
    {
        "code": "PHL",
        "name": "Philadelphia, PA: Philadelphia International"
    },
    {
        "code": "PHX",
        "name": "Phoenix, AZ: Phoenix Sky Harbor International"
    },
    {
        "code": "BOS",
        "name": "Boston, MA: Logan International"
    }
]

```

## `/carriers`
This route supports retrieving all carriers in the dataset.
##### GET
Return all carriers within the dataset.
###### Sample result (JSON)
*NOTE* some results have been elided by `...`.
```json
[
    {
        "code": "AA",
        "name": "American Airlines Inc."
    },
    {
        "code": "AS",
        "name": "Alaska Airlines Inc."
    },
    {
        "code": "B6",
        "name": "JetBlue Airways"
    },
    ...,
    {
        "code": "HA",
        "name": "Hawaiian Airlines Inc."
    },
    {
        "code": "VX",
        "name": "Virgin America"
    },
    {
        "code": "WN",
        "name": "Southwest Airlines Co."
    }
]
```

### `/carriers?<airport_code>`
This route supports retrieving all carriers in the dataset that operate at the airport specified by `<airport_code>`.
##### GET
This will return all carriers operating at the airport denoted by `<airport_code>`
###### Sample result (JSON)
```json
[
    {
        "code": "AA",
        "name": "American Airlines Inc."
    },
    {
        "code": "AS",
        "name": "Alaska Airlines Inc."
    },
    {
        "code": "CO",
        "name": "Continental Air Lines Inc."
    },
    ...,
    {
        "code": "UA",
        "name": "United Air Lines Inc."
    },
    {
        "code": "VX",
        "name": "Virgin America"
    },
    {
        "code": "WN",
        "name": "Southwest Airlines Co."
    }
]
```

## `/statistics`
This route supports retrieving and manipulating the statistics in the dataset. When retrieving the statistics, the required information about the carrier, the airport, and the time needed to uniquely identify the statistic is included.
##### GET
Return all statistics within the dataset.
###### Sample result (JSON)
*NOTE* some results have been elided by `...`.
```json
[
    {
        "airport": {
            "code": "ATL",
            "name": "Atlanta, GA: Hartsfield-Jackson Atlanta International"
        },
        "carrier": {
            "code": "AA",
            "name": "American Airlines Inc."
        },
        "statistics": {
            "flights": {
                "cancelled": 5,
                "delayed": 186,
                "diverted": 0,
                "on time": 561,
                "total": 752
            },
            "minutes delayed": {
                "carrier": 1367,
                "late aircraft": 1269,
                "national aviation system": 3817,
                "security": 139,
                "total": 8314,
                "weather": 1722
            },
            "# of delays": {
                "carrier": 34,
                "late aircraft": 18,
                "national aviation system": 105,
                "security": 2,
                "weather": 28
            }
        },
        "time": {
            "label": "2003/6",
            "month": 6,
            "year": 2003
        }
    },
    ...,
    {
        "airport": {
            "code": "BOS",
            "name": "Boston, MA: Logan International"
        },
        "carrier": {
            "code": "WN",
            "name": "Southwest Airlines Co."
        },
        "statistics": {
            "flights": {
                "cancelled": 27,
                "delayed": 137,
                "diverted": 0,
                "on time": 740,
                "total": 904
            },
            "minutes delayed": {
                "carrier": 1894,
                "late aircraft": 2738,
                "national aviation system": 1300,
                "security": 0,
                "total": 6264,
                "weather": 332
            },
            "# of delays": {
                "carrier": 48,
                "late aircraft": 55,
                "national aviation system": 32,
                "security": 0,
                "weather": 2
            }
        },
        "time": {
            "label": "2016/1",
            "month": 1,
            "year": 2016
        }
    }
]
```
##### POST
Add a new statistic to the dataset. The request requires the data in the request body to specify the carrier, the airport, and the time alongside the statistics.

Alongside the 200 status code, the server should respond with a payload of the statistic that was just added.

### `/statistics?<carrier_code>&<airport_code>&<month>`
Return all statistics within the dataset filtered by the carrier identified by the `<carrier_code>`, the airport identified by the `<airport_code>`, and the month corresponding to the `<month>`.

Each of these filters is optional and when omitted the data is not filtered by that omitted value (the full range for that value is returned**.
##### GET
The statistics as filtered by the provided query parameters.

### <code>/statistics?<i><u><carrier_code>&<airport_code>&\<month>&\<year></u></i></code>

Interact with statistics in the dataset as identified by `<carrier_code>`, `<airport_code>`, `<month>`, and `<year>`.

This represents the endpoint where the query parameters are required. This is because the statistic must be fully identified in order to modify or delete it.
##### GET
The statistics as filtered by the provided query parameters.
##### PUT
The statistics identified by the query parameters is updated with the statics provided in the request body. The request body need only include the statistics and not any information on the airport, carrier, or time.
##### PATCH
The statistics identified by the query parameters is updated with the statics provided in the request body. The request body need only include the changed statistics and not any information on the airport, carrier, or time.

*NOTE* best practices involve making use of the JSON patch format. We are uncertain of how this should be handled in the case of the `Content-Type` header being `text/csv`. There does not appear to be a CSV patch format that's specified for use in REST APIs. 
##### DELETE
Delete the statistic specified by the `<carrier_code>`, `<airport_code>`, `<month>`, and `<year>`.

The server should respond with a 204 status code on success.

## `/statistics/on-time`
Get the statistics on the number of on-time flights.
##### GET
Return the statistics on the number of on-time flights.
### `/statistics/on-time?<carrier_code>&<airport_code>&<month>`
Get the statistics on the number of on-time flights where the carrier, the airport, and the month may be specified.
##### GET
Return the statistics on the number of on-time flights filtered by the specified carrier, airport, and month if specified.

## `/statistics/delayed`
Get the statistics on the number of delayed flights.
##### GET
Return the statistics on the number of delayed flights.
### `/statistics/delayed?<carrier_code>&<airport_code>&<month>`
Get the statistics on the number of delayed flights where the carrier, the airport, and the month may be specified.
##### GET
Return the statistics on the number of on-time flights filtered by the specified carrier, airport, and month if specified.

## `/statistics/cancelled`
Get the statistics on the number of cancelled flights.
##### GET
Return the statistics on the number of cancelled flights.
### `/statistics/cancelled?<carrier_code>&<airport_code>&<month>`
Get the statistics on the number of cancelled flights where the carrier, the airport, and the month may be specified.
##### GET
Return the statistics on the number of cancelled flights filtered by the specified carrier, airport, and month if specified.


## `/statistics/minutes_delayed`
Get the statistics on the minutes delayed.
##### GET
Return the statistics on the number of minutes delayed.
### `/statistics/minutes_delayed?<carrier_code>&<airport_code>&<month>&<reason>`
Get the statistics on the minutes delayed filtered by `<carrier_code>`, `<airport_code>`, `<month>`, and `<reason>`. The reason corresponds with one of the following values: "late aircraft", "weather", "carrier", "security", "total", or "national aviation system". Multiple reason parameters may be passed to include more reasons. If the reason parameter is not set, the minutes delayed for all reasons are returned.
##### GET
Return the statistics on the number of minutes delayed as filtered by the provided query parameters.

## Summary
- /airports
- /carriers
- /carriers?airport=<airport_code>
- /statistics?carrier=<carrier_code>&airport=<airport_code>&month=<month>
- /statistics/on-time?carrier=<carrier_code>&airport=<airport_code>&month=<month>
- /statistics/delayed?carrier=<carrier_code>&airport=<airport_code>&month=<month>
- /statistics/cancelled?carrier=<carrier_code>&airport=<airport_code>&month=<month>
- /statistics/delayed/number_of_minutes?**carrier=<carrier_code**>&airport=<airport_code>&month=<month>&reason=\<reason\> 


# Appendix
## JSON Example Data
```json
[
    {
        "airport": {
            "code": "ATL",
            "name": "Atlanta, GA: Hartsfield-Jackson Atlanta International"
        },
        "statistics": {
            "flights": {
                "cancelled": 5,
                "on time": 561,
                "total": 752,
                "delayed": 186,
                "diverted": 0
            },
            "# of delays": {
                "late aircraft": 18,
                "weather": 28,
                "security": 2,
                "national aviation system": 105,
                "carrier": 34
            },
            "minutes delayed": {
                "late aircraft": 1269,
                "weather": 1722,
                "carrier": 1367,
                "security": 139,
                "total": 8314,
                "national aviation system": 3817
            }
        },
        "time": {
            "label": "2003/6",
            "year": 2003,
            "month": 6
        },
        "carrier": {
            "code": "AA",
            "name": "American Airlines Inc."
        }
    },
    {
        "airport": {
            "code": "BOS",
            "name": "Boston, MA: Logan International"
        },
        "statistics": {
            "flights": {
                "cancelled": 7,
                "on time": 1034,
                "total": 1266,
                "delayed": 225,
                "diverted": 0
            },
            "# of delays": {
                "late aircraft": 46,
                "weather": 24,
                "security": 2,
                "national aviation system": 84,
                "carrier": 69
            },
            "minutes delayed": {
                "late aircraft": 3043,
                "weather": 1783,
                "carrier": 4201,
                "security": 45,
                "total": 12139,
                "national aviation system": 3067
            }
        },
        "time": {
            "label": "2003/6",
            "year": 2003,
            "month": 6
        },
        "carrier": {
            "code": "AA",
            "name": "American Airlines Inc."
        }
    }
]
```


