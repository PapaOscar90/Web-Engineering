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

# Introduction
The aim of this document is to detail the design of a RESTful Web API allowing access to the data of the [CORGIS Airlines Dataset](https://think.cs.vt.edu/corgis/json/airlines/airlines.html). The requirements of the API are detailed in the [specification](specification.pdf) that was provided as part of the Web Engineering course material. 

It is important to note that this document will continue to evolve in the future. As the specification changes with added or modified requirements, this document will also change. Please also refer to the Architecture document for details about the technology and high level architecture driving this amazing web app.

# Dataset
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

By considering the structure of the data it is apparent the data does not enforce a hierarchical relationship between `airport`s, `carrier`s, `time`s, or `statistics`. This is critical to consider as it influences the choice of using query parameters or path segments to specify data. According to [rfc3986](https://tools.ietf.org/html/rfc3986#section-3.4), query parameters may only be used to represent non-hierarchical data.

# Requirements
The specification that was provided details the minimum requires for the API.
- [x] All endpoints should support representing resources and receiving data in both JSON and CSV.
- [x] All airports in the dataset should be retrievable.
- [x] All carriers should be retrievable.
  - [x] This should filterable by specifying a particular airport.
- [x] All statistics for a carrier should be retrievable.
  - [x] This should be filterable by specifying an airport or a month.
- [x] New statistics for a carrier should be able to be added.
- [x] Specific statistics for a carrier should be able to be modified.
- [x] Specific statistics for a carrier should be able to be deleted.
- [x] The number of on-time flights for a carrier should be retrievable
  - [x] This should be filterable by specifying an airport or a month.
- [x] The number of delayed flights for a carrier should be retrievable
  - [x] This should be filterable by specifying an airport or a month.
- [x] The number of cancelled flights for a carrier should be retrievable
  - [x] This should be filterable by specifying an airport or a month.
- [x] The number of minutes of delay for a carrier should be retrievable.
  - [x] This should be filterable by specifying a reason (allowing for the filtration of carrier-specific reasons), an airport, or a month.
- [x] Descriptive statistics for carrier-specific delays averaged between two airports.
  - [x] This should be filterable by carrier.

# Endpoints
The requirement of supporting communication in JSON and CSV will be met by using the `Content-Type` header. A user of the API will specify that their request is in `application/json` or `text/csv`, and the API will respond accordingly. If the `Content-Type` is not specified, JSON is considered the default. As this requirement does not directly influence the underlying endpoint design, each endpoint should be considered to implicitly support both JSON and CSV.

Values included in `{}` are query parameters. For example `{airport}` is actually entered as `airport=<airport-code>` where '\<airport-code\>' is the three letter identifier of the airport.

All endpoints should be assumed to respond with status code 200 on success unless specified otherwise.

Responses in text/csv are differing in only their apearence. Since it would be a waste of time and space to copy paste the same data over and over again, we will provide the json output, and assume the reader can read json. If examples are needed, the user can use the emacs restclient on the file: [../backend/request_examples.http](../backend/request_examples.http). This file automatically creates `CURL` requests to the server and outputs within a buffer of emacs. Example responses for `JSON` and `text/csv` for comparison:
- an airport
```json
"airport": {
    "code": "ATL",
    "name": "Atlanta, GA: Hartsfield-Jackson Atlanta International"
}
```
```text/csv
code,name
ATL,"Atlanta, GA: Hartsfield-Jackson Atlanta International"
```
- a carrier
```json
"carrier": {
    "code": "AA",
    "name": "American Airlines Inc."
}
```
```text/csv
code,name
AA,"American Airlines Inc."
```
- a time
```json
"time": {
    "label": "2003/6",
    "year": 2003,
    "month": 6
}
```
```text/csv
label,year,month
2003/6,2003,6
```
Notice the order and data is exactly the same. So a user can easily read the csv by reading the data in the json, without any extra punctuation besides a comma seperating items, and newlines seperating objects.

---
## `/airports`
This route supports retrieving all airports in the dataset.
##### GET
Return all airports within the dataset.
###### Sample result (JSON)
*NOTE* some results have been elided by `...`.
```json
{
    _links: {
        self: {href: "{URL}/airports"},
        item: [
            { href: "{URL}/airports/ATL" },
            { href: "{URL}/airports/BOS" },
            { href: "{URL}/airports/BWI" },
            ...,
            { href: "{URL}/airports/PHL" },
            { href: "{URL}/airports/PHX" },
            { href: "{URL}/airports/BOS" }
        ]
    },
    data: [
        {
            code: "ATL",
            name: "Atlanta, GA: Hartsfield-Jackson Atlanta International",
            _links: { href: "{URL}/airports/ATL" }
        },
        {
            code: "BOS",
            name: "Boston, MA: Logan International",
            _links: { href: "{URL}/airports/BOS" }
        },
        {
            code: "BWI",
            name: "Baltimore, MD: Baltimore/Washington International Thurgood Marshall",
            _links: { href: "{URL}/airports/BWI" }
        },
        ...,
        {
            code: "PHL",
            name: "Philadelphia, PA: Philadelphia International",
            links: { href: "{URL}/airports/PHL" }
        },
        {
            code: "PHX",
            name: "Phoenix, AZ: Phoenix Sky Harbor International",
            links: { href: "{URL}/airports/PHX" }
        },
        {
            code: "BOS",
            name: "Boston, MA: Logan International",
            links: { href: "{URL}/airports/BOS" }
        }
    ]
}
```

## `/carriers`
This route supports retrieving all carriers in the dataset.
##### GET
Return all carriers within the dataset.
###### Sample result (JSON)
*NOTE* some results have been elided by `...`.
```json
{
    _links: {
        self: { href: "{URL}/carriers" },
        item: [
            { href: "{URL}/carriers/AA" },
            { href: "{URL}/carriers/AS" },
            { href: "{URL}/carriers/B6" },
            ...,
            { href: "{URL}/carriers/HA" },
            { href: "{URL}/airports/VX" },
            { href: "{URL}/airports/WN" }
        ]
    },
    data: [
        {
            code: "AA",
            name: "American Airlines Inc.",
            _links: { href: "{URL}/carriers/AA" }
        },
        {
            code: "AS",
            name: "Alaska Airlines Inc.",
            _links: { href: "{URL}/carriers/AS" }
        },
        {
            code: "B6",
            name: "JetBlue Airways",
            _links: { href: "{URL}/carriers/B6" }
        },
        ...,
        {
            code: "HA",
            name: "Hawaiian Airlines Inc.",
            _links: { href: "{URL}/carriers/HA" }
        },
        {
            code: "VX",
            name: "Virgin America",
            _links: { href: "{URL}/airports/VX" }
        },
        {
            code: "WN",
            name: "Southwest Airlines Co.",
            _links: { href: "{URL}/airports/WN" }
        }
    ]
}
```

### `/carriers?{airport_code}`
This route supports retrieving all carriers in the dataset that operate at the airport specified by `{airport_code}`.
##### GET
This will return all carriers operating at the airport denoted by `{airport_code}`
###### Sample result (JSON)
```json
{
    _links: {
        self: { href: "{URL}/carriers?airport_code=PHL" },
        item: [
            { href: "{URL}/carriers/AA" },
            { href: "{URL}/carriers/AS" },
            { href: "{URL}/carriers/CO" },
            ...,
            { href: "{URL}/carriers/UA" },
            { href: "{URL}/airports/VX" },
            { href: "{URL}/airports/WN" }
        ]
    },
    data: [
        {
            code: "AA",
            name: "American Airlines Inc.",
            _links: { href: "{URL}/carriers/AA" }
        },
        {
            code: "AS",
            name: "Alaska Airlines Inc.",
            _links: { href: "{URL}/carriers/AS" }
        },
        {
            code: "CO",
            name: "Continental Air Lines Inc.",
            _links: { href: "{URL}/carriers/CO" }
        },
        ...,
        {
            code: "UA",
            name: "United Air Lines Inc.",
            _links: { href: "{URL}/carriers/UA" }
        },
        {
            code: "VX",
            name: "Virgin America",
            _links: { href: "{URL}/airports/VX" }
        },
        {
            code: "WN",
            name: "Southwest Airlines Co.",
            _links: { href: "{URL}/airports/WN" }
        }
    ]
}
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

### `/statistics?{carrier_code}&{airport_code}&{month}`
Return all statistics within the dataset filtered by the carrier identified by the `{carrier_code}`, the airport identified by the `{airport_code}`, and the month corresponding to the `{month}`.

Each of these filters is optional and when omitted the data is not filtered by that omitted value (the full range for that value is returned**.
##### GET
The statistics as filtered by the provided query parameters.

### <code>/statistics?<i><u>{carrier_code}&{airport_code}&{month}&{year}</u></i></code>

Interact with statistics in the dataset as identified by `{carrier_code}`, `{airport_code}`, `{month}`, and `{year}`.

This represents the endpoint where the query parameters are required. This is because the statistic must be fully identified in order to modify or delete it.
##### GET
The statistics as filtered by the provided query parameters.
##### PUT
The statistics identified by the query parameters is updated with the statics provided in the request body. The request body need only include the statistics and not any information on the airport, carrier, or time.
##### PATCH
The statistics identified by the query parameters is updated with the statics provided in the request body. The request body need only include the changed statistics and not any information on the airport, carrier, or time.

*NOTE* best practices involve making use of the JSON patch format. We are uncertain of how this should be handled in the case of the `Content-Type` header being `text/csv`. There does not appear to be a CSV patch format that's specified for use in REST APIs. 
##### DELETE
Delete the statistic specified by the `{carrier_code}`, `{airport_code}`, `{month}`, and `{year}`.

The server should respond with a 204 status code on success.

## `/statistics/flights`
Get the statistics on the number of on-time flights.
##### GET
Return the statistics on the number of on-time flights.
###### Sample response (JSON)
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
    "flights": {
      "cancelled": 5,
      "delayed": 186,
      "diverted": 0,
      "on time": 561,
      "total": 752
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
      "code": "ATL",
      "name": "Atlanta, GA: Hartsfield-Jackson Atlanta International"
    },
    "carrier": {
      "code": "AA",
      "name": "American Airlines Inc."
    },
    "flights": {
      "cancelled": 7,
      "delayed": 210,
      "diverted": 1,
      "on time": 556,
      "total": 774
    },
    "time": {
      "label": "2003/7",
      "month": 7,
      "year": 2003
    }
  }
]
```
## `/statistics/minutes-delayed`
Get the statistics on the number of delayed flights.
##### GET
Return the statistics on the number of delayed flights.
###### Sample response (text/csv)
Since the reponse is similar to the previous, we show the `text/csv` output here:
```
Airport.Code,Airport.Name,Carrier.Code,Carrier.Name,Minutes Delayed.Carrier,Minutes Delayed.Late Aircraft,Minutes Delayed.National Aviation System,Minutes Delayed.Security,Minutes Delayed.Total,Minutes Delayed.Weather,Time.Label,Time.Month,Time.Year
ATL,"Atlanta, GA: Hartsfield-Jackson Atlanta International",AA,American Airlines Inc.,1367,1367,1367,1367,1367,1722,2003/6,6,2003
BOS,"Boston, MA: Logan International",AA,American Airlines Inc.,4201,4201,4201,4201,4201,1783,2003/6,6,2003
BWI,"Baltimore, MD: Baltimore/Washington International Thurgood Marshall",AA,American Airlines Inc.,1058,1058,1058,1058,1058,1332,2003/6,6,2003
CLT,"Charlotte, NC: Charlotte Douglas International",AA,American Airlines Inc.,968,968,968,968,968,742,2003/6,6,2003
DCA,"Washington, DC: Ronald Reagan Washington National",AA,American Airlines Inc.,2048,2048,2048,2048,2048,1484,2003/6,6,2003
DEN,"Denver, CO: Denver International",AA,American Airlines Inc.,2098,2098,2098,2098,2098,1193,2003/6,6,2003
DFW,"Dallas/Fort Worth, TX: Dallas/Fort Worth International",AA,American Airlines Inc.,31999,31999,31999,31999,31999,4787,2003/6,6,2003
DTW,"Detroit, MI: Detroit Metro Wayne County",AA,American Airlines Inc.,1029,1029,1029,1029,1029,791,2003/6,6,2003
...
```

### `/statistics/minutes-delayed?{carrier_code}&{airport_code}&{month}&{reason}`
Get the statistics on the minutes delayed filtered by `{carrier_code}`, `{airport_code}`, `{month}`, and `{reason}`. The reason corresponds with one of the following values: "late aircraft", "weather", "carrier", "security", "total", or "national aviation system". Multiple reason parameters may be passed to include more reasons. If the reason parameter is not set, the minutes delayed for all reasons are returned.
##### GET
Return the statistics on the number of minutes delayed as filtered by the provided query parameters.

## <code>/statistics/connection?<i><u>{airport_1_code}&{airport_2_code}</i></u></code>
*NOTE* the name of this endpoint is under evaluation.

Represents the descriptive statistics (mean, median, standard deviation, etc...) of the averages between 2 airports. The query parameters for the two airports are required. What precise statistics have not been specified in the provided specification, so this route will return these averages and the descriptive statistics for all the `statistics` contained in the dataset.

##### GET
Return the descriptive statistics between the two provided airports.

### <code>/statistics/connection?<i><u>{airport_1_code}&{airport_2_code}</i></u>&{carrier}</code>
*NOTE* the name of this endpoint is under evaluation.

Allows for retrieval of the descriptive statistics filtered by a carrier.

##### GET
Return the descriptive statistics between the two provided airports specific to a carrier.

# Summary
The following table summarizes the routes that are to be created. Mandatory query parameters are in **bold**.

| Endpoint                                                                               | HTTP Verbs |
|----------------------------------------------------------------------------------------|------------|
| `/airports`                                                                            | GET        |
| `/carriers`                                                                            | GET        |
| `/carriers?{airport_code}`                                                             | GET        |
| `/statistics`                                                                          | GET, POST  |
| `/statistics?{carrier_code}&{airport_code}&{month}`                                    | GET        |
| <code>/statistics?<b>{carrier_code}&{airport_code}&\{month}&\{year}</b></code>         | GET,       |
| `/statistics/flights`                                                                  | GET        |
| `/statistics/flights?{carrier_code}&{airport_code}&{month}`                            | GET        |
| `/statistics/minutes-delayed`                                                          | GET        |
| `/statistics/minutes-delayed?{carrier_code}&{airport_code}&{month}&{reason}`           | GET        |
| <code>/statistics/connection?<b>{airport_1_code}&{airport_2_code}</b></code>           | GET        |
| <code>/statistics/connection?<b>{airport_1_code}&{airport_2_code}</b>&{carrier}</code> | GET        |

# Appendix
### JSON Example Data
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


