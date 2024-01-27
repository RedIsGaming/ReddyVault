import { IconSource } from "svelte-hero-icons";

export type Routing = {
  url: string,
  name: string,
  component?: Function,
  target? : string,
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

export type NotFound = {
  name: string,
  description: string,
  return: string,
}
