import { invoke } from "@tauri-apps/api/core";
import type { Player } from "../../types/player";
export async function getPlayers(): Promise<Player[]> {
  return invoke<Player[]>("get_players");
}
