[%bs.raw {|require('./app.css')|}];
[@bs.module] external logo: string = "./logo.svg";
[@bs.module] external icon: string = "./icon.jpg";

// Available routes
type route =
  | Home
  | Carriers
  | Airports
  | Statistics
  | Graphs
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
  | ["airports"] => Airports
  | ["statistics"] => Statistics
  | ["graphs"] => Graphs
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
      <div>
        <MaterialUi.AppBar>
          <MaterialUi.Toolbar>
            <MaterialUi.Button href="http://localhost:3000/carriers">
              "Carriers"
            </MaterialUi.Button>
            <MaterialUi.Button href="http://localhost:3000/airports">
              "Airports"
            </MaterialUi.Button>
            <MaterialUi.Button href="http://localhost:3000/statistics">
              "Statistics"
            </MaterialUi.Button>
            <MaterialUi.Button href="http://localhost:3000/graphs">
              "Graphs"
            </MaterialUi.Button>
          </MaterialUi.Toolbar>
        </MaterialUi.AppBar>
      </div>
      {switch (self.state.route) {
       | Home => ReasonReact.string("Home")
       | Carriers => <Carriers />
       | Airports => <Airports />
       | Statistics => <Statistics />
       | Graphs => <Graphs />
       | NotFound => <NotFound />
       }}
    </div>;
  },
};
