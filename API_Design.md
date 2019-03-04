# API Design

<!-- markdown-toc start -->
**Table of Contents**

- [API Design](#api-design)
    - [Introduction](#introduction)
    - [Dataset](#dataset)
    - [Requirements](#requirements)
    - [Endpoints](#Endpoints)
    - [Summary](#summary)
- [Appendix](#appendix)
    - [Truncated JSON Data](#truncated-json-data)
- [Reworked Routes](#reworked-routes)

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

---
## /airports/
This route supports retrieving all airports in the dataset.
##### GET
Return all airports within the dataset.
###### Sample result (JSON)
**NOTE** some results have been elided by `...`.
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

## /carriers/
This route supports retrieving all carriers in the dataset.
##### GET
Return all carriers within the dataset.
###### Sample result (JSON)
**NOTE** some results have been elided by `...`.
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

### /carriers?<airport_code>
This route supports retrieving all carriers in the dataset that operating at the airport specified by `<airport_code>`.
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

### /carriers/statistics?reason=<reason_name>&month=<month_number>&airport=<airport_id>
##### GET
###### reason=<reason_name>
This can be a singular reason, a set, or a name that refers to a set of carrier specific. If empty, returns all.
###### month=<month_number>
This designates the month which should be filtered on the month field.
###### airport=<airport_id>
This designates the airport which should be filtered on the airport field (aka, should this airport).

### /carriers/statistics/delays?from=<airport_id>&to=<airport_id>&carrier=<carrier_id>
##### GET
###### from=<airport_id>
This designates the airport which should be filtered on the from field.
###### to=<airport_id>
This designates the airport which should be filtered on the to field.
###### carrier=<carrier_id>
This designates the airport which should be filtered on the carrier field.

### /carriers/<carrier_id>/statistics?from=<airport_id>&to=<airport_id>&month=<month_number>
##### GET
This will query the statistics of a carrier. The query strings are optional, if they are not provided the data is not filtered.
###### from=<airport_id>
This designates the airport which should be filtered on the from field.
###### to=<airport_id>
This designates the airport which should be filtered on the to field.
###### month=<month_number>
This designates the month which should be filtered on the month field.

### /carriers/<carrier_id>/on-time?from=<airport_id>&to=<airport_id>&month=<month_number>
##### GET
This will query the statistics of a carrier. The query strings are optional, if they are not provided the data is not filtered.
###### from=<airport_id>
This designates the airport which should be filtered on the from field.
###### to=<airport_id>
This designates the airport which should be filtered on the to field.
###### month=<month_number>
This designates the month which should be filtered on the month field.

### /carriers/<carrier_id>/delayed?from=<airport_id>&to=<airport_id>&month=<month_number>
##### GET
This will query the statistics of a carrier. The query strings are optional, if they are not provided the data is not filtered.
###### from=<airport_id>
This designates the airport which should be filtered on the from field.
###### to=<airport_id>
This designates the airport which should be filtered on the to field.
###### month=<month_number>
This designates the month which should be filtered on the month field.

### /carriers/<carrier_id>/cancelled?from=<airport_id>&to=<airport_id>&month=<month_number>
##### GET
This will query the statistics of a carrier. The query strings are optional, if they are not provided the data is not filtered.
###### from=<airport_id>
This designates the airport which should be filtered on the from field.
###### to=<airport_id>
This designates the airport which should be filtered on the to field.
###### month=<month_number>
This designates the month which should be filtered on the month field.

## Summary


# Appendix
## Truncated JSON Data
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

# Reworked Routes
- /airports
- /carriers
- /carriers?airport=<airport_code>
- /statistics?carrier=<carrier_code>&airport=<airport_code>&month=<month_number>
- /statistics/on_time?carrier=<carrier_code>&airport=<airport_code>&month=<month_number>
- /statistics/delayed?carrier=<carrier_code>&airport=<airport_code>&month=<month_number>
- /statistics/cancelled?carrier=<carrier_code>&airport=<airport_code>&month=<month_number>
- /statistics/delayed/number_of_minutes?**carrier=<carrier_code**>&airport=<airport_code>&month=<month_number>&reason=\<reason\> 


