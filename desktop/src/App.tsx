import {useState} from "react";

export default function App(){

const [locked,setLocked]=useState(true);

return (
<div>
<h1>Orvpass v3.0</h1>

{
locked ?
<button onClick={()=>setLocked(false)}>
Unlock Vault
</button>
:
<>
<h2>Vault</h2>
<button>Add Password</button>
<button>Generate</button>
<button>Settings</button>
</>
}

</div>
)
}
