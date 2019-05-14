open Utils;

let component = ReasonReact.statelessComponent("Statistic");

let make = (~statistic: Models.statistic, ~index: int, _children) => {
  ...component,
  render: self =>
    <div className="StatisticItem">
      {ReasonReact.string(
         string_of_int(statistic.id)
         ++ " flights delayed total:"
         ++ string_of_int(statistic.flightsDelayed)
         ++ " minutes delayed total:"
         ++ string_of_int(statistic.minutesDelayedTotal),
       )}
    </div>,
};
