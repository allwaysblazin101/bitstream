import React, { useEffect, useState } from "react";
import { submitProposal, voteProposal } from "../wallet/tx";
import { getApi } from "../wallet/wallet";

const DAOPage = () => {
  const [proposals, setProposals] = useState([]);
  const [newDesc, setNewDesc] = useState("");
  const [status, setStatus] = useState("");

  const loadProposals = async () => {
    const api = getApi();
    const count = await api.query.dao.proposalCount();
    let result = [];

    for (let i = 0; i < count.toNumber(); i++) {
      const prop = await api.query.dao.proposals(i);
      if (prop.isSome) {
        const data = prop.unwrap();
        result.push({
          id: i,
          desc: data.description.toHuman(),
          yes: data.yesVotes.toNumber(),
          no: data.noVotes.toNumber(),
          executed: data.executed.isTrue,
        });
      }
    }
    setProposals(result);
  };

  const create = async () => {
    setStatus("Submitting...");
    try {
      await submitProposal(newDesc);
      setStatus("Proposal submitted.");
      await loadProposals();
    } catch (e) {
      console.error(e);
      setStatus("Submit failed.");
    }
  };

  const vote = async (id, approve) => {
    setStatus("Voting...");
    try {
      await voteProposal(id, approve);
      setStatus("Vote cast.");
      await loadProposals();
    } catch (e) {
      console.error(e);
      setStatus("Vote failed.");
    }
  };

  useEffect(() => {
    loadProposals();
  }, []);

  return (
    <div className="dao-page">
      <h1>Bitstream DAO</h1>

      <input
        type="text"
        placeholder="Proposal description"
        onChange={(e) => setNewDesc(e.target.value)}
      />
      <button onClick={create}>Submit Proposal</button>
      <p>{status}</p>

      <div>
        {proposals.map((p) => (
          <div key={p.id}>
            <p><b>#{p.id}</b> — {p.desc}</p>
            <p>Yes: {p.yes} | No: {p.no} | Executed: {p.executed ? "Yes" : "No"}</p>
            {!p.executed && (
              <>
                <button onClick={() => vote(p.id, true)}>Yes</button>
                <button onClick={() => vote(p.id, false)}>No</button>
              </>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default DAOPage;
