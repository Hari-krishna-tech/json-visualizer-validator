import React, {useState} from 'react';

import {Copy} from 'lucide-react'



const SchemaTools: React.FC = () => {
    const [activeTab, setActiveTab] = useState<'generator' | 'validator'>('generator');



    return (
        <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          JSON Schema Tools
        </h2>
        <div className="flex space-x-2">
          <button
            onClick={() => setActiveTab('generator')}
            className={`px-4 py-2 rounded-md transition-colors ${
              activeTab === 'generator'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200'
            }`}
          >
            Generator
          </button>
          <button
            onClick={() => setActiveTab('validator')}
            className={`px-4 py-2 rounded-md transition-colors ${
              activeTab === 'validator'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200'
            }`}
          >
            Validator
          </button>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4 h-[600px]">
        <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 p-4">
          <textarea
            className="w-full h-full bg-transparent resize-none focus:outline-none font-mono text-sm text-gray-900 dark:text-gray-100"
            placeholder={activeTab === 'generator' ? "Enter JSON data..." : "Enter JSON Schema..."}
          />
        </div>

        <div className="relative border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 p-4">
          <div className="absolute top-4 right-4">
            <button className="p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-colors">
              <Copy className="w-4 h-4 text-gray-600 dark:text-gray-400" />
            </button>
          </div>
          <pre className="font-mono text-sm text-gray-900 dark:text-gray-100 h-full">
            {activeTab === 'generator' 
              ? "Generated schema will appear here..."
              : "Validation results will appear here..."}
          </pre>
        </div>
      </div>
    </div>
    );
}

export default SchemaTools;