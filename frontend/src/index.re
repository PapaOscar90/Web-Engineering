/* Bring `Utils` module into scope */
open Utils;

registerServiceWorker();

/* Render the main application */
ReactDOMRe.renderToElementWithId(<App />, "root");

/* Start routing */
ReasonReact.Router.push("");

unregisterServiceWorker();
