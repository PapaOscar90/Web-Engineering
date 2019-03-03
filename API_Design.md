# Reworked Routes
- /airports
- /carriers
- /carriers?airport=<airport_code>
- /statistics?carrier=<carrier_code>&airport=<airport_code>&month=<month_number>
- /statistics/on_time?carrier=<carrier_code>&airport=<airport_code>&month=<month_number>
- /statistics/delayed?carrier=<carrier_code>&airport=<airport_code>&month=<month_number>
- /statistics/cancelled?carrier=<carrier_code>&airport=<airport_code>&month=<month_number>
- /statistics/delayed/number_of_minutes?**carrier=<carrier_code**>&airport=<airport_code>&month=<month_number>&reason=\<reason\> 



# API Design
The aim of this document is to detail the design of a RESTful Web API allowing access to the data of the [CORGIS Airlines Dataset](https://think.cs.vt.edu/corgis/json/airlines/airlines.html). The requirements of the API are detailed in the [specification](specification.pdf) that was provided as part of the Web Engineering course material. The design was developed and is documented as follows:
1. Familiarization with the content and structure of the dataset.
2. Discussion and elaboration on the requirements of the Web API.
3. Map requirements to routes, providing motivation and specifying potential alternatives.
4. Summary of the resulting API design to ease implementation detail.

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

For this intersection the record also specifies some statistics.
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


# Endpoints
All endpoints will support:
- application/json
- text/csv

The default serve and recieve is JSON query. The user of the endpoint specifies the media type via the content type header in their http request.

---
## /airports/
##### GET
Return all airports within the USA


##### POST
Add an airport to the list of airports

### /airports?carrier=<carrier_id>
##### GET
This will query the airports for all airports serving <carrier_id>

## /carriers/
##### GET
Return all airports within the USA


##### POST
Add an airport to the list of airports


### /carriers?airport=<airport_id>
##### GET
This will query the carriers for all carriers operating at <airport_id>.

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
