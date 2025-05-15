import SwapPage from "./pages/SwapPage";
import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import UploadPage from './pages/UploadPage';
import TokenPage from './pages/TokenPage';

const App = () => {
  return (
    <Router>
      <div className="app-shell">
        <header className="top-bar">
          <h1>Bitstream</h1>
          <nav>
        <a href="/swap/BTC/ETH">Swap</a>
            <a href="/upload">Upload</a>
            <a href="/token/0">Token #0</a>
          </nav>
        </header>

        <main className="main-content">
          <Routes>
    <Route path="/swap/:from/:to" element={<SwapPage />} />
            <Route path="/upload" element={<UploadPage />} />
            <Route path="/token/:tokenId" element={<TokenPage />} />
          </Routes>
        </main>

        <footer className="footer">
          <p>&copy; Bitstream 2025</p>
        </footer>
      </div>
    </Router>
  );
};

export default App;
