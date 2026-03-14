import { invoke } from "@tauri-apps/api/core";

let _privacyMode = $state(false);

export const privacyStore = {
  get privacyMode() { return _privacyMode; },
  set privacyMode(v) { _privacyMode = v; },
};

export async function loadPrivacyMode() {
  try {
    const settings = await invoke("get_settings");
    _privacyMode = settings.privacy_mode ?? false;
  } catch (e) {
    console.error("Failed to load privacy mode:", e);
  }
}
