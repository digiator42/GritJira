"use client";

import { useEffect, useRef, useState, type ReactNode } from "react";

export function Dropdown({
  trigger,
  align = "right",
  panelClassName = "",
  children,
}: {
  trigger: (props: { open: boolean; toggle: () => void }) => ReactNode;
  align?: "left" | "right";
  panelClassName?: string;
  children: ReactNode | ((close: () => void) => ReactNode);
}) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onPointer = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") setOpen(false);
    };
    const onBlur = () => setOpen(false);
    document.addEventListener("mousedown", onPointer);
    document.addEventListener("keydown", onKey);
    window.addEventListener("blur", onBlur);
    return () => {
      document.removeEventListener("mousedown", onPointer);
      document.removeEventListener("keydown", onKey);
      window.removeEventListener("blur", onBlur);
    };
  }, [open]);

  const toggle = () => setOpen((v) => !v);

  return (
    <div ref={ref} className="relative">
      {trigger({ open, toggle })}
      <div
        role="menu"
        className={`absolute z-50 mt-1 min-w-44 rounded-md border border-jira-border bg-jira-panel py-1 shadow-2xl transition-all duration-150 ease-out ${
          align === "right" ? "right-0 origin-top-right" : "left-0 origin-top-left"
        } ${open ? "visible scale-100 opacity-100" : "invisible scale-95 opacity-0"} ${panelClassName}`}
        aria-hidden={!open}
      >
        {typeof children === "function" ? children(close) : children}
      </div>
    </div>
  );
}