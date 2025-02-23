import React, {useState} from 'react';
import MonacoEditor from '@monaco-editor/react';
import { useEditorStore } from '../store/editorStore';
import formatXML from 'xml-formatter';
import YAML from 'yaml';
import Papa from 'papaparse'


interface EditorProps {

    isDarkMode: boolean;
}

const Editor: React.FC<EditorProps> = ({isDarkMode}) => {
  const {content, setContent, format, setFormat} = useEditorStore();
  const [error, setError] = useState('');

  const formatContent = () => {
    let formatted = '';

    
    try {
      switch (format) {
        case 'json':
          const json = JSON.parse(content);
          formatted = JSON.stringify(json, null, 2);
          break;
        case 'xml':
          formatted = formatXML(content, { indentation: '  ' });
          break;
        case 'yaml':
          const yamlObj = YAML.parse(content);
          formatted = YAML.stringify(yamlObj, { indent: 2 });
          break;
        case 'csv':
          const csvData = Papa.parse(content, { header: true });
          formatted = Papa.unparse(csvData.data, { quotes: true });
          break;
        default:
          formatted = content;
      }
      setContent(formatted);
    } catch(error) {
      console.error('Error parsing content:', error);
      error instanceof Error && setError(error.message);
    }
  }
  
    return (
        <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Code Editor
        </h2>
        <div className="flex space-x-2">
          <select 
          value={format}
          onChange={(e) => setFormat(e.target.value as 'json' | 'xml' | 'yaml' | 'csv') }
          className="px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-gray-700 dark:text-gray-200">
            <option value="json">JSON</option>
            <option value="xml">XML</option>
            <option value="yaml">YAML</option>
            <option value="csv">CSV</option>
          </select>
          <button onClick={formatContent} className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors">
            Format
          </button>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4 h-[600px]">
        <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900">
          <div className="font-mono text-sm h-full">
            {/* Editor will be mounted here */}
            <MonacoEditor
              height="100%"
              language={format === 'csv' ? 'plaintext': format}
              value={content}
              onChange={(newValue) => setContent(newValue || '')}
              theme={isDarkMode ? 'vs-dark' : 'vs'}
              options={
                {
                  roundedSelection: false,
                  scrollBeyondLastLine: false,
                  scrollbar: {
                    vertical: 'visible',
                    horizontal: 'visible',
                  },
                  readOnly: false,
                  minimap: {
                    enabled: false
                  }
                }
              }
            />
            {error && (
  <div className="fixed bottom-4 left-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded shadow-lg" onClick={()=>setError("")}>
    <p className="font-medium">Error</p>
    <p className="text-sm">{error}</p>
  </div>
)}
          </div>
          
        </div>

        <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 p-4">
          <div className="font-mono text-sm h-full text-gray-900 dark:text-gray-100">
            {/* Visualization will be shown here */}
            <div className="h-full flex items-center justify-center text-gray-500 dark:text-gray-400">
              Visualization will appear here
            </div>
          </div>
        </div>
      </div>
    </div>
    )
}

export default Editor;