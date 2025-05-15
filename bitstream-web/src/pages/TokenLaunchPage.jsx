import React, { useState } from "react";
import { launchToken } from "../wallet/tx";
import { useNavigate } from "react-router-dom";

const TokenLaunchPage = () => {
  const [tier, setTier] = useState(1);
  const [supply, setSupply] = useState(1000000);
  const [status, setStatus] = useState("");
  const navigate = useNavigate();

  const handleLaunch = async () => {
    setStatus("Launching...");
    try {
      await launchToken(tier, supply);
      setStatus("Token launched.");
      navigate("/token/0"); // Replace with actual token ID tracker
    } catch (e) {
      console.error(e);
      setStatus("Launch failed.");
    }
  };

  const flatFees = [60, 250, 500, 1000, 1500];
  const percentCuts = [2, 2, 2, 3, 3];
  const flat = flatFees[tier - 1];
  const cut = (supply * percentCuts[tier - 1]) / 100;

  return (
    <div className="launch-page">
      <h1>Launch Your Token</h1>

      <label>Tier (1–5):</label>
      <input
        type="number"
        value={tier}
        min="1"
        max="5"
        onChange={(e) => setTier(parseInt(e.target.value))}
      />

      <label>Supply:</label>
      <input
        type="number"
        value={supply}
        onChange={(e) => setSupply(parseInt(e.target.value))}
      />

      <p>Flat Fee:  + {percentCuts[tier - 1]}% of supply ({cut} tokens)</p>

      <button onClick={handleLaunch}>Launch</button>
      <p>{status}</p>
    </div>
  );
};

export default TokenLaunchPage;
