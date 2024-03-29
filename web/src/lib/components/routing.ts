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

export const routes: Routing[] = [
  {
    url: "/",
    name: "Check more",
    target: "_self"
  },
  {
    url: "https://github.com/RedIsGaming/ReddyVault",
    name: "Github",
    target: "_blank"
  },
  {
    url: "/login",
    name: "Login",
    target: "_self"
  },
];

export const localhost: string = "http://localhost:8080/";
