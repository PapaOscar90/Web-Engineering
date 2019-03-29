open Belt;
open Utils;

type state = {
  carriers: Models.carriers,
  loading: bool,
};

type action =
  | Loaded(Models.carriers)
  | Loading;

let component = ReasonReact.reducerComponent("Carriers");

let make = _children => {
  let loadCarriers = ({ReasonReact.state, send}) => {
    Js.log("Fetching carriers");
    Models.fetchCarriers(payload => send(Loaded(payload))) |> ignore;
    Js.log("Carriers loaded");
    send(Loading);
  };
  {
    ...component,
    initialState: () => {carriers: [||], loading: false},
    reducer: (action, state) =>
      switch (action) {
      | Loading => ReasonReact.Update({...state, loading: true})
      | Loaded(data) =>
        let updatedCarriers = Array.concat(state.carriers, data);
        ReasonReact.Update({carriers: updatedCarriers, loading: false});
      },
    didMount: self => {
      loadCarriers(self);
    },
    render: self =>
      <div>
        {if (Array.length(self.state.carriers) > 0) {
           self.state.carriers
           ->(
               Array.mapWithIndex((index, carrier) =>
                 <Carrier key={string_of_int(carrier.id)} index carrier />
               )
             )
           ->ReasonReact.array;
         } else {
           ReasonReact.null;
         }}
      </div>,
  };
};
