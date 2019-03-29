open Utils;

let component = ReasonReact.statelessComponent("Carrier");

let make = (~carrier: Models.carrier, ~index: int, _children) => {
  ...component,
  render: self =>
    <div className="CarrierItem">
      {ReasonReact.string(carrier.code ++ " " ++ carrier.name)}
    </div>,
};
