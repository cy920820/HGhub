import { invoke } from "@tauri-apps/api/core";
import { platform } from "./api";

export type Haptic = "light" | "medium" | "heavy" | "success" | "warning" | "error" | "selection";

/* iOS 的 WebKit 不实现 Vibration API，navigator.vibrate 是空操作，只能走
   UIImpactFeedbackGenerator（tauri-plugin-haptics）。Android 维持原状：换成插件
   还要重出 gen/android 并带 VIBRATE 权限，收益不抵风险。 */
const ANDROID: Record<Haptic, number | number[]> = {
  light: 8,
  medium: 16,
  heavy: 26,
  success: [10, 40, 18],
  warning: [16, 30, 16],
  error: [26, 26, 26],
  selection: 6,
};

export function haptic(kind: Haptic = "medium") {
  if (platform === "ios") {
    const call = (cmd: string, args?: Record<string, string>) => invoke(cmd, args).catch(() => {});
    if (kind === "selection") call("plugin:haptics|selection_feedback");
    else if (kind === "success" || kind === "warning" || kind === "error")
      call("plugin:haptics|notification_feedback", { type: kind });
    else call("plugin:haptics|impact_feedback", { style: kind });
  } else if (platform === "android") {
    try {
      navigator.vibrate?.(ANDROID[kind]);
    } catch {
      /* 部分 ROM 禁震动，忽略 */
    }
  }
}
