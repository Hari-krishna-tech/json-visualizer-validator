import React, { use, useEffect, useState } from "react";

import { Copy } from "lucide-react";
import MonacoEditor from "@monaco-editor/react";
import { useEditorStore, useSchemaToolsStore } from "../store/editorStore";
import * as wasmModule from "lib";

interface SchemaToolsProps {
  isDarkMode: boolean;
}

interface WasmModule {
  json_to_json_schema: (json: string) => string;
}

const SchemaTools: React.FC<SchemaToolsProps> = ({ isDarkMode }) => {
  const { sourceContent, setSourceContent, targetContent, setTargetContent } =
    useSchemaToolsStore();
  // const [sourceContent, setSourceContent] = useState("");
  // const [targetContent, setTargetContent] = useState("");
  const { content: globalContent } = useEditorStore();
  const [wasm, setWasm] = useState<WasmModule | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(targetContent);
  };

  const loadFromEditor = () => {
    setSourceContent(globalContent);
    handleSourceContentChange(globalContent);
  };

  useEffect(() => {
    const initWasm = async () => {
      try {
        // Check if wasmModule has an init function
        if (typeof wasmModule.default === "function") {
          await wasmModule.default();
          setWasm(wasmModule as unknown as WasmModule);
        } else {
          // If no init function, assume it's already initialized
          setWasm(wasmModule as unknown as WasmModule);
        }
        setIsLoading(false);
      } catch (err) {
        console.error("Failed to initialize WASM module:", err);
        setError("Failed to load conversion tools. Please refresh the page.");
        setIsLoading(false);
      }
    };

    initWasm();
  }, []);

  const handleSourceContentChange = (content: string | undefined) => {
    if (wasm) {
      try {
        setTargetContent(wasm.json_to_json_schema(content || "") || "");
        setError(null);
      } catch (err) {
        console.error("Failed to convert JSON to JSON Schema:", err);
        setError("Failed to convert JSON to JSON Schema.");
      }
    }
  };

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Schema Generator (JSON)
        </h2>
      </div>

      <div className="space-y-4 flex items-end">
        <button
          onClick={loadFromEditor}
          className="px-4 py-2 bg-blue-500 dark:bg-blue-600 text-white rounded-md hover:bg-blue-600 dark:hover:bg-blue-700 transition-colors"
        >
          Load from Editor
        </button>
      </div>

      <div className="grid grid-cols-2 gap-8">
        <div className="space-y-4">
          <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 h-[500px]">
            <MonacoEditor
              height="100%"
              language="json"
              value={sourceContent}
              onChange={handleSourceContentChange}
              theme={isDarkMode ? "vs-dark" : "vs"}
            />
          </div>
        </div>

        <div className="space-y-4">
          <div className="relative border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 h-[500px]">
            <div className="absolute top-4 right-4 z-10">
              <button className="p-2 bg-gray-400 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-lg transition-colors">
                <Copy
                  onClick={copyToClipboard}
                  className="w-5 h-5 text-gray-200 dark:text-gray-200"
                />
              </button>
            </div>
            <MonacoEditor
              height="100%"
              language="json"
              value={targetContent}
              onChange={(value) => setTargetContent(value || "")}
              theme={isDarkMode ? "vs-dark" : "vs"}
              options={{ readOnly: true }}
            />
            {error && (
              <div className="absolute inset-0 flex items-center justify-center bg-red-500 bg-opacity-10 text-red-700 dark:bg-red-600 dark:bg-opacity-10 dark:text-red-300">
                {error}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default SchemaTools;
