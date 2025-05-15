import { uploadContent } from "../wallet/tx";
import React, { useState } from 'react';
import axios from 'axios';

const UploadPage = () => {
  const [file, setFile] = useState(null);
  const [cid, setCid] = useState('');
  const [status, setStatus] = useState('');

  const handleUpload = async () => {
    if (!file) return;

    setStatus('Uploading to IPFS...');
    const form = new FormData();
    form.append('file', file);

    try {
      const res = await axios.post('http://localhost:8080/upload', form, {
        headers: { 'Content-Type': 'multipart/form-data' }
      });
      setCid(res.data.cid);
      setStatus('Success: ' + res.data.cid);
    } catch (err) {
      console.error(err);
      setStatus('Upload failed');
    }
  };

  return (
    <div className="upload-page">
      <h1>Upload to Bitstream</h1>
      <input type="file" onChange={e => setFile(e.target.files[0])} />
      <button onClick={handleUpload}>Upload</button>
      <p>Status: {status}</p>
      {cid && <p>IPFS CID: {cid}</p>}
    </div>
  );
};

export default UploadPage;
