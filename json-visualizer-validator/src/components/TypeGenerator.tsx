import React, {useState} from 'react'
import {Copy} from 'lucide-react'
import MonacoEditor from '@monaco-editor/react';
import { useEditorStore } from '../store/editorStore';

interface TypeGeneratorProps {
    isDarkMode: boolean;
}
const TypeGenerator: React.FC<TypeGeneratorProps> = ({isDarkMode}) => {
    const [sourceContent, setSourceContent] = useState('');
    const [targetContent, setTargetContent] = useState('');
    const [sourceFormat, setSourceFormat] = useState('json');
    const [targetFormat, setTargetFormat] = useState('typescript');
    const {content: globalContent} = useEditorStore();

    const loadFromEditor = () => {
        setSourceContent(globalContent);
    }

    const copyToClipboard = () => {
            navigator.clipboard.writeText(targetContent);
        }


    return (
      <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Type Generator 
        </h2>
      </div>

      <div className="grid grid-cols-2 gap-8">
        <div className="space-y-4">
          <div className="flex items-center space-x-4">
          <select
            value={sourceFormat}
            onChange={(e) => setSourceFormat(e.target.value)} 
            className="flex-grow px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300
            dark:border-gray-600 rounded-md shadow-sm text-gray-700 dark:text-gray-200 focus:ring-2 focus:ring-blue-500">
            <option value="json">JSON</option>
            <option value="yaml">YAML</option>
            <option value="xml">XML</option>
            <option value="csv">CSV</option>
          </select>
          <button 
            onClick={loadFromEditor}
            className="px-4 py-2 bg-blue-500 dark:bg-blue-600 text-white rounded-md hover:bg-blue-600 dark:hover:bg-blue-700 transition-colors"
            >
            Load from Editor
          </button>
          </div>
          <div className="border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-800 h-[500px]">
          <MonacoEditor
              height="100%"
              language={sourceFormat === 'csv' ? 'plaintext' : sourceFormat}
              value={sourceContent}
              onChange={(value) => setSourceContent(value || '')}
              theme={isDarkMode ? 'vs-dark' : 'vs'}
            />
          </div>
        </div>

        <div className="space-y-4">
          <select 
            value={targetFormat}
            onChange={(e) => setTargetFormat(e.target.value)}
            className="w-full px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-gray-700 dark:text-gray-200 focus:ring-2 focus:ring-blue-500">
            <option value="typescript">Typescript</option>
            <option value="java">Java</option>
            <option value="golang">Golang</option>
            <option value="python">Python</option>
          </select>

          <div className="relative border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-800 h-[500px]">
            <div className="absolute top-4 right-4 z-10">
              <button className="p-2 bg-gray-400 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-lg transition-colors" 
               title="Copy to clipboard">
                <Copy onClick={copyToClipboard} className="w-5 h-5 text-gray-200 dark:text-gray-200" />
              </button>
            </div>
            <MonacoEditor
              height="100%"
              language={targetFormat === 'csv' ? 'plaintext' : targetFormat}
              value={targetContent}
              onChange={(value) => setTargetContent(value || '')}
              theme={isDarkMode ? 'vs-dark' : 'vs'}
              options={{ readOnly: true }}
            />
          </div>
        </div>
      </div>
    </div>
    );
}

export default TypeGenerator; 