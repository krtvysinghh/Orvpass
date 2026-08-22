import {useState} from "react";
import LockScreen from "./components/LockScreen";
import Vault from "./components/Vault";
import "./styles/app.css";

export default function App(){

  const [unlocked,setUnlocked]=useState(false);

  return unlocked
    ? <Vault/>
    : <LockScreen onUnlock={()=>setUnlocked(true)}/>;

}
