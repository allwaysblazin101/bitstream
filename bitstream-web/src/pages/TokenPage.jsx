import { postToForum } from "../wallet/tx";
import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import axios from 'axios';

const TokenPage = () => {
  const { tokenId } = useParams();
  const [posts, setPosts] = useState([]);
  const [cid, setCid] = useState('');
  const [newPostCID, setNewPostCID] = useState('');

  const loadForum = async () => {
    // Placeholder — replace with chain call to pallet-forum
    const mock = await axios.get(`http://localhost:8080/forum/${tokenId}`).catch(() => ({ data: [] }));
    setPosts(mock.data);
  };

  const submitPost = async () => {
    try {
      // Submit post CID to chain via pallet-forum
      console.log('Post CID submitted:', newPostCID);
    await postToForum(tokenId, newPostCID);
    } catch (err) {
      console.error('Error submitting post:', err);
    }
  };

  useEffect(() => {
    loadForum();
  }, [tokenId]);

  return (
    <div className="token-page">
      <h1>Token #{tokenId}</h1>

      <section className="token-info">
        <p>[Market Price Placeholder]</p>
        <p>[Supply | Volume | PoSE Score]</p>
        <button>Swap</button>
        <button>Stake</button>
      </section>

      <section className="forum">
        <h2>Discussion Forum</h2>
        <input type="text" placeholder="Enter IPFS CID for post" onChange={e => setNewPostCID(e.target.value)} />
        <button onClick={submitPost}>Post</button>
        <div>
          {posts.map((p, i) => (
            <div key={i}>
              <p><b>{p.author}</b> — {p.cid}</p>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
};

export default TokenPage;
