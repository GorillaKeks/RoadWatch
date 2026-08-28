import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "../../types/settings";

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export async function saveSettings(
  settings: AppSettings,
): Promise<AppSettings> {
  return invoke<AppSettings>("save_settings", { settings });
}
