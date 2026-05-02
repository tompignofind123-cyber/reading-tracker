import React from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";

interface Props {}

export const ExportButton: React.FC<Props> = () => {
  const doExport = async (format: "json" | "csv") => {
    const ext = format === "json" ? "json" : "csv";
    const data: string = await invoke(format === "json" ? "export_json" : "export_csv");

    const filePath = await save({
      defaultPath: `阅读记录.${ext}`,
      filters: [{ name: format.toUpperCase(), extensions: [ext] }],
    });

    if (filePath) {
      await writeTextFile(filePath, data);
    }
  };

  return (
    <div className="export-buttons">
      <button className="btn-ghost btn-sm" onClick={() => doExport("json")}>导出 JSON</button>
      <button className="btn-ghost btn-sm" onClick={() => doExport("csv")}>导出 CSV</button>
    </div>
  );
};
