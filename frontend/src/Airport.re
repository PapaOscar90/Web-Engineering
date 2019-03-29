open Utils;

let component = ReasonReact.statelessComponent("Airport");

let make = (~airport: Models.airport, ~index: int, _children) => {
  ...component,
  render: self =>
    <div className="AirportItem">
      {ReasonReact.string(airport.code ++ " " ++ airport.name)}
    </div>,
};
