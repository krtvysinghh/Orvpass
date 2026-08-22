import { invoke } from "@tauri-apps/api/core";

export async function getAppInfo(){
    return await invoke<string>("get_app_info");
}
