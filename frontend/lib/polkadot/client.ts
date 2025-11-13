import {ApiPromise, WsProvider} from "@polkadot/api"

let api: ApiPromise | null = null;

export async function getPolkadotApi() {
  if (api) return api;

  const wsProvider = new WsProvider("the node");
  api = await ApiPromise.create({provider: wsProvider});
  return api;
}

