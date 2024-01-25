import App from "./App.svelte";
import "./tailwind.css";

const app: App = new App({
  target: document.getElementById("app")!,
});

export default app;
