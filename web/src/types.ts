import { IconSource } from "svelte-hero-icons";

export type Routing = {
  url: string,
  name: string,
  component?: Function,
};

export type Slogan = {
  description: string,
}

export type Quality = {
  name: string,
  description: string,
  icon: IconSource,
}

export type ReddyVault = "ReddyVault";
