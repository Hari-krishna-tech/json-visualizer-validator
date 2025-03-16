import React, { useEffect, useState } from "react";
import MonacoEditor from "@monaco-editor/react";
import { useEditorStore } from "../store/editorStore";
import formatXML from "xml-formatter";
import YAML from "yaml";
import Papa from "papaparse";
import * as wasmModule from "lib";
import * as d3 from "d3";

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
interface JsonNode extends d3.SimulationNodeDatum {
  id: string;
  label: string;
  value: string;
  depth: number;
  parent: string | null;
  is_leaf: boolean;
}

interface JsonLink {
  source: string;
  target: string;
}

interface JsonOutput {
  nodes: JsonNode[];
  links: JsonLink[];
}

const Editor: React.FC<EditorProps> = ({ isDarkMode }) => {
  const { content, setContent, format, setFormat } = useEditorStore();
  const [error, setError] = useState("");
  const [wasm, setWasm] = useState<WasmModule | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const d3Container = React.useRef(null);
  const [activeView, setActiveView] = useState<"tree" | "graph">("tree");

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

      try {
        if (wasm) {
          const result: JsonOutput = JSON.parse(
            wasm.process_json(JSON.stringify(value))
          );
          if (activeView === "tree") {
            renderTree(result);
          } else {
            renderGraph(result);
          }
        }
      } catch (error) {
        console.error("Error parsing content:", error);
        error instanceof Error && setError(error.message);
        clearAndShowError(
          error instanceof Error ? error.message : "Invalid JSON"
        );
      }
    }
  };

  // Clear visualization area and show error message
  const clearAndShowError = (message: string) => {
    if (!d3Container.current) return;

    d3.select(d3Container.current).selectAll("*").remove();

    d3.select(d3Container.current)
      .append("div")
      .attr("class", "error-message")
      .style("color", "red")
      .style("padding", "20px")
      .text(`Error: ${message}`);
  };

  // Toggle between tree and graph views
  const toggleView = () => {
    const newView = activeView === "tree" ? "graph" : "tree";
    setActiveView(newView);

    if (content && wasm) {
      try {
        const result: JsonOutput = JSON.parse(wasm.process_json(content));

        if (newView === "tree") {
          renderTree(result);
        } else {
          renderForceGraph(result);
        }
      } catch (err) {
        console.error("Error toggling view:", err);
        clearAndShowError("Failed to update visualization");
      }
    }
  };
  // Render hierarchical tree visualization
  /* const renderTree = (data: JsonOutput) => {
    const container = d3Container.current;
    if (!container) return;

    // Clear previous visualization
    d3.select(container).selectAll('*').remove();

    // Set up dimensions and margins
    const margin = { top: 40, right: 90, bottom: 40, left: 90 };
    const width = 800 - margin.left - margin.right;
    const height = 600 - margin.top - margin.bottom;

    // Create SVG container
    const svg = d3.select(container)
      .append('svg')
      .attr('width', width + margin.left + margin.right)
      .attr('height', height + margin.top + margin.bottom)
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);
    
    // Add title
    svg.append('text')
      .attr('x', width / 2)
      .attr('y', -20)
      .attr('text-anchor', 'middle')
      .style('font-size', '16px')
      .style('font-weight', 'bold')
      .text('JSON Tree Visualization');

    // Find the root node
    const rootNode = data.nodes.find(n => n.parent === null);
    if (!rootNode) {
      svg.append('text')
        .attr('x', width / 2)
        .attr('y', height / 2)
        .attr('text-anchor', 'middle')
        .text('No data to display');
      return;
    }

    // Create hierarchical structure
    try {
      // Create stratify operator
      const stratify = d3.stratify<JsonNode>()
        .id(d => d.id)
        .parentId(d => d.parent || '');

      // Build hierarchy
      const root = stratify(data.nodes);
      
      // Create tree layout
      const treeLayout = d3.tree<JsonNode>()
        .size([height, width])
        .separation((a, b) => (a.parent === b.parent ? 1 : 1.5));
      
      // Apply layout
      const rootHierarchy = treeLayout(root);
      
      // Create links
      svg.append('g')
        .attr('fill', 'none')
        .attr('stroke', '#555')
        .attr('stroke-opacity', 0.6)
        .attr('stroke-width', 1.5)
        .selectAll('path')
        .data(rootHierarchy.links())
        .join('path')
        .attr('d', d3.linkHorizontal<d3.HierarchyLink<JsonNode>, any>()
          .x(d => d.y)
          .y(d => d.x)
        );
      
      // Node group
      const nodeGroup = svg.append('g')
        .selectAll('g')
        .data(rootHierarchy.descendants())
        .join('g')
        .attr('transform', d => `translate(${d.y},${d.x})`)
        .attr('cursor', 'pointer');
      
      // Add node colors based on type
      const getNodeColor = (label: string) => {
        switch (label) {
          case 'Object': return '#4CAF50';
          case 'Array': return '#2196F3';
          case 'Key': return '#E91E63';
          case 'Index': return '#3F51B5';
          case 'String': return '#FFC107';
          case 'Number': return '#9C27B0';
          case 'Boolean': return '#FF5722';
          case 'Null': return '#607D8B';
          default: return '#795548';
        }
      };
      
      // Node rectangles
      nodeGroup.append('rect')
        .attr('width', d => Math.max(80, d.data.value.length * 7 + 20))
        .attr('height', 30)
        .attr('x', d => -(Math.max(80, d.data.value.length * 7 + 20) / 2))
        .attr('y', -15)
        .attr('rx', 5)
        .attr('ry', 5)
        .attr('fill', d => getNodeColor(d.data.label))
        .attr('fill-opacity', 0.9)
        .attr('stroke', '#000')
        .attr('stroke-width', 1);
      
      // Node labels
      nodeGroup.append('text')
        .attr('dy', '0.32em')
        .attr('text-anchor', 'middle')
        .attr('font-size', '12px')
        .attr('fill', '#fff')
        .text(d => {
          // Truncate long values
          const value = .text(d => {
          // Truncate long values
          const value = d.data.value.length > 20 
            ? d.data.value.substring(0, 20) + '...' 
            : d.data.value;
          return `${d.data.label}: ${value}`;
        });
      
      // Add tooltips
      nodeGroup.append('title')
        .text(d => `ID: ${d.data.id}\nDepth: ${d.data.depth}\nType: ${d.data.label}\nValue: ${d.data.value}`);
      
      // Add zoom capability
      const zoom = d3.zoom()
        .scaleExtent([0.5, 3])
        .on('zoom', (event) => {
          svg.attr('transform', event.transform);
        });
      
      d3.select(container)
        .select('svg')
        .call(zoom as any);
      
    } catch (err) {
      console.error('Error creating tree:', err);
      svg.append('text')
        .attr('x', width / 2)
        .attr('y', height / 2)
        .attr('text-anchor', 'middle')
        .attr('fill', 'red')
        .text('Error creating visualization');
    }
  }; */
  const renderTree = (data: JsonOutput) => {
    const container = d3Container.current;
    if (!container) return;

    // Clear previous visualization
    d3.select(container).selectAll("*").remove();

    // Set up dimensions and margins
    const margin = { top: 40, right: 90, bottom: 40, left: 90 };
    const width = 800 - margin.left - margin.right;
    const height = 600 - margin.top - margin.bottom;

    // Create SVG container
    const svg = d3
      .select(container)
      .append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr("transform", `translate(${margin.left},${margin.top})`);

    // Add title
    svg
      .append("text")
      .attr("x", width / 2)
      .attr("y", -20)
      .attr("text-anchor", "middle")
      .style("font-size", "16px")
      .style("font-weight", "bold")
      .text("JSON Tree Visualization");

    // Find the root node
    const rootNode = data.nodes.find((n) => n.parent === null);
    if (!rootNode) {
      svg
        .append("text")
        .attr("x", width / 2)
        .attr("y", height / 2)
        .attr("text-anchor", "middle")
        .text("No data to display");
      return;
    }

    // Create hierarchical structure
    try {
      // Create stratify operator
      const stratify = d3
        .stratify<JsonNode>()
        .id((d) => d.id)
        .parentId((d) => d.parent || "");

      // Build hierarchy
      const root = stratify(data.nodes);

      // Create tree layout
      const treeLayout = d3
        .tree<JsonNode>()
        .size([height, width])
        .separation((a, b) => (a.parent === b.parent ? 1 : 1.5));

      // Apply layout
      const rootHierarchy = treeLayout(root);

      // Create links
      svg
        .append("g")
        .attr("fill", "none")
        .attr("stroke", "#555")
        .attr("stroke-opacity", 0.6)
        .attr("stroke-width", 1.5)
        .selectAll("path")
        .data(rootHierarchy.links())
        .join("path")
        .attr(
          "d",
          d3
            .linkHorizontal<d3.HierarchyLink<JsonNode>, any>()
            .x((d) => d.y)
            .y((d) => d.x)
        );

      // Node group
      const nodeGroup = svg
        .append("g")
        .selectAll("g")
        .data(rootHierarchy.descendants())
        .join("g")
        .attr("transform", (d) => `translate(${d.y},${d.x})`)
        .attr("cursor", "pointer");

      // Add node colors based on type
      const getNodeColor = (label: string) => {
        switch (label) {
          case "Object":
            return "#4CAF50";
          case "Array":
            return "#2196F3";
          case "Key":
            return "#E91E63";
          case "Index":
            return "#3F51B5";
          case "String":
            return "#FFC107";
          case "Number":
            return "#9C27B0";
          case "Boolean":
            return "#FF5722";
          case "Null":
            return "#607D8B";
          default:
            return "#795548";
        }
      };

      // Node rectangles
      nodeGroup
        .append("rect")
        .attr("width", (d) => Math.max(80, d.data.value.length * 7 + 20))
        .attr("height", 30)
        .attr("x", (d) => -(Math.max(80, d.data.value.length * 7 + 20) / 2))
        .attr("y", -15)
        .attr("rx", 5)
        .attr("ry", 5)
        .attr("fill", (d) => getNodeColor(d.data.label))
        .attr("fill-opacity", 0.9)
        .attr("stroke", "#000")
        .attr("stroke-width", 1);

      // Node labels
      nodeGroup
        .append("text")
        .attr("dy", "0.32em")
        .attr("text-anchor", "middle")
        .attr("font-size", "12px")
        .attr("fill", "#fff")
        .text((d) => {
          // Truncate long values
          const value =
            d.data.value.length > 20
              ? d.data.value.substring(0, 20) + "..."
              : d.data.value;
          return `${d.data.label}: ${value}`;
        });

      // Add tooltips
      nodeGroup
        .append("title")
        .text(
          (d) =>
            `ID: ${d.data.id}\nDepth: ${d.data.depth}\nType: ${d.data.label}\nValue: ${d.data.value}`
        );

      // Add zoom capability
      const zoom = d3
        .zoom()
        .scaleExtent([0.5, 3])
        .on("zoom", (event) => {
          svg.attr("transform", event.transform);
        });

      d3.select(container)
        .select("svg")
        .call(zoom as any);
    } catch (err) {
      console.error("Error creating tree:", err);
      svg
        .append("text")
        .attr("x", width / 2)
        .attr("y", height / 2)
        .attr("text-anchor", "middle")
        .attr("fill", "red")
        .text("Error creating visualization");
    }
  };

  // Render force-directed graph visualization
  const renderForceGraph = (data: JsonOutput) => {
    const container = d3Container.current;
    if (!container) return;

    // Clear previous visualization
    d3.select(container).selectAll("*").remove();

    // Set up dimensions
    const width = 800;
    const height = 600;

    // Create SVG container
    const svg = d3
      .select(container)
      .append("svg")
      .attr("width", width)
      .attr("height", height)
      .attr("viewBox", [0, 0, width, height]);

    // Add title
    svg
      .append("text")
      .attr("x", width / 2)
      .attr("y", 20)
      .attr("text-anchor", "middle")
      .style("font-size", "16px")
      .style("font-weight", "bold")
      .text("JSON Graph Visualization");

    // Create color scale based on node type
    const getNodeColor = (label: string) => {
      switch (label) {
        case "Object":
          return "#4CAF50";
        case "Array":
          return "#2196F3";
        case "Key":
          return "#E91E63";
        case "Index":
          return "#3F51B5";
        case "String":
          return "#FFC107";
        case "Number":
          return "#9C27B0";
        case "Boolean":
          return "#FF5722";
        case "Null":
          return "#607D8B";
        default:
          return "#795548";
      }
    };

    // Create node size scale based on is_leaf property
    const getNodeSize = (node: JsonNode) => {
      return node.is_leaf ? 6 : 10;
    };

    try {
      // Convert string IDs to object references for d3 force simulation
      const nodeMap = new Map(
        data.nodes.map((node) => [
          node.id,
          { ...node, x: undefined, y: undefined },
        ])
      );

      // Create links with resolved references
      const links = data.links.map((link) => ({
        source: nodeMap.get(link.source)!,
        target: nodeMap.get(link.target)!,
      }));

      // Create simulation
      const simulation = d3
        .forceSimulation(Array.from(nodeMap.values()))
        .force(
          "link",
          d3
            .forceLink<JsonNode, { source: JsonNode; target: JsonNode }>(links)
            .id((d) => d.id)
            .distance(70)
            .strength(0.7)
        )
        .force("charge", d3.forceManyBody().strength(-200))
        .force("center", d3.forceCenter(width / 2, height / 2))
        .force(
          "collision",
          d3.forceCollide<JsonNode>().radius((d) => getNodeSize(d) * 1.5)
        );

      // Create container group for links
      const link = svg
        .append("g")
        .attr("stroke", "#999")
        .attr("stroke-opacity", 0.6)
        .selectAll("line")
        .data(links)
        .join("line")
        .attr("stroke-width", 1);

      // Create container for nodes
      const node = svg
        .append("g")
        .attr("stroke", "#fff")
        .attr("stroke-width", 1.5)
        .selectAll("circle")
        .data(Array.from(nodeMap.values()))
        .join("circle")
        .attr("r", (d) => getNodeSize(d))
        .attr("fill", (d) => getNodeColor(d.label))
        .call(drag(simulation) as any);

      // Add tooltips to nodes
      node.append("title").text((d) => `${d.label}: ${d.value}`);

      // Add hover effect
      node
        .on("mouseover", function () {
          d3.select(this).attr("stroke", "#000").attr("stroke-width", 2);
        })
        .on("mouseout", function () {
          d3.select(this).attr("stroke", "#fff").attr("stroke-width", 1.5);
        });

      // Add labels only to non-leaf nodes
      const labels = svg
        .append("g")
        .attr("font-family", "sans-serif")
        .attr("font-size", 10)
        .selectAll("text")
        .data(Array.from(nodeMap.values()).filter((d) => !d.is_leaf))
        .join("text")
        .text((d) => d.label)
        .attr("stroke", "#fff")
        .attr("stroke-width", 0.5)
        .attr("paint-order", "stroke");

      // Update function for simulation
      simulation.on("tick", () => {
        link
          .attr("x1", (d) => (d.source as any).x)
          .attr("y1", (d) => (d.source as any).y)
          .attr("x2", (d) => (d.target as any).x)
          .attr("y2", (d) => (d.target as any).y);

        node.attr("cx", (d) => d.x!).attr("cy", (d) => d.y!);

        labels
          .attr("x", (d) => d.x!)
          .attr("y", (d) => d.y! - getNodeSize(d) - 5);
      });

      // Add zoom capability
      const zoom = d3
        .zoom()
        .scaleExtent([0.5, 3])
        .on("zoom", (event) => {
          svg.selectAll("g").attr("transform", event.transform);
        });

      svg.call(zoom as any);

      // Add legend
      const legend = svg.append("g").attr("transform", "translate(20, 40)");

      const nodeTypes = [
        "Object",
        "Array",
        "String",
        "Number",
        "Boolean",
        "Null",
        "Key",
        "Index",
      ];

      nodeTypes.forEach((type, i) => {
        legend
          .append("circle")
          .attr("cx", 10)
          .attr("cy", i * 20)
          .attr("r", 6)
          .attr("fill", getNodeColor(type));

        legend
          .append("text")
          .attr("x", 20)
          .attr("y", i * 20 + 4)
          .text(type)
          .style("font-size", "12px")
          .attr("alignment-baseline", "middle");
      });

      // Drag handler for nodes
      function drag(simulation: d3.Simulation<JsonNode, undefined>) {
        function dragstarted(event: d3.D3DragEvent<any, JsonNode, any>) {
          if (!event.active) simulation.alphaTarget(0.3).restart();
          event.subject.fx = event.subject.x;
          event.subject.fy = event.subject.y;
        }

        function dragged(event: d3.D3DragEvent<any, JsonNode, any>) {
          event.subject.fx = event.x;
          event.subject.fy = event.y;
        }

        function dragended(event: d3.D3DragEvent<any, JsonNode, any>) {
          if (!event.active) simulation.alphaTarget(0);
          event.subject.fx = null;
          event.subject.fy = null;
        }

        return d3
          .drag<any, JsonNode>()
          .on("start", dragstarted)
          .on("drag", dragged)
          .on("end", dragended);
      }
    } catch (err) {
      console.error("Error creating force graph:", err);
      svg
        .append("text")
        .attr("x", width / 2)
        .attr("y", height / 2)
        .attr("text-anchor", "middle")
        .attr("fill", "red")
        .text("Error creating graph visualization");
    }
  };

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
  const renderGraph = (data: JsonOutput) => {
    const container = d3Container.current;
    if (!container) return;

    // Clear previous SVG
    d3.select(container).selectAll("*").remove();

    // Convert string IDs to object references
    const nodes: JsonNode[] = data.nodes.map((n) => ({
      ...n,
      x: undefined,
      y: undefined,
    }));

    const links = data.links.map((l) => ({
      source: nodes.find((n) => n.id === l.source)!,
      target: nodes.find((n) => n.id === l.target)!,
    }));

    const simulation = d3
      .forceSimulation<JsonNode>(nodes)
      .force("charge", d3.forceManyBody().strength(-1000))
      .force(
        "link",
        d3
          .forceLink<JsonNode, d3.SimulationLinkDatum<JsonNode>>(links)
          .id((d: JsonNode) => d.id)
      )
      .force("center", d3.forceCenter(400, 300));

    const svg = d3
      .select(container)
      .append("svg")
      .attr("width", 800)
      .attr("height", 600);

    // Draw links
    const link = svg
      .append("g")
      .attr("stroke", "#999")
      .selectAll("line")
      .data(links)
      .enter()
      .append("line")
      .attr("stroke-width", 1);

    // Draw nodes
    const node = svg
      .append("g")
      .selectAll("circle")
      .data(nodes)
      .enter()
      .append("circle")
      .attr("r", 10)
      .attr("fill", "#2196F3");

    simulation.on("tick", () => {
      link
        .attr("x1", (d) => d.source.x ?? 0)
        .attr("y1", (d) => d.source.y ?? 0)
        .attr("x2", (d) => d.target.x ?? 0)
        .attr("y2", (d) => d.target.y ?? 0);

      node.attr("cx", (d) => d.x ?? 0).attr("cy", (d) => d.y ?? 0);
    });
  };
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

      <div className="grid grid-cols-2 gap-4 h-[600px]">
        <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900">
          <div className="font-mono text-sm h-full">
            {/* Editor will be mounted here */}
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

        <div className="border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 p-4">
          <div className="font-mono text-sm h-full text-gray-900 dark:text-gray-100">
            {/* Visualization will be shown here */}
            <div className="h-full flex items-center justify-center text-gray-500 dark:text-gray-400">
              {content ? (
                <div ref={d3Container} className="w-full h-full"></div>
              ) : (
                "Visualization will appear here"
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Editor;
