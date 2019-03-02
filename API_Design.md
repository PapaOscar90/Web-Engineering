# API Design
Description here

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

