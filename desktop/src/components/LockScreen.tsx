interface Props {
  onUnlock: () => void;
}

export default function LockScreen({onUnlock}: Props){
  return (
    <div className="screen">
      <h1>🔐 Orvpass v3.0</h1>
      <button onClick={onUnlock}>
        Unlock Vault
      </button>
    </div>
  );
}
