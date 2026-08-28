import { invoke } from "@tauri-apps/api/core";

export async function getBackendStatus(): Promise<string> {
  return invoke<string>("get_backend_status");
}
