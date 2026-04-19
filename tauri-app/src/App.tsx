import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./components/Sidebar";
import AuditPanel from "./components/AuditPanel";
import OptimizePanel from "./components/OptimizePanel";
import GeneratePanel from "./components/GeneratePanel";
import ProjectsPanel from "./components/ProjectsPanel";

export type View = "audit" | "optimize" | "generate" | "projects";

const styles: Record<string, React.CSSProperties> = {
  app: {
    display: "flex",
    height: "100vh",
    background: "#0d1117",
    color: "#e6edf3",
    overflow: "hidden",
  },
  main: {
    flex: 1,
    overflowY: "auto",
    padding: "32px",
  },
};

export default function App() {
  const [view, setView] = useState<View>("audit");

  return (
    <div style={styles.app}>
      <Sidebar active={view} onChange={setView} />
      <main style={styles.main}>
        {view === "audit" && <AuditPanel />}
        {view === "optimize" && <OptimizePanel />}
        {view === "generate" && <GeneratePanel />}
        {view === "projects" && <ProjectsPanel />}
      </main>
    </div>
  );
}
