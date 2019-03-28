[%bs.raw {|require('./app.css')|}];
[@bs.module] external logo: string = "./logo.svg";
[@bs.module] external icon: string = "./icon.jpg";

// Available routes
type route =
  | Home
  | Carriers
  | Carrier(int)
  | Airports
  | Airport(int)
  | Statistics
  | Statistic(int)
  | NotFound;

// Current route
type state = {route};

// Possible actions
type action =
  | ChangeRoute(route);

// Reducer based on action
let reducer = (action, _state) =>
  switch (action) {
  | ChangeRoute(route) => ReasonReact.Update({route: route})
  };

// Convert from url to route
let urlToRoute = (url: ReasonReact.Router.url) =>
  switch (url.path) {
  | [] => Home
  | ["carriers"] => Carriers
  | ["carriers", id] => Carrier(int_of_string(id))
  | ["airports"] => Airports
  | ["airports", id] => Airport(int_of_string(id))
  | ["statistics"] => Statistics
  | ["statistics", id] => Statistic(int_of_string(id))
  | _ => NotFound
  };

// Declare the component as one which requires a reducer
let component = ReasonReact.reducerComponent("App");

// Constructor
let make = _children => {
  ...component,
  reducer,
  initialState: () => {route: Home},
  didMount: self => {
    let watchId =
      ReasonReact.Router.watchUrl(url =>
        self.send(ChangeRoute(url |> urlToRoute))
      );
    self.onUnmount(() => ReasonReact.Router.unwatchUrl(watchId));
  },
  // Map the routes to component
  render: self => {
    <div>
      <div
        /* <MaterialUi.AppBar> <MaterialUi.Toolbar /> </MaterialUi.AppBar> */
      />
      {switch (self.state.route) {
       | Home => ReasonReact.string("Home")
       | Carriers => <Carriers />
       | Carrier(id) => ReasonReact.string("Carrier: " ++ string_of_int(id))
       | Airports => ReasonReact.string("Airports")
       | Airport(id) => ReasonReact.string("Airport: " ++ string_of_int(id))
       | Statistics => ReasonReact.string("Statistics")
       | Statistic(id) =>
         ReasonReact.string("Statistic: " ++ string_of_int(id))
       | NotFound => <NotFound />
       }}
    </div>;
  },
};
