open Belt;
open Utils;

type state = {
  statistics: Models.statistics,
  loading: bool,
};

type action =
  | Loaded(Models.statistics)
  | Loading;

let component = ReasonReact.reducerComponent("Statistics");

let make = _children => {
  let loadStatistics = ({ReasonReact.state, send}) => {
    Models.fetchStatistics(100, payload => send(Loaded(payload))) |> ignore;
    send(Loading);
  };
  {
    ...component,
    initialState: () => {statistics: [||], loading: false},
    reducer: (action, state) =>
      switch (action) {
      | Loading => ReasonReact.Update({...state, loading: true})
      | Loaded(data) =>
        let updatedStatistics = Array.concat(state.statistics, data);
        ReasonReact.Update({statistics: updatedStatistics, loading: false});
      },
    didMount: self => {
      loadStatistics(self);
    },
    render: self =>
      <div>
        <BsRecharts.LineChart
          margin={"top": 0, "right": 0, "bottom": 0, "left": 0}
          data={self.state.statistics}>
          <BsRecharts.Line name="Minutes delayed" dataKey="minutesDelayed" />
        </BsRecharts.LineChart>
      </div>,
  };
};
