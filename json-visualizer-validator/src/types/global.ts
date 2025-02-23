export type FormatType = 'json' | 'yaml' | 'xml' | 'csv';


export interface EditorState {
    content: string;
    format: FormatType;
}