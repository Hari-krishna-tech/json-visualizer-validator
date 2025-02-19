import React from 'react';

const Editor: React.FC = () => {
    return (
        <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Code Editor
        </h2>
        <div className="flex space-x-2">
          <select className="px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-gray-700 dark:text-gray-200">
            <option value="json">JSON</option>
            <option value="xml">XML</option>
            <option value="yaml">YAML</option>
            <option value="csv">CSV</option>
          </select>
          <button className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors">
            Format
          </button>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4 h-[600px]">
        <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 p-4">
          <div className="font-mono text-sm h-full">
            {/* Editor will be mounted here */}
            <textarea
              className="w-full h-full bg-transparent resize-none focus:outline-none text-gray-900 dark:text-gray-100"
              placeholder="Enter your code here..."
            />
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