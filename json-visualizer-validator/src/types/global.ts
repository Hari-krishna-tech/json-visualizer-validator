export type FormatType = "json" | "yaml" | "xml" | "csv";

export type TypeGeneratorType = "typescript" | "java" | "go" | "python";

export interface EditorState {
  content: string;
  format: FormatType;
}
