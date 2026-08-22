import { useState } from "react";

export default function App() {
  const [locked, setLocked] = useState(true);

  return (
    <div style={{
      minHeight:"100vh",
      background:"#111",
      color:"#fff",
      padding:"40px",
      fontFamily:"system-ui"
    }}>
      <h1>🔐 Orvpass v3.0</h1>

      {locked ? (
        <button
          style={{
            padding:"12px 20px",
            fontSize:"18px"
          }}
          onClick={()=>setLocked(false)}
        >
          Unlock Vault
        </button>
      ) : (
        <>
          <h2>Vault</h2>
          <button>Add Password</button>
          <button>Generate</button>
          <button>Settings</button>
        </>
      )}
    </div>
  );
}
