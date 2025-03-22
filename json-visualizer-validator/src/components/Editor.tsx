import React, { useEffect, useState } from "react";
import MonacoEditor from "@monaco-editor/react";
import { useEditorStore } from "../store/editorStore";
import formatXML from "xml-formatter";
import YAML from "yaml";
import Papa from "papaparse";
import * as wasmModule from "lib";
import JSONVisualizer from "./JsonVisualizer";
import { tree } from "d3";
import TreeVisualizer from "./TreeVisualizer";

interface WasmModule {
  process_json: (json: string) => string;
  process_json_tree: (json: string) => string;
}
interface EditorProps {
  isDarkMode: boolean;
}
/*
for tree 
{
  "person": {
    "name": "Jane",
    "details": {
      "age": 28,
      "occupation": "Engineer"
    }
  }
}
{
  "name": "root",
  "children": [
    {
      "name": "person",
      "children": [
        {
          "name": "name",
          "value": "\"Jane\""
        },
        {
          "name": "details",
          "children": [
            {
              "name": "age",
              "value": "28"
            },
            {
              "name": "occupation",
              "value": "\"Engineer\""
            }
          ]
        }
      ]
    }
  ]
}
{
  "company": {
    "name": "Tech Solutions",
    "founded": 2010,
    "departments": [
      {
        "name": "Engineering",
        "employees": [
          {"id": 101, "name": "Alice", "skills": ["JavaScript", "Rust", "WebAssembly"]},
          {"id": 102, "name": "Bob", "skills": ["Python", "Data Science"]}
        ]
      },
      {
        "name": "Marketing",
        "employees": [
          {"id": 201, "name": "Charlie", "specialization": "Content"}
        ]
      }
    ],
    "active": true
  }
}
{
  "name": "root",
  "children": [
    {
      "name": "company",
      "children": [
        {
          "name": "name",
          "value": "\"Tech Solutions\""
        },
        {
          "name": "founded",
          "value": "2010"
        },
        {
          "name": "departments",
          "children": [
            {
              "name": "departments[0]",
              "children": [
                {
                  "name": "name",
                  "value": "\"Engineering\""
                },
                {
                  "name": "employees",
                  "children": [
                    {
                      "name": "employees[0]",
                      "children": [
                        {
                          "name": "id",
                          "value": "101"
                        },
                        {
                          "name": "name",
                          "value": "\"Alice\""
                        },
                        {
                          "name": "skills",
                          "children": [
                            {
                              "name": "skills[0]",
                              "value": "\"JavaScript\""
                            },
                            {
                              "name": "skills[1]",
                              "value": "\"Rust\""
                            },
                            {
                              "name": "skills[2]",
                              "value": "\"WebAssembly\""
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "name": "employees[1]",
                      "children": [
                        {
                          "name": "id",
                          "value": "102"
                        },
                        {
                          "name": "name",
                          "value": "\"Bob\""
                        },
                        {
                          "name": "skills",
                          "children": [
                            {
                              "name": "skills[0]",
                              "value": "\"Python\""
                            },
                            {
                              "name": "skills[1]",
                              "value": "\"Data Science\""
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "name": "departments[1]",
              "children": [
                {
                  "name": "name",
                  "value": "\"Marketing\""
                },
                {
                  "name": "employees",
                  "children": [
                    {
                      "name": "employees[0]",
                      "children": [
                        {
                          "name": "id",
                          "value": "201"
                        },
                        {
                          "name": "name",
                          "value": "\"Charlie\""
                        },
                        {
                          "name": "specialization",
                          "value": "\"Content\""
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        {
          "name": "active",
          "value": "true"
        }
      ]
    }
  ]
}

{
  nodes: [
    { id: "1", label: "Object", value: "4 items", depth: 0, parent: null, is_leaf: false },
    { id: "2", label: "Value", value: "John", depth: 1, parent: "1", is_leaf: true },
    { id: "3", label: "Value", value: "30", depth: 1, parent: "1", is_leaf: true },
    { id: "4", label: "Object", value: "2 items", depth: 1, parent: "1", is_leaf: false },
    // ... more nodes
  ],
  links: [
    { source: "1", target: "2" },
    { source: "1", target: "3" },
    { source: "1", target: "4" },
    // ... more links
  ]
}
*/

const Editor: React.FC<EditorProps> = ({ isDarkMode }) => {
  const { content, setContent, format, setFormat, activeView, setActiveView } =
    useEditorStore();
  const [error, setError] = useState("");
  const [wasm, setWasm] = useState<WasmModule | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  //const [activeView, setActiveView] = useState<"tree" | "graph">("graph");
  const [graphData, setGraphData] = useState<any | null>(null);

  const formatContent = () => {
    let formatted = "";

    try {
      switch (format) {
        case "json":
          const json = JSON.parse(content);
          formatted = JSON.stringify(json, null, 2);
          break;
        case "xml":
          formatted = formatXML(content, { indentation: "  " });
          break;
        case "yaml":
          const yamlObj = YAML.parse(content);
          formatted = YAML.stringify(yamlObj, { indent: 2 });
          break;
        case "csv":
          const csvData = Papa.parse(content, { header: true });
          formatted = Papa.unparse(csvData.data, { quotes: true });
          break;
        default:
          formatted = content;
      }
      setContent(formatted);
      setError("");
    } catch (error) {
      console.error("Error parsing content:", error);
      error instanceof Error && setError(error.message);
    }
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
        // handleContentChange(content);
      } catch (err) {
        console.error("Failed to initialize WASM module:", err);
        setError("Failed to load conversion tools. Please refresh the page.");
        setIsLoading(false);
      }
    };

    initWasm();
  }, []);

  useEffect(() => {
    handleContentChange(content);
  }, [wasm, activeView]);

  const handleContentChange = (value: string | undefined) => {
    if (value) {
      setContent(value);
      console.log(value);

      try {
        if (wasm) {
          console.log("wasm", wasm);
          setError("");
          if (activeView === "tree") {
            const result: any = wasm.process_json_tree(value);
            setGraphData(result);
            console.log(result);
          } else {
            const result: any = wasm.process_json(value);
            console.log("after process_json " + result);
            setGraphData(result);
            // setActiveView("graph");
          }
        } else {
          console.log(wasm === null);
        }
      } catch (error) {
        console.error("Error parsing content:", error);
        error instanceof Error && setError(error.message);
      }
    }
  };

  // check format is correct
  /*
  const isGraphFormat = (graphDataForChecking: string) => {
    try {
      const graphData = JSON.parse(graphDataForChecking);
      console.log("inside isGraphFormat", graphData);
      if (!graphData.nodes || !graphData.links) {
        return false;
      }
      return true;
    } catch (error) {
      return false;
    }
  };

  const isTreeFormat = (graphDataForChecking: string) => {
    try {
      const graphData = JSON.parse(graphDataForChecking);
      if (!graphData.name || !graphData.children) {
        return false;
      }
      return true;
    } catch (error) {
      return false;
    }
  }; */
  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-[600px]">
        <p className="text-gray-500 dark:text-gray-400">Loading...</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Code Editor
        </h2>
        <div className="flex space-x-2">
          <select
            value={activeView}
            onChange={(e) => setActiveView(e.target.value as "tree" | "graph")}
            className="px-4 py-2 bg-white dark:bg-gray-800 border-2 border-blue-500 
            dark:border-blue-400 rounded-lg shadow-md text-gray-700 dark:text-gray-200 
            font-medium transition-all duration-200 hover:border-blue-600 
            focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 
            cursor-pointer appearance-none min-w-[120px]
            bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyMCAyMCI+PHBhdGggZD0iTTUuMjkzIDcuMjkzYTEgMSAwIDAxMS40MTQgMEwxMCAxMC41ODZsNC4yOTMtMy4yOTNhMSAxIDAgMTExLjQxNCAxLjQxNGwtNSA1YTEgMSAwIDAxLTEuNDE0IDBsLTUtNWExIDEgMCAwMTAtMS40MTR6Ii8+PC9zdmc+')] 
            bg-[length:1.5em_1.5em] bg-no-repeat bg-[right_0.5em_center] pr-10"
          >
            <option value="tree">Tree</option>
            <option value="graph">Graph</option>
          </select>
          <select
            value={format}
            onChange={(e) =>
              setFormat(e.target.value as "json" | "xml" | "yaml" | "csv")
            }
            className="px-4 py-2 bg-white dark:bg-gray-800 border-2 border-blue-500 
            dark:border-blue-400 rounded-lg shadow-md text-gray-700 dark:text-gray-200 
            font-medium transition-all duration-200 hover:border-blue-600 
            focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 
            cursor-pointer appearance-none min-w-[120px]
            bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyMCAyMCI+PHBhdGggZD0iTTUuMjkzIDcuMjkzYTEgMSAwIDAxMS40MTQgMEwxMCAxMC41ODZsNC4yOTMtMy4yOTNhMSAxIDAgMTExLjQxNCAxLjQxNGwtNSA1YTEgMSAwIDAxLTEuNDE0IDBsLTUtNWExIDEgMCAwMTAtMS40MTR6Ii8+PC9zdmc+')] 
            bg-[length:1.5em_1.5em] bg-no-repeat bg-[right_0.5em_center] pr-10"
          >
            <option value="json">JSON</option>
            <option value="xml">XML</option>
            <option value="yaml">YAML</option>
            <option value="csv">CSV</option>
          </select>
          <button
            onClick={formatContent}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
          >
            Format
          </button>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-4 h-[600px]">
        <div className="col-span-1 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900">
          <div className="font-mono text-sm h-full">
            <MonacoEditor
              height="100%"
              language={format === "csv" ? "plaintext" : format}
              value={content}
              onChange={handleContentChange}
              theme={isDarkMode ? "vs-dark" : "vs"}
              options={{
                roundedSelection: false,
                scrollBeyondLastLine: false,
                scrollbar: {
                  vertical: "visible",
                  horizontal: "visible",
                },
                readOnly: false,
                minimap: {
                  enabled: false,
                },
              }}
            />
            {error && (
              <div
                className="fixed bottom-4 left-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded shadow-lg"
                onClick={() => setError("")}
              >
                <p className="font-medium">Error</p>
                <p className="text-sm">{error}</p>
              </div>
            )}
          </div>
        </div>

        <div className="col-span-2 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900  h-full overflow-hidden">
          <div className="h-full w-full relative">
            {graphData ? (
              activeView === "graph" ? (
                <div className="absolute inset-0">
                  <JSONVisualizer
                    key={isDarkMode ? "dark" : "light"}
                    data={graphData}
                    isDarkMode={isDarkMode}
                  />
                </div>
              ) : (
                <div className="absolute inset-0">
                  <TreeVisualizer
                    key={isDarkMode ? "dark" : "light"}
                    data={graphData}
                    isDarkMode={isDarkMode}
                  />
                </div>
              )
            ) : (
              <div className="h-full w-full flex items-center justify-center text-gray-500 dark:text-gray-400">
                Visualization will appear here
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Editor;
