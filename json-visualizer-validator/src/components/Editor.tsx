import React, { useEffect, useState } from "react";
import MonacoEditor from "@monaco-editor/react";
import { useEditorStore } from "../store/editorStore";
import formatXML from "xml-formatter";
import YAML from "yaml";
import Papa from "papaparse";
import * as wasmModule from "lib";
import JSONVisualizer from "./JsonVisualizer";

interface WasmModule {
  process_json: (json: string) => string;
}
interface EditorProps {
  isDarkMode: boolean;
}
/*
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
// interface Node {
//   id: string;
//   label: string;
//   value: string;
//   depth: number;
//   parent: string | null;
//   is_leaf: boolean;
//   x?: number;
//   y?: number;
// }

// interface Link {
//   source: string;
//   target: string;
// }

// interface GraphData {
//   nodes: Node[];
//   links: Link[];
// }

const Editor: React.FC<EditorProps> = ({ isDarkMode }) => {
  const { content, setContent, format, setFormat } = useEditorStore();
  const [error, setError] = useState("");
  const [wasm, setWasm] = useState<WasmModule | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [activeView, setActiveView] = useState<"tree" | "graph">("graph");
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
      } catch (err) {
        console.error("Failed to initialize WASM module:", err);
        setError("Failed to load conversion tools. Please refresh the page.");
        setIsLoading(false);
      }
    };

    initWasm();
  }, []);

  const handleContentChange = (value: string | undefined) => {
    if (value) {
      setContent(value);
      console.log(value);

      try {
        if (wasm) {
          console.log("wasm", wasm);
          // const result: GraphData =
          const result: any = wasm.process_json(value);
          if (activeView === "tree") {
            setActiveView("tree");
            // console.log(result);
            //*///*///*///*///*///*///*///renderTree(result);
          } else {
            // //renderGraph(result);
            console.log(result);
            setGraphData(result);
          }
        }
      } catch (error) {
        console.error("Error parsing content:", error);
        error instanceof Error && setError(error.message);
        /*clearAndShowError(
          error instanceof Error ? error.message : "Invalid JSON"
        );*/
      }
    }
  };

  // Clear visualization area and show error message
  // const clearAndShowError = (message: string) => {
  //   if (!d3Container.current) return;

  //   d3.select(d3Container.current).selectAll("*").remove();

  //   d3.select(d3Container.current)
  //     .append("div")
  //     .attr("class", "error-message")
  //     .style("color", "red")
  //     .style("padding", "20px")
  //     .text(`Error: ${message}`);
  // };

  // Toggle between tree and graph views
  /*  const toggleView = () => {
    const newView = activeView === "tree" ? "graph" : "tree";
    setActiveView(newView);

    if (content && wasm) {
      try {
        const result: JsonOutput = JSON.parse(wasm.process_json(content));

        if (newView === "tree") {
          // renderTree(result);
        } else {
          // renderForceGraph(result);
        }
      } catch (err) {
        console.error("Error toggling view:", err);
        clearAndShowError("Failed to update visualization");
      }
    }
  };
*/

  /* const renderTree = (data: JsonOutput) => {
    const container = d3Container.current;
    if (!container) return;

    // Clear previous SVG
    d3.select(container).selectAll("*").remove();

    const svg = d3
      .select(container)
      .append("svg")
      .attr("width", 800)
      .attr("height", 600);

    // Convert flat data to hierarchy
    const rootNode = data.nodes.find((n) => n.parent === null);
    if (!rootNode) return;

    const stratify = d3
      .stratify<JsonNode>()
      .id((d) => d.id)
      .parentId((d) => d.parent || "");

    const root = stratify(data.nodes);

    const treeLayout = d3
      .tree<JsonNode>()
      .size([600, 400])
      .separation((a, b) => (a.parent === b.parent ? 1 : 2))(root);

    // Draw links
    svg
      .append("g")
      .attr("fill", "none")
      .attr("stroke", "#555")
      .selectAll("path")
      .data(treeLayout.links())
      .enter()
      .append("path")
      .attr(
        "d",
        d3
          .linkVertical<d3.HierarchyLink<JsonNode>, unknown>()
          .x((d) => (d as any).x)
          .y((d) => (d as any).y)
      );

    // Draw nodes
    const nodes = svg
      .append("g")
      .selectAll("g")
      .data(root.descendants())
      .enter()
      .append("g")
      .attr("transform", (d) => `translate(${d.x},${d.y})`);

    nodes
      .append("rect")
      .attr("width", 100)
      .attr("height", 30)
      .attr("fill", "#4CAF50")
      .attr("x", -50) // Center the rectangle
      .attr("y", -15);
  };
*/

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
            value={format}
            onChange={(e) =>
              setFormat(e.target.value as "json" | "xml" | "yaml" | "csv")
            }
            className="px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-gray-700 dark:text-gray-200"
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
              <div className="absolute inset-0">
                <JSONVisualizer
                  key={isDarkMode ? "dark" : "light"}
                  data={graphData}
                  isDarkMode={isDarkMode}
                />
              </div>
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
