import React, {useState} from 'react';

import {Copy} from 'lucide-react'
import MonacoEditor from '@monaco-editor/react'
import { useEditorStore } from '../store/editorStore';

interface SchemaToolsProps {

    isDarkMode: boolean;
}

const SchemaTools: React.FC<SchemaToolsProps> = ({isDarkMode}) => {
    const [activeTab, setActiveTab] = useState<'generator' | 'validator'>('generator');
    const [sourceContent, setSourceContent] = useState('');
    const [targetContent, setTargetContent] = useState('');


    const copyToClipboard = () => {
      navigator.clipboard.writeText(targetContent);
    }


    return (
             <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
         Schema Generator 
        </h2>
      </div>

      <div className="grid grid-cols-2 gap-8">
        <div className="space-y-4">
          <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 h-[500px]">
          <MonacoEditor
              height="100%"
              language='json'
              value={sourceContent}
              onChange={(value) => setSourceContent(value || '')}
              theme={isDarkMode ? 'vs-dark' : 'vs'}
            />
          </div>
        </div>

        <div className="space-y-4">

          <div className="relative border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 h-[500px]">
            <div className="absolute top-4 right-4">
              <button className="p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-colors">
                <Copy onClick={copyToClipboard} className="w-4 h-4 text-gray-600 dark:text-gray-400" />
              </button>
            </div>
            <MonacoEditor
              height="100%"
              language='json'
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

export default SchemaTools;