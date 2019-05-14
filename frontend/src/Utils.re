/* require css file for side effect only */
[@bs.val] external requireCSS: string => unit = "require";

[@bs.module "./serviceWorker"]
external registerServiceWorker: unit => unit = "register";

[@bs.module "./serviceWorker"]
external unregisterServiceWorker: unit => unit = "unregister";
