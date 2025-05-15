import { getApi, getKeypair } from "./wallet";

export async function uploadContent(cid) {
  const api = getApi();
  const kp = getKeypair();
  const tx = api.tx.content.upload(cid);
  return tx.signAndSend(kp);
}

export async function postToForum(tokenId, cid) {
  const api = getApi();
  const kp = getKeypair();
  const tx = api.tx.forum.addPost(tokenId, cid);
  return tx.signAndSend(kp);
}

export async function executeSwap(a, b, amount) {
  const api = getApi();
  const kp = getKeypair();
  const tx = api.tx.dex.swap(a, b, amount);
  return tx.signAndSend(kp);
}

export async function launchToken(tier, supply) {
  const api = getApi();
  const kp = getKeypair();
  const tx = api.tx.tokenLaunch.createToken(tier, supply);
  return tx.signAndSend(kp);
}

export async function submitProposal(desc) {
  const api = getApi();
  const kp = getKeypair();
  const tx = api.tx.dao.createProposal(desc);
  return tx.signAndSend(kp);
}

export async function voteProposal(id, approve) {
  const api = getApi();
  const kp = getKeypair();
  const tx = api.tx.dao.vote(id, approve);
  return tx.signAndSend(kp);
}
