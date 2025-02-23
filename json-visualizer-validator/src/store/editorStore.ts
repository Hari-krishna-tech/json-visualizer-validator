import create from 'zustand'
import { FormatType } from '../types/global';
interface EditorState {
    content: string;
    setContent: (content: string) => void;
    format: FormatType;
    setFormat: (format: FormatType) => void;
}


export const useEditorStore = create<EditorState>((set) => ({
    content: '',
    setContent: (content: string) => set(() => ({ content })),
    format: 'json',
    setFormat: (format: FormatType) => set(() => ({ format })),
}))