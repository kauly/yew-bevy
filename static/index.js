import "./app.css";

import("../pkg").then((module) => {
  module.run_app();
});
