// Zustand store for global state management

import { create } from "zustand";
import type {
  PlayerProfile,
  GameState,
  GameStats,
  PlayerSettings,
} from "@/types";

interface AppStore {
  // Player state
  player: PlayerProfile | null;
  setPlayer: (player: PlayerProfile) => void;

  // Game state
  currentGame: GameState | null;
  setCurrentGame: (game: GameState | null) => void;

  // Stats
  stats: GameStats | null;
  setStats: (stats: GameStats) => void;

  // UI state
  theme: "light" | "dark";
  setTheme: (theme: "light" | "dark") => void;

  // Modal/Dialog state
  showHints: boolean;
  setShowHints: (show: boolean) => void;

  showSettings: boolean;
  setShowSettings: (show: boolean) => void;

  // Message display
  message: string | null;
  messageType: "success" | "error" | "info" | "warning";
  setMessage: (message: string | null, type?: "success" | "error" | "info" | "warning") => void;

  // Initialize app
  initializeApp: (playerName: string) => Promise<void>;

  // Update settings
  updateSettings: (settings: Partial<PlayerSettings>) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  // Initial state
  player: null,
  currentGame: null,
  stats: null,
  theme: (localStorage.getItem("theme") as "light" | "dark") || "light",
  showHints: false,
  showSettings: false,
  message: null,
  messageType: "info",

  // Actions
  setPlayer: (player) => set({ player }),
  setCurrentGame: (game) => set({ currentGame: game }),
  setStats: (stats) => set({ stats }),

  setTheme: (theme) => {
    localStorage.setItem("theme", theme);
    set({ theme });
  },

  setShowHints: (show) => set({ showHints: show }),
  setShowSettings: (show) => set({ showSettings: show }),

  setMessage: (message, messageType = "info") =>
    set({ message, messageType }),

  initializeApp: async (playerName: string) => {
    try {
      // Initialize database
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("init_db_cmd");

      // Create or get player
      const player = (await invoke("get_or_create_player_cmd", {
        playerName,
      })) as PlayerProfile;

      set({ player });

      // Create new game
      const game = (await invoke("new_game_cmd", {
        request: {
          player_id: player.id,
          game_mode: "Classic",
        },
      })) as GameState;

      set({ currentGame: game });

      // Get player stats
      const stats = (await invoke("get_player_stats_cmd", {
        playerId: player.id,
      })) as GameStats;

      set({ stats });
    } catch (error) {
      console.error("Failed to initialize app:", error);
      set({
        message: "Failed to initialize app",
        messageType: "error",
      });
    }
  },

  updateSettings: (newSettings) => {
    set((state) => ({
      player: state.player
        ? {
            ...state.player,
            settings: {
              ...state.player.settings,
              ...newSettings,
            },
          }
        : null,
    }));
  },
}));

// Persist theme preference
export const initializeTheme = () => {
  const theme = useAppStore.getState().theme;
  document.documentElement.classList.toggle("dark-theme", theme === "dark");
};
