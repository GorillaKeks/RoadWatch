import type { Location } from "./location";

export type PlayerStatus = "online" | "offline";
export type GameType = "ets2" | "ats";

export interface LivePosition {
  x: number;
  y: number;
  z?: number;
}

export interface Player {
  id: number;
  truckersmpId: number;
  username: string;
  role: string;
  membership: string;
  avatarUrl?: string;

  status: PlayerStatus;
  game?: GameType;
  server?: string;

  location?: Location;
  livePosition?: LivePosition;
  distance?: string;
}
