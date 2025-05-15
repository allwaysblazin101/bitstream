import { mnemonicGenerate, cryptoWaitReady } from "@polkadot/util-crypto";
import { Keyring } from "@polkadot/keyring";
import { ApiPromise, WsProvider } from "@polkadot/api";

let api, keypair;

export async function initWallet() {
  await cryptoWaitReady();

  const mnemonic = mnemonicGenerate();
  const keyring = new Keyring({ type: "sr25519" });
  keypair = keyring.addFromUri(mnemonic);
  return { mnemonic, address: keypair.address };
}

export async function connectNode() {
  const provider = new WsProvider("ws://127.0.0.1:9944");
  api = await ApiPromise.create({ provider });
}

export async function getBalance() {
  const { data: balance } = await api.query.system.account(keypair.address);
  return balance.free.toHuman();
}

export async function signAndSend(extrinsic) {
  return await extrinsic.signAndSend(keypair);
}

export function getKeypair() {
  return keypair;
}

export function getApi() {
  return api;
}
