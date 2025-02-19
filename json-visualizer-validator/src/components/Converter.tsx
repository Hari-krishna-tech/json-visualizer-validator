import React from "react";
import { ArrowRight, Copy } from "lucide-react";


const Converter: React.FC = () => {
    return (
        <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Format Converter
        </h2>
      </div>

      <div className="grid grid-cols-2 gap-8">
        <div className="space-y-4">
          <select className="w-full px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-gray-700 dark:text-gray-200">
            <option value="json">JSON</option>
            <option value="yaml">YAML</option>
            <option value="xml">XML</option>
            <option value="csv">CSV</option>
          </select>

          <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 p-4 h-[500px]">
            <textarea
              className="w-full h-full bg-transparent resize-none focus:outline-none font-mono text-sm text-gray-900 dark:text-gray-100"
              placeholder="Enter source format..."
            />
          </div>
        </div>

        <div className="space-y-4">
          <select className="w-full px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-gray-700 dark:text-gray-200">
            <option value="yaml">YAML</option>
            <option value="json">JSON</option>
            <option value="xml">XML</option>
            <option value="csv">CSV</option>
          </select>

          <div className="relative border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 p-4 h-[500px]">
            <div className="absolute top-4 right-4">
              <button className="p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-colors">
                <Copy className="w-4 h-4 text-gray-600 dark:text-gray-400" />
              </button>
            </div>
            <pre className="font-mono text-sm text-gray-900 dark:text-gray-100 h-full">
              Converted output will appear here...
            </pre>
          </div>
        </div>
      </div>
    </div>
    )
}

export default Converter;