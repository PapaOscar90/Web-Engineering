[%bs.raw {|require('./app.css')|}];

[@bs.module] external logo : string = "./logo.svg";

[@bs.module] external icon : string = "./icon.jpg";

let component = ReasonReact.statelessComponent("App");

let make = (~message, _children) => {
  ...component,
  
  // Override the render for App
  render: _self =>

    // Return a div "App"
    <div className="App">
      // Also containing a p with more text and different class (for style)
      <img src=icon className="App-icon" alt="icon" />
      <p className="App-intro">
        (ReasonReact.string("Welcome to Corgi Flight Statistics"))
        <code> (ReasonReact.string(" Fly Smart! ")) </code>
      </p>

      // That contains an App-Header that contains an image and text
      <div className="App-header">
        <img src=logo className="App-logo" alt="logo" />
        <h2> (ReasonReact.string(message)) </h2>
      </div>
    </div>,
};
