import type { Routing } from "../../types";
import Home from "../pages/Home.svelte";
import About from "../pages/About.svelte";
import Login from "../pages/Login.svelte";

export const routing: Routing[] = [
  {
    url: "/",
    name: "Home",
    component: Home
  },
  {
    url: "/about",
    name: "About",
    component: About
  },
  {
    url: "/login",
    name: "Login",
    component: Login
  },
];

export const localhost: string = "http://localhost:8080/";
