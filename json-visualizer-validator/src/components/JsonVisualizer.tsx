import React, { useEffect, useRef } from "react";
import * as d3 from "d3";

interface Node {
  id: string;
  label: string;
  value: string;
  depth: number;
  parent: string | null;
  is_leaf: boolean;
  x?: number;
  y?: number;
}

interface Link {
  source: string;
  target: string;
}

// interface GraphData {
//   nodes: Node[];
//   links: Link[];
// }

interface JSONVisualizerProps {
  data: any;
  isDarkMode: boolean;
}

const JSONVisualizer: React.FC<JSONVisualizerProps> = ({
  data,
  isDarkMode,
}) => {
  const d3Container = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (data && d3Container.current) {
      renderGraph();
    }

    return () => {
      if (d3Container.current) {
        d3.select(d3Container.current).selectAll("*").remove();
      }
    };
  }, [data]);

  const renderGraph = () => {
    const container = d3Container.current;
    if (!container) return;

    // Clear previous visualization
    d3.select(container).selectAll("*").remove();

    // Get container dimensions
    const width = container.clientWidth || 900;
    const height = container.clientHeight || 600;

    // Create SVG
    const svg = d3
      .select(container)
      .append("svg")
      .attr("width", width)
      .attr("height", height)
      .style("background-color", isDarkMode ? "#1e1e1e" : "white");

    // Add zoom behavior
    const zoom = d3
      .zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on("zoom", (event) => {
        graphGroup.attr("transform", event.transform);
      });

    svg.call(zoom);

    // Add a group for the entire graph that will be draggable and zoomable
    const graphGroup = svg.append("g");

    // Clone nodes to avoid modifying original data
    const nodes = JSON.parse(JSON.stringify(data.nodes)) as Node[];

    const linkGroup = graphGroup.append("g").attr("class", "links");
    const nodeGroup = graphGroup.append("g").attr("class", "nodes");

    // [Rest of your existing code remains exactly the same from here...]
    const nodesByDepth: Record<number, Node[]> = {};
    nodes.forEach((node) => {
      if (!nodesByDepth[node.depth]) {
        nodesByDepth[node.depth] = [];
      }
      nodesByDepth[node.depth].push(node);
    });

    // Layout configuration
    const nodeWidth = 160;
    const nodeHeight = 40;
    const horizontalSpacing = 80;
    const verticalSpacing = 30;

    // Calculate x positions for each depth level
    const xPositions: Record<number, number> = {};
    let currentX = 50;

    for (
      let i = 0;
      i <= Math.max(...Object.keys(nodesByDepth).map(Number));
      i++
    ) {
      xPositions[i] = currentX;
      currentX += nodeWidth + horizontalSpacing;
    }

    // Position nodes
    Object.entries(nodesByDepth).forEach(([depthStr, nodesAtDepth]) => {
      const depth = parseInt(depthStr);
      const totalDepthHeight =
        nodesAtDepth.length * nodeHeight +
        (nodesAtDepth.length - 1) * verticalSpacing;
      const startY = (height - totalDepthHeight) / 2;

      nodesAtDepth.forEach((node, i) => {
        node.x = xPositions[depth];
        node.y = startY + i * (nodeHeight + verticalSpacing);
      });
    });

    // Create links
    data.links.forEach((link: Link) => {
      const source: Node | undefined = nodes.find(
        (n: Node) => n.id === link.source
      );
      const target: Node | undefined = nodes.find(
        (n: Node) => n.id === link.target
      );

      if (
        source &&
        source.x !== undefined &&
        source.y !== undefined &&
        target &&
        target.x !== undefined &&
        target.y !== undefined
      ) {
        const sourceX: number = source.x + nodeWidth;
        const sourceY: number = source.y + nodeHeight / 2;
        const targetX: number = target.x;
        const targetY: number = target.y + nodeHeight / 2;

        const path: d3.Path = d3.path();
        path.moveTo(sourceX, sourceY);

        const controlX: number = (sourceX + targetX) / 2;

        path.bezierCurveTo(
          controlX,
          sourceY,
          controlX,
          targetY,
          targetX,
          targetY
        );

        linkGroup
          .append("path")
          .attr("d", path.toString())
          .attr("stroke", "#555")
          .attr("stroke-width", 1)
          .attr("fill", "none");
      }
    });

    nodes.forEach((node) => {
      if (node.x === undefined || node.y === undefined) return;

      const nodeG = nodeGroup
        .append("g")
        .attr("transform", `translate(${node.x}, ${node.y})`);

      nodeG
        .append("rect")
        .attr("width", nodeWidth)
        .attr("height", nodeHeight)
        .attr("rx", 5)
        .attr("ry", 5)
        .attr("fill", getNodeColor(node))
        .attr("stroke", "#444")
        .attr("stroke-width", 1);

      nodeG
        .append("text")
        .attr("x", 10)
        .attr("y", nodeHeight / 2)
        .attr("dominant-baseline", "middle")
        .attr("fill", "white")
        .text(getNodeText(node));
    });

    function getNodeColor(node: Node): string {
      if (node.depth === 0) return "#2c3e50";
      if (node.label === "name") return "#27ae60";
      if (node.label === "color") return "#8e44ad";
      if (node.label === "nutrients") return "#2980b9";
      if (node.label === "calories") return "#e74c3c";
      if (node.label === "fiber") return "#f39c12";
      if (node.label === "vitaminC" || node.label === "potassium")
        return "#16a085";
      return "#34495e";
    }

    function getNodeText(node: Node): string {
      if (node.depth === 0) {
        return `${node.label} [${node.value}]`;
      } else if (node.label.includes("name") || node.label.includes("color")) {
        return `${node.label}: "${node.value}"`;
      } else {
        return `${node.label}: ${node.value}`;
      }
    }
  };

  return <div ref={d3Container} className="w-full h-full min-h-[600px]" />;
};

export default JSONVisualizer;
