open Utils;

let component = ReasonReact.statelessComponent("Carrier");

let make = (~carrier: Models.carrier, ~index: int, _children) => {
  ...component,
  render: self =>
    <div className="CarrierItem">
      {ReasonReact.string(string_of_int(carrier.id))}
      {ReasonReact.string(carrier.code)}
      {ReasonReact.string(carrier.name)}
    </div>,
};
