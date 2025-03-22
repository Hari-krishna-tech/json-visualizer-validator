import React, { useEffect, useState } from "react";
import { Copy } from "lucide-react";
import MonacoEditor from "@monaco-editor/react";
import { useEditorStore, useTypeGeneratorStore } from "../store/editorStore";
import * as wasmModule from "lib";

interface WasmModule {
  json_to_typescript: (json: string) => string;
  json_to_java: (json: string) => string;
  json_to_golang: (json: string) => string;
  json_to_python: (json: string) => string;
  yaml_to_typescript: (yaml: string) => string;
  yaml_to_java: (yaml: string) => string;
  yaml_to_golang: (yaml: string) => string;
  yaml_to_python: (yaml: string) => string;
  xml_to_typescript: (xml: string) => string;
  xml_to_java: (xml: string) => string;
  xml_to_golang: (xml: string) => string;
  xml_to_python: (xml: string) => string;
  csv_to_typescript: (csv: string) => string;
  csv_to_java: (csv: string) => string;
  csv_to_golang: (csv: string) => string;
  csv_to_python: (csv: string) => string;
}

interface TypeGeneratorProps {
  isDarkMode: boolean;
}
const TypeGenerator: React.FC<TypeGeneratorProps> = ({ isDarkMode }) => {
  const {
    sourceContent,
    setSourceContent,
    sourceFormat,
    setSourceFormat,
    targetContent,
    targetFormat,
    setTargetContent,
    setTargetFormat,
  } = useTypeGeneratorStore();
  // const [sourceContent, setSourceContent] = useState<string | undefined>("");
  // const [targetContent, setTargetContent] = useState("");
  // const [sourceFormat, setSourceFormat] = useState("json");
  // const [targetFormat, setTargetFormat] = useState("typescript");
  const { content: globalContent } = useEditorStore();
  const [wasm, setWasm] = useState<WasmModule | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadFromEditor = () => {
    setSourceContent(globalContent);
    handleSourceContentChange(globalContent);
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(targetContent);
  };

  const handleSourceFormatChange = (
    e: React.ChangeEvent<HTMLSelectElement>
  ) => {
    setSourceFormat(e.target.value as "json" | "yaml" | "xml" | "csv");
  };

  const handleTargetFormatChange = (
    e: React.ChangeEvent<HTMLSelectElement>
  ) => {
    setTargetFormat(e.target.value as "typescript" | "java" | "go" | "python");
  };

  // load wasm module

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
  useEffect(() => {});

  useEffect(() => {
    handleSourceContentChange(sourceContent);
  }, [sourceFormat, targetFormat]);

  const handleSourceContentChange = (value: string | undefined) => {
    setSourceContent(value || "");
    if (value === "") {
      return setTargetContent("");
    }
    if (!wasm) return;
    if (sourceFormat === "json") {
      switch (targetFormat) {
        case "typescript":
          setTargetContent(wasm.json_to_typescript(value || "") || "");
          break;
        case "java":
          setTargetContent(wasm.json_to_java(value || "") || "");
          break;
        case "go":
          setTargetContent(wasm.json_to_golang(value || "") || "");
          break;
        case "python":
          setTargetContent(wasm.json_to_python(value || "") || "");
          break;
        default:
          break;
      }
    } else if (sourceFormat === "yaml") {
      switch (targetFormat) {
        case "typescript":
          setTargetContent(wasm.yaml_to_typescript(value || "") || "");
          break;
        case "java":
          setTargetContent(wasm.yaml_to_java(value || "") || "");
          break;
        case "go":
          setTargetContent(wasm.yaml_to_golang(value || "") || "");
          break;
        case "python":
          setTargetContent(wasm.yaml_to_python(value || "") || "");
          break;
        default:
          break;
      }
    } else if (sourceFormat === "xml") {
      switch (targetFormat) {
        case "typescript":
          setTargetContent(wasm.xml_to_typescript(value || "") || "");
          break;
        case "java":
          setTargetContent(wasm.xml_to_java(value || "") || "");
          break;
        case "go":
          setTargetContent(wasm.xml_to_golang(value || "") || "");
          break;
        case "python":
          setTargetContent(wasm.xml_to_python(value || "") || "");
          break;
        default:
          break;
      }
    } else if (sourceFormat === "csv") {
      switch (targetFormat) {
        case "typescript":
          setTargetContent(wasm.csv_to_typescript(value || "") || "");
          break;
        case "java":
          setTargetContent(wasm.csv_to_java(value || "") || "");
          break;
        case "go":
          setTargetContent(wasm.csv_to_golang(value || "") || "");
          break;
        case "python":
          setTargetContent(wasm.csv_to_python(value || "") || "");
          break;
        default:
          break;
      }
    }
  };

  if (isLoading || !wasm || error) {
    return (
      <div className="flex justify-center items-center h-[500px]">
        {isLoading && <p>Loading...</p>}
        {error && <p className="text-red-500">{error}</p>}
      </div>
    );
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
              onChange={handleSourceFormatChange}
              className="flex-grow px-4 py-2 bg-white dark:bg-gray-800 border-2 border-blue-500 
            dark:border-blue-400 rounded-lg shadow-md text-gray-700 dark:text-gray-200 
            font-medium transition-all duration-200 hover:border-blue-600 
            focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 
            cursor-pointer appearance-none min-w-[120px]
            bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyMCAyMCI+PHBhdGggZD0iTTUuMjkzIDcuMjkzYTEgMSAwIDAxMS40MTQgMEwxMCAxMC41ODZsNC4yOTMtMy4yOTNhMSAxIDAgMTExLjQxNCAxLjQxNGwtNSA1YTEgMSAwIDAxLTEuNDE0IDBsLTUtNWExIDEgMCAwMTAtMS40MTR6Ii8+PC9zdmc+')] 
            bg-[length:1.5em_1.5em] bg-no-repeat bg-[right_0.5em_center] pr-100"
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
              onChange={handleSourceContentChange}
              theme={isDarkMode ? "vs-dark" : "vs"}
            />
          </div>
        </div>

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
            <option value="typescript">Typescript</option>
            <option value="java">Java</option>
            <option value="go">Golang</option>
            <option value="python">Python</option>
          </select>

          <div className="relative border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-800 h-[500px]">
            <div className="absolute top-4 right-4 z-10">
              <button
                className="p-2 bg-gray-400 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-lg transition-colors"
                title="Copy to clipboard"
              >
                <Copy
                  onClick={copyToClipboard}
                  className="w-5 h-5 text-gray-200 dark:text-gray-200"
                />
              </button>
            </div>
            <MonacoEditor
              height="100%"
              language={targetFormat}
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

export default TypeGenerator;
