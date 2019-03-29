open Utils;

let component = ReasonReact.statelessComponent("Statistic");

let make = (~statistic: Models.statistic, ~index: int, _children) => {
  ...component,
  render: self =>
    <div className="StatisticItem">
      {ReasonReact.string(string_of_int(statistic.id))}
      {ReasonReact.string(string_of_int(statistic.flightsDelayed))}
      {ReasonReact.string(string_of_int(statistic.flightsDelayed))}
    </div>,
};
