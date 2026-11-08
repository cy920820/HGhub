import { useEffect, useState } from "react";

/**
 * 键盘遮住输入框的兜底高度（px）。
 * iOS 的 WKWebView 弹键盘时不缩 layout viewport，只有 visualViewport 变矮，
 * 所以用 innerHeight - visualViewport 量出键盘高度，让弹层自己上移。
 */
export function useKeyboardInset(): number {
  const [inset, setInset] = useState(0);

  useEffect(() => {
    const vv = window.visualViewport;
    if (!vv) return undefined;
    let last = 0;
    const measure = () => {
      const next = Math.max(0, Math.round(window.innerHeight - vv.height - vv.offsetTop));
      setInset(next);
      // iOS 收起键盘后常把文档留在被顶起的位置，键盘消失那一刻把它拉回顶部
      if (last > 0 && next === 0) window.scrollTo(0, 0);
      last = next;
    };
    measure();
    vv.addEventListener("resize", measure);
    vv.addEventListener("scroll", measure);
    return () => {
      vv.removeEventListener("resize", measure);
      vv.removeEventListener("scroll", measure);
    };
  }, []);

  return inset;
}
