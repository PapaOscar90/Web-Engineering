open Belt;

let apiBaseUrl = "http://localhost:8000";
let airportsUrl = () => {j|$apiBaseUrl/airports|j};
let airportUrl = id => {j|$airportsUrl/$id|j};
let carriersUrl = () => {j|$apiBaseUrl/carriers|j};
let carrierUrl = id => {j|$carriersUrl/$id|j};

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
