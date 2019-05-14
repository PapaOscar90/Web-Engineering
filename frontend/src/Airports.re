open Belt;
open Utils;

type state = {
  airports: Models.airports,
  loading: bool,
};

type action =
  | Loaded(Models.airports)
  | Loading;

let component = ReasonReact.reducerComponent("Airports");

let make = _children => {
  let loadAirports = ({ReasonReact.state, send}) => {
    Models.fetchAirports(payload => send(Loaded(payload))) |> ignore;
    send(Loading);
  };
  {
    ...component,
    initialState: () => {airports: [||], loading: false},
    reducer: (action, state) =>
      switch (action) {
      | Loading => ReasonReact.Update({...state, loading: true})
      | Loaded(data) =>
        let updatedAirports = Array.concat(state.airports, data);
        ReasonReact.Update({airports: updatedAirports, loading: false});
      },
    didMount: self => {
      loadAirports(self);
    },
    render: self =>
      <div>
        {if (Array.length(self.state.airports) > 0) {
           self.state.airports
           ->(
               Array.mapWithIndex((index, airport) =>
                 <Airport key={string_of_int(airport.id)} index airport />
               )
             )
           ->ReasonReact.array;
         } else {
           ReasonReact.null;
         }}
      </div>,
  };
};
