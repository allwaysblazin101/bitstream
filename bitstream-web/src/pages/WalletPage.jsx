import React, { useState, useEffect } from 'react';
import { initWallet, connectNode, getBalance } from '../wallet/wallet';

const WalletPage = () => {
  const [wallet, setWallet] = useState({});
  const [balance, setBalance] = useState('');

  useEffect(() => {
    (async () => {
      await connectNode();
      const w = await initWallet();
      setWallet(w);
      const b = await getBalance();
      setBalance(b);
    })();
  }, []);

  return (
    <div className="wallet-page">
      <h1>Bitstream Wallet</h1>
      <p><b>Address:</b> {wallet.address}</p>
      <p><b>Balance:</b> {balance}</p>
      <p><b>Mnemonic:</b> {wallet.mnemonic}</p>
    </div>
  );
};

export default WalletPage;
