use super::*;

#[test]
fn test_record_from_object() {
    let data = r##"
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
       }"##;

    let expected = Record {
        airport: Airport {
            code: "ATL".to_string(),
            name: "Atlanta, GA: Hartsfield-Jackson Atlanta International".to_string(),
        },
        carrier: Carrier {
            code: "AA".to_string(),
            name: "American Airlines Inc.".to_string(),
        },
        statistics: Statistics {
            flights: Flights {
                cancelled: 5,
                delayed: 186,
                diverted: 0,
                on_time: 561,
                total: 752,
            },
            minutes_delayed: MinutesDelayed {
                carrier: 1367,
                late_aircraft: 1269,
                national_aviation_system: 3817,
                security: 139,
                total: 8314,
                weather: 1722,
            },
            number_of_delays: NumberOfDelays {
                carrier: 34,
                late_aircraft: 18,
                national_aviation_system: 105,
                security: 2,
                weather: 28,
            },
        },
        time: Time {
            label: "2003/6".to_string(),
            month: 6,
            year: 2003,
        },
    };
    let actual: Record = serde_json::from_str(data).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_data_store_from_object() {
    let data = r##"
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
]"##;

    let actual: DataSet = serde_json::from_str(data).unwrap();
    let expected = DataSet {
        records: vec![
            Record {
                airport: Airport {
                    code: "ATL".to_string(),
                    name: "Atlanta, GA: Hartsfield-Jackson Atlanta International".to_string(),
                },
                carrier: Carrier {
                    code: "AA".to_string(),
                    name: "American Airlines Inc.".to_string(),
                },
                statistics: Statistics {
                    flights: Flights {
                        cancelled: 5,
                        delayed: 186,
                        diverted: 0,
                        on_time: 561,
                        total: 752,
                    },
                    minutes_delayed: MinutesDelayed {
                        carrier: 1367,
                        late_aircraft: 1269,
                        national_aviation_system: 3817,
                        security: 139,
                        total: 8314,
                        weather: 1722,
                    },
                    number_of_delays: NumberOfDelays {
                        carrier: 34,
                        late_aircraft: 18,
                        national_aviation_system: 105,
                        security: 2,
                        weather: 28,
                    },
                },
                time: Time {
                    label: "2003/6".to_string(),
                    month: 6,
                    year: 2003,
                },
            },
            Record {
                airport: Airport {
                    code: "BOS".to_string(),
                    name: "Boston, MA: Logan International".to_string(),
                },
                carrier: Carrier {
                    code: "AA".to_string(),
                    name: "American Airlines Inc.".to_string(),
                },
                statistics: Statistics {
                    flights: Flights {
                        cancelled: 7,
                        delayed: 225,
                        diverted: 0,
                        on_time: 1034,
                        total: 1266,
                    },
                    minutes_delayed: MinutesDelayed {
                        carrier: 4201,
                        late_aircraft: 3043,
                        national_aviation_system: 3067,
                        security: 45,
                        total: 12139,
                        weather: 1783,
                    },
                    number_of_delays: NumberOfDelays {
                        carrier: 69,
                        late_aircraft: 46,
                        national_aviation_system: 84,
                        security: 2,
                        weather: 24,
                    },
                },
                time: Time {
                    label: "2003/6".to_string(),
                    month: 6,
                    year: 2003,
                },
            },
        ],
    };

    assert_eq!(expected, actual);
}

#[test]
fn test_data_store_new() {
    let data_store = DataSet::new();
    let expected = 54013;
    let actual = data_store.records.len();
    assert_eq!(expected, actual);
}
