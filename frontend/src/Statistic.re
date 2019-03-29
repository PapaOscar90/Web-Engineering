open Utils;

let component = ReasonReact.statelessComponent("Statistic");

let make = (~statistic: Models.statistic, ~index: int, _children) => {
  ...component,
  render: self =>
    <div className="StatisticItem">
      {ReasonReact.string(
         string_of_int(statistic.id)
         ++ " "
         ++ string_of_int(statistic.flights_delayed)
         ++ " "
         ++ string_of_int(statistic.minutes_delayed),
       )}
    </div>,
};
