import React, { useState, useEffect } from "react";
import { Copy } from "lucide-react";
import MonacoEditor from "@monaco-editor/react";
import { useEditorStore, useConverterStore } from "../store/editorStore";
import * as wasmModule from "lib";

interface WasmModule {
  json_to_yaml: (json: string) => string;
  json_to_xml: (json: string) => string;
  json_to_csv: (json: string) => string;
  yaml_to_json: (yaml: string) => string;
  yaml_to_xml: (yaml: string) => string;
  yaml_to_csv: (yaml: string) => string;
  xml_to_json: (xml: string) => string;
  xml_to_yaml: (xml: string) => string;
  xml_to_csv: (xml: string) => string;
  csv_to_json: (csv: string) => string;
  csv_to_yaml: (csv: string) => string;
  csv_to_xml: (csv: string) => string;
}

interface ConverterProps {
  isDarkMode: boolean;
}
const Converter: React.FC<ConverterProps> = ({ isDarkMode }) => {
  const {
    sourceContent,
    setSourceContent,
    targetContent,
    setTargetContent,
    sourceFormat,
    setSourceFormat,
    targetFormat,
    setTargetFormat,
  } = useConverterStore();
  /*  const [sourceContent, setSourceContent] = useState("");
  const [targetContent, setTargetContent] = useState("");
  const [sourceFormat, setSourceFormat] = useState("json");
  const [targetFormat, setTargetFormat] = useState("yaml"); */
  const [wasm, setWasm] = useState<WasmModule | null>(null);
  const [isloading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const { content: globalContent, format } = useEditorStore();

  useEffect(() => {
    async function initWasm() {
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
    }

    initWasm();
  }, []);

  useEffect(() => {
    handleSourceContentChange(sourceContent);
  }, [sourceFormat, targetFormat]);

  const handleSourceFormatChange = (
    e: React.ChangeEvent<HTMLSelectElement>
  ) => {
    setSourceFormat(e.target.value as "json" | "yaml" | "xml" | "csv");
  };
  const handleTargetFormatChange = (
    e: React.ChangeEvent<HTMLSelectElement>
  ) => {
    setTargetFormat(e.target.value as "xml" | "json" | "yaml" | "csv");
  };
  const handleSourceContentChange = (value: string) => {
    setSourceContent(value); // Update the state with the new value
    if (!wasm) {
      console.log("WASM not loaded");
      return;
    }
    try {
      if (sourceFormat === "json") {
        if (targetFormat === "yaml") {
          console.log(value);
          const result = wasm.json_to_yaml(value);
          console.log(result);
          setTargetContent(result);
        } else if (targetFormat === "xml") {
          const result = wasm.json_to_xml(value);
          setTargetContent(result);
        } else if (targetFormat === "csv") {
          // Convert JSON to CSV
          console.log("JSON to CSV");
          const result = wasm.json_to_csv(value);
          setTargetContent(result);
        } else {
          setTargetContent(value);
        }
      } else if (sourceFormat === "yaml") {
        if (targetFormat === "json") {
          console.log(value);
          const result = wasm.yaml_to_json(value);
          console.log(result);
          setTargetContent(result);
        } else if (targetFormat === "xml") {
          const result = wasm.yaml_to_xml(value);
          setTargetContent(result);
        } else if (targetFormat === "csv") {
          const result = wasm.yaml_to_csv(value);
          setTargetContent(result);
        } else {
          setTargetContent(value);
        }
      } else if (sourceFormat === "xml") {
        if (targetFormat === "json") {
          const result = wasm.xml_to_json(value);
          setTargetContent(result);
        } else if (targetFormat === "yaml") {
          const result = wasm.xml_to_yaml(value);
          setTargetContent(result);
        } else if (targetFormat === "csv") {
          const result = wasm.xml_to_csv(value);
          setTargetContent(result);
        } else {
          setTargetContent(value);
        }
      } else if (sourceFormat === "csv") {
        if (targetFormat === "json") {
          const result = wasm.csv_to_json(value);
          setTargetContent(result);
        } else if (targetFormat === "yaml") {
          const result = wasm.csv_to_yaml(value);
          setTargetContent(result);
        } else if (targetFormat === "xml") {
          const result = wasm.csv_to_xml(value);
          setTargetContent(result);
        } else {
          setTargetContent(value);
        }
      }
    } catch (e) {
      console.error(e);
      setTargetContent(`Error: ${e instanceof Error ? e.message : String(e)}`);
    }
  };

  const loadFromEditor = () => {
    setSourceContent(globalContent);
    setSourceFormat(format);
    handleSourceContentChange(globalContent);
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(targetContent);
  };

  if (isloading) {
    return (
      <div className="flex justify-center items-center h-64">
        Loading conversion tools...
      </div>
    );
  }
  if (error) {
    return <div className="text-red-500">{error}</div>;
  }
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Format Converter
        </h2>
      </div>

      <div className="grid grid-cols-2 gap-8">
        {/* Source Area */}
        <div className="space-y-4">
          <div className="flex items-center space-x-4">
            <select
              value={sourceFormat}
              onChange={handleSourceFormatChange}
              className="flex-grow px-4 py-2 bg-white dark:bg-gray-800 border-2 border-blue-500 
            dark:border-blue-400 rounded-lg shadow-md text-gray-700 dark:text-gray-200 
            font-medium transition-all duration-200 hover:border-blue-600 
            focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 
            cursor-pointer appearance-none min-w-[120px]
            bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyMCAyMCI+PHBhdGggZD0iTTUuMjkzIDcuMjkzYTEgMSAwIDAxMS40MTQgMEwxMCAxMC41ODZsNC4yOTMtMy4yOTNhMSAxIDAgMTExLjQxNCAxLjQxNGwtNSA1YTEgMSAwIDAxLTEuNDE0IDBsLTUtNWExIDEgMCAwMTAtMS40MTR6Ii8+PC9zdmc+')] 
            bg-[length:1.5em_1.5em] bg-no-repeat bg-[right_0.5em_center] pr-10"
            >
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
              language={sourceFormat === "csv" ? "plaintext" : sourceFormat}
              value={sourceContent}
              onChange={(value) => handleSourceContentChange(value || "")}
              theme={isDarkMode ? "vs-dark" : "vs"}
            />
          </div>
        </div>

        {/* Target Area */}
        <div className="space-y-4">
          <select
            value={targetFormat}
            onChange={handleTargetFormatChange}
            className="w-full px-4 py-2 bg-white dark:bg-gray-800 border-2 border-blue-500 
            dark:border-blue-400 rounded-lg shadow-md text-gray-700 dark:text-gray-200 
            font-medium transition-all duration-200 hover:border-blue-600 
            focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 
            cursor-pointer appearance-none min-w-[120px]
            bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyMCAyMCI+PHBhdGggZD0iTTUuMjkzIDcuMjkzYTEgMSAwIDAxMS40MTQgMEwxMCAxMC41ODZsNC4yOTMtMy4yOTNhMSAxIDAgMTExLjQxNCAxLjQxNGwtNSA1YTEgMSAwIDAxLTEuNDE0IDBsLTUtNWExIDEgMCAwMTAtMS40MTR6Ii8+PC9zdmc+')] 
            bg-[length:1.5em_1.5em] bg-no-repeat bg-[right_0.5em_center] pr-10"
          >
            <option value="yaml">YAML</option>
            <option value="json">JSON</option>
            <option value="xml">XML</option>
            <option value="csv">CSV</option>
          </select>
          <div className="relative border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-800 h-[500px]">
            <div className="absolute top-4 right-4 z-10">
              <button
                className="p-2 bg-gray-400 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-lg transition-colors flex items-center gap-2"
                title="Copy to clipboard"
                onClick={() => {
                  copyToClipboard();
                  const icon = document.querySelector(".copy-icon");
                  if (icon instanceof SVGElement) {
                    icon.innerHTML = '<path d="M20 6L9 17l-5-5"/>';
                    icon.setAttribute("stroke-width", "3");
                    setTimeout(() => {
                      icon.innerHTML =
                        '<path d="M8 17.929H6c-1.105 0-2-.912-2-2.036V5.036C4 3.912 4.895 3 6 3h8c1.105 0 2 .912 2 2.036v1.866m-6 .17h8c1.105 0 2 .91 2 2.035v10.857C20 21.088 19.105 22 18 22h-8c-1.105 0-2-.911-2-2.036V9.107c0-1.124.895-2.036 2-2.036z"/>';
                      icon.setAttribute("stroke-width", "2");
                    }, 1000);
                  }
                }}
              >
                <Copy className="w-5 h-5 text-gray-200 dark:text-gray-200 copy-icon" />
              </button>
            </div>
            <MonacoEditor
              height="100%"
              language={targetFormat === "csv" ? "plaintext" : targetFormat}
              value={targetContent}
              onChange={(value) => setTargetContent(value || "")}
              theme={isDarkMode ? "vs-dark" : "vs"}
              options={{ readOnly: true }}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Converter;
