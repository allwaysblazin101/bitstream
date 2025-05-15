import React, { useState } from 'react';
import { useParams } from 'react-router-dom';

const SwapPage = () => {
  const { from, to } = useParams();
  const [amount, setAmount] = useState('');
  const [status, setStatus] = useState('');

  const handleSwap = async () => {
    setStatus('Swapping...');
    try {
      // TODO: hook into pallet-dex
      setTimeout(() => setStatus(`Swapped ${amount} ${from} → ${to}`), 1000);
    } catch (e) {
      setStatus('Swap failed');
    }
  };

  return (
    <div className="swap-page">
      <h1>Swap {from} → {to}</h1>
      <input
        type="number"
        placeholder="Amount"
        onChange={e => setAmount(e.target.value)}
      />
      <button onClick={handleSwap}>Execute Swap</button>
      <p>Status: {status}</p>
    </div>
  );
};

export default SwapPage;
