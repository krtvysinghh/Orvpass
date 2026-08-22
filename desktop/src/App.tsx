import {useEffect,useState} from "react";
import {getAppInfo} from "./lib/core";
import LockScreen from "./components/LockScreen";
import Vault from "./components/Vault";
import "./styles/app.css";

export default function App(){

const [unlocked,setUnlocked]=useState(false);
const [info,setInfo]=useState("");

useEffect(()=>{
    getAppInfo().then(setInfo);
},[]);

return (
    unlocked
    ? <Vault/>
    :
    <LockScreen onUnlock={()=>setUnlocked(true)}/>
);
}
