import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { installMockIpcWhenTauriIsAbsent } from "../../shared/mock";
import "../../index.css";

installMockIpcWhenTauriIsAbsent();

createRoot(document.getElementById("root")!).render(<StrictMode>panel</StrictMode>);
