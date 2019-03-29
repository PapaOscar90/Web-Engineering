open Belt;

let apiBaseUrl = "http://localhost:8000";
let airportsUrl = () => {j|$apiBaseUrl/airports|j};
let airportUrl = id => {j|$airportsUrl/$id|j};
let carriersUrl = () => {j|$apiBaseUrl/carriers|j};
let carrierUrl = id => {j|$carriersUrl/$id|j};
let statisticsUrl = limit => {j|$apiBaseUrl/statistics?limit=$limit|j};
let statisticUrl = id => {j|$statisticsUrl/$id|j};

type airport = {
  id: int,
  code: string,
  name: string,
};

type airports = array(airport);

type carrier = {
  id: int,
  code: string,
  name: string,
};

type carriers = array(carrier);

type statistic = {
  id: int,
  flightsCancelled: int,
  flightsDelayed: int,
  flightsDiverted: int,
  flightsOnTime: int,
  flightsTotal: int,
  minutesDelayedCarrier: int,
  minutesDelayedLateAircraft: int,
  minutesDelayedNationalAviationSystem: int,
  minutesDelayedSecurity: int,
  minutesDelayedWeather: int,
  minutesDelayedTotal: int,
  numberOfDelaysCarrier: int,
  numberOfDelaysLateAircraft: int,
  numberOfDelaysNationalAviationSystem: int,
  numberOfDelaysSecurity: int,
  numberOfDelaysWeather: int,
  time: string,
  carrierId: int,
  airportId: int,
};

type statistics = array(statistic);

module Decode = {
  let airport = (json): airport =>
    Json.Decode.{
      id: json |> field("id", int),
      code: json |> field("code", string),
      name: json |> field("name", string),
    };

  let carrier = (json): carrier =>
    Json.Decode.{
      id: json |> field("id", int),
      code: json |> field("code", string),
      name: json |> field("name", string),
    };

  let airports = (json): array(airport) => {
    Json.Decode.(json |> array(airport));
  };

  let carriers = (json): array(carrier) =>
    Json.Decode.(json |> array(carrier));

  let statistic = (json): statistic => {
    Json.Decode.{
      id: json |> field("id", int),
      flightsCancelled: json |> field("flights_cancelled", int),
      flightsDelayed: json |> field("flights_delayed", int),
      flightsDiverted: json |> field("flights_diverted", int),
      flightsOnTime: json |> field("flights_on_time", int),
      flightsTotal: json |> field("flights_total", int),
      minutesDelayedCarrier: json |> field("minutes_delayed_carrier", int),
      minutesDelayedLateAircraft:
        json |> field("minutes_delayed_late_aircraft", int),
      minutesDelayedNationalAviationSystem:
        json |> field("minutes_delayed_national_aviation_system", int),
      minutesDelayedSecurity: json |> field("minutes_delayed_security", int),
      minutesDelayedWeather: json |> field("minutes_delayed_weather", int),
      minutesDelayedTotal: json |> field("minutes_delayed_total", int),
      numberOfDelaysCarrier: json |> field("number_of_delays_carrier", int),
      numberOfDelaysLateAircraft:
        json |> field("number_of_delays_late_aircraft", int),
      numberOfDelaysNationalAviationSystem:
        json |> field("number_of_delays_national_aviation_system", int),
      numberOfDelaysSecurity: json |> field("number_of_delays_security", int),
      numberOfDelaysWeather: json |> field("number_of_delays_weather", int),
      time: json |> field("time", string),
      carrierId: json |> field("carrier_id", int),
      airportId: json |> field("airport_id", int),
    };
  };

  let statistics = (json): array(statistic) =>
    Json.Decode.(json |> array(statistic));
};

let fetchAirport = (id, callback) =>
  Js.Promise.(
    Fetch.fetch(airportUrl(id))
    |> then_(Fetch.Response.json)
    |> then_(json =>
         json
         |> Decode.airport
         |> (
           airports => {
             callback(airports);
             resolve();
           }
         )
       )
    |> ignore
  ); /* TODO: error handling */

let fetchAirport = (id, callback) =>
  Js.Promise.(
    Fetch.fetch(airportUrl(id))
    |> then_(Fetch.Response.json)
    |> then_(json =>
         json
         |> Decode.airport
         |> (
           airports => {
             callback(airports);
             resolve();
           }
         )
       )
    |> ignore
  ); /* TODO: error handling */

let fetchAirports = callback =>
  Js.Promise.(
    Fetch.fetch(airportsUrl())
    |> then_(Fetch.Response.json)
    |> then_(json =>
         json
         |> Decode.airports
         |> (
           airports => {
             callback(airports);
             resolve();
           }
         )
       )
    |> ignore
  ); /* TODO: error handling */

let fetchCarrier = (id, callback) =>
  Js.Promise.(
    Fetch.fetch(carrierUrl(id))
    |> then_(Fetch.Response.json)
    |> then_(json =>
         json
         |> Decode.carrier
         |> (
           carriers => {
             callback(carriers);
             resolve();
           }
         )
       )
    |> ignore
  ); /* TODO: error handling */

let fetchCarriers = callback =>
  Js.Promise.(
    Fetch.fetch(carriersUrl())
    |> then_(Fetch.Response.json)
    |> then_(json =>
         json
         |> Decode.carriers
         |> (
           carriers => {
             callback(carriers);
             resolve();
           }
         )
       )
    |> ignore
  ); /* TODO: error handling */

let fetchStatistics = (limit, callback) => {
  Js.log(statisticsUrl(limit));
  Js.Promise.(
    Fetch.fetch(statisticsUrl(limit))
    |> then_(Fetch.Response.json)
    |> then_(json =>
         json
         |> Decode.statistics
         |> (
           statistics => {
             Js.log("Statistics loaded");
             callback(statistics);
             resolve();
           }
         )
       )
    |> ignore
  ); /* TODO: error handling */
};
