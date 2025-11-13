import {getPolkadotApi} from "./client";
import {web3FromSource} from "@polkadot/extension-dapp";

export async function deposit(assetId: number, amount: bigint, account: string){
    const api = getPolkadotApi();
    const injector = await web3FromSource("polakdot-js");

    const tx = (await api).tx.assets.deposit(assetId,amount);

    return tx.signAndSend(account, { signer: injector.signer });
} 

export async function withdraw(assetId: number, amount: bigint, account: string) {
    const api = await getPolkadotApi();
    const injector = await web3FromSource("polkadot-js");
  
    const tx = api.tx.exchange.withdraw(assetId, amount);
  
    return tx.signAndSend(account, { signer: injector.signer });
  }