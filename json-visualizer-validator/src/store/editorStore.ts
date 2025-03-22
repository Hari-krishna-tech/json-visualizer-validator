import create from "zustand";
import { persist } from "zustand/middleware";
import { FormatType, TypeGeneratorType } from "../types/global";

interface EditorState {
  content: string;
  setContent: (content: string) => void;
  format: FormatType;
  setFormat: (format: FormatType) => void;
  activeView: "tree" | "graph";
  setActiveView: (view: "tree" | "graph") => void;
}
export const useEditorStore = create(
  persist<EditorState>(
    (set) => ({
      content: "",
      setContent: (content: string) => set(() => ({ content })),
      format: "json",
      setFormat: (format: FormatType) => set(() => ({ format })),
      activeView: "graph",
      setActiveView: (view: "tree" | "graph") =>
        set(() => ({ activeView: view })),
    }),
    {
      name: "editor-storage",
    }
  )
);

interface SchemaToolsState {
  sourceContent: string;
  setSourceContent: (content: string) => void;
  targetContent: string;
  setTargetContent: (content: string) => void;
}

export const useSchemaToolsStore = create(
  persist<SchemaToolsState>(
    (set) => ({
      sourceContent: "",
      setSourceContent: (content: string) =>
        set(() => ({ sourceContent: content })),
      targetContent: "",
      setTargetContent: (content: string) =>
        set(() => ({ targetContent: content })),
    }),
    {
      name: "schema-tools-storage",
    }
  )
);

interface ConverterState {
  sourceContent: string;
  setSourceContent: (content: string) => void;
  targetContent: string;
  setTargetContent: (content: string) => void;
  sourceFormat: FormatType;
  setSourceFormat: (format: FormatType) => void;
  targetFormat: FormatType;
  setTargetFormat: (format: FormatType) => void;
}

export const useConverterStore = create(
  persist<ConverterState>(
    (set) => ({
      sourceContent: "",
      setSourceContent: (content: string) =>
        set(() => ({ sourceContent: content })),
      targetContent: "",
      setTargetContent: (content: string) =>
        set(() => ({ targetContent: content })),
      sourceFormat: "json",
      setSourceFormat: (format: FormatType) =>
        set(() => ({ sourceFormat: format })),
      targetFormat: "yaml",
      setTargetFormat: (format: FormatType) =>
        set(() => ({ targetFormat: format })),
    }),
    {
      name: "converter-storage",
    }
  )
);

interface TypeGeneratorState {
  sourceContent: string;
  setSourceContent: (content: string) => void;
  targetContent: string;
  setTargetContent: (content: string) => void;
  sourceFormat: FormatType;
  setSourceFormat: (format: FormatType) => void;
  targetFormat: TypeGeneratorType;
  setTargetFormat: (format: TypeGeneratorType) => void;
}

export const useTypeGeneratorStore = create(
  persist<TypeGeneratorState>(
    (set) => ({
      sourceContent: "",
      setSourceContent: (content: string) =>
        set(() => ({ sourceContent: content })),
      targetContent: "",
      setTargetContent: (content: string) =>
        set(() => ({ targetContent: content })),
      sourceFormat: "json",
      setSourceFormat: (format: FormatType) =>
        set(() => ({ sourceFormat: format })),
      targetFormat: "typescript",
      setTargetFormat: (format: TypeGeneratorType) =>
        set(() => ({ targetFormat: format })),
    }),
    {
      name: "type-generator-storage",
    }
  )
);
