import express from "express";
import bodyParser from "body-parser";
import cors from "cors";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";

const app = express();
app.use(cors());
app.use(bodyParser.json());

let api, sudo;

const init = async () => {
  const provider = new WsProvider("ws://127.0.0.1:9944");
  api = await ApiPromise.create({ provider });
  const keyring = new Keyring({ type: "sr25519" });

  sudo = keyring.addFromUri("//Alice"); // dev key
};

app.post("/faucet", async (req, res) => {
  try {
    const { address } = req.body;
    if (!address) return res.status(400).send("Missing address");

    const tx = api.tx.balances.transfer(address, 1000000000000);
    await tx.signAndSend(sudo);

    res.json({ status: "ok", tx: tx.hash.toHex() });
  } catch (err) {
    res.status(500).json({ error: err.toString() });
  }
});

app.listen(8181, () => console.log("Faucet running on port 8181"));
init();
