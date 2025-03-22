// JsonTreeVisualizer.tsx
import React, { useEffect, useRef } from "react";
import * as d3 from "d3";

// Define types for our tree data
interface TreeNode {
  name: string;
  children?: TreeNode[];
  value?: string;
}

interface TreeVisualizerProps {
  data: any;
  isDarkMode: boolean;
}

const isTreeFormat = (data: any): boolean => {
  return data?.name && data?.children;
};

const TreeVisualizer: React.FC<TreeVisualizerProps> = ({
  data,
  isDarkMode,
}) => {
  try {
    if (!isTreeFormat(JSON.parse(data))) {
      return <div className="text-red-500">Invalid data format</div>;
    }
  } catch (e) {
    return <div className="text-red-500">Invalid data format</div>;
  }
  const svgRef = useRef<SVGSVGElement>(null);
  const width = 900;
  const height = 700;

  const nodeFillColor = isDarkMode ? "#ddd" : "#555";
  const linkColor = isDarkMode ? "#666" : "#ccc";
  const nodeRadius = 5;
  const animated = true;

  const treeData = JSON.parse(data) as TreeNode;
  useEffect(() => {
    if (!treeData || !svgRef.current) return;

    // Clear previous visualization
    d3.select(svgRef.current).selectAll("*").remove();

    // Set up margins and dimensions
    const margin = { top: 20, right: 120, bottom: 20, left: 120 };
    const innerWidth = width - margin.left - margin.right;
    const innerHeight = height - margin.top - margin.bottom;

    // Create SVG container with zoom support
    const svg = d3
      .select(svgRef.current)
      .attr("width", width)
      .attr("height", height);

    // Add zoom behavior
    const zoom = d3
      .zoom()
      .scaleExtent([0.1, 3])
      .on("zoom", (event) => {
        g.attr("transform", event.transform);
      });

    svg.call(zoom as any);

    // Create a group for the tree
    const g = svg
      .append("g")
      .attr("transform", `translate(${margin.left},${margin.top})`);

    // Create hierarchical data structure
    const root = d3.hierarchy(treeData);

    // Create tree layout
    const treeLayout = d3.tree<TreeNode>().size([innerHeight, innerWidth]);

    // Compute node positions
    const treeData2 = treeLayout(root);

    // Create links
    const links = g
      .selectAll(".link")
      .data(treeData2.links())
      .enter()
      .append("path")
      .attr("class", "link")
      .attr("fill", "none")
      .attr("stroke", linkColor)
      .attr("stroke-width", 1.5);

    if (animated) {
      links
        .attr(
          "d",
          d3
            .linkHorizontal<any, any>()
            .x(() => 0)
            .y((d) => d.x)
        )
        .transition()
        .duration(750)
        .attr(
          "d",
          d3
            .linkHorizontal<any, any>()
            .x((d) => d.y)
            .y((d) => d.x)
        );
    } else {
      links.attr(
        "d",
        d3
          .linkHorizontal<any, any>()
          .x((d) => d.y)
          .y((d) => d.x)
      );
    }

    // Create node groups
    const nodes = g
      .selectAll(".node")
      .data(treeData2.descendants())
      .enter()
      .append("g")
      .attr(
        "class",
        (d) => `node ${d.children ? "node--internal" : "node--leaf"}`
      )
      .attr("transform", (d) => `translate(${animated ? 0 : d.y},${d.x})`);

    if (animated) {
      nodes
        .transition()
        .duration(750)
        .attr("transform", (d) => `translate(${d.y},${d.x})`);
    }

    // Add circles for nodes
    nodes
      .append("circle")
      .attr("r", nodeRadius)
      .attr("fill", (d) =>
        d.children ? nodeFillColor : isDarkMode ? "#fff" : "#999"
      )
      .attr("stroke", isDarkMode ? "#555" : "#fff")
      .attr("stroke-width", 1.5);

    // Add node labels
    nodes
      .append("text")
      .attr("dy", ".31em")
      .attr("x", (d) => (d.children ? -8 : 8))
      .attr("text-anchor", (d) => (d.children ? "end" : "start"))
      .text((d) => d.data.name)
      .attr("font-size", "12px")
      .attr("font-family", "Arial, sans-serif")
      .attr("fill", isDarkMode ? "#fff" : "#000")
      .style("fill-opacity", animated ? 0 : 1);

    if (animated) {
      nodes
        .selectAll("text")
        .transition()
        .duration(750)
        .style("fill-opacity", 1);
    }

    // Add value labels for leaf nodes
    nodes
      .filter((d) => !d.children && !!d.data.value)
      .append("text")
      .attr("dy", "1.3em")
      .attr("x", 8)
      .attr("text-anchor", "start")
      .text((d) => {
        let value = d.data.value;
        if (typeof value === "string") {
          if (value.startsWith('"') && value.endsWith('"')) {
            value = value.substring(1, value.length - 1);
          }
        }
        return value || "";
      })
      .attr("font-size", "10px")
      .attr("font-family", "Arial, sans-serif")
      .attr("fill", isDarkMode ? "#fff" : "#555")
      .style("fill-opacity", animated ? 0 : 1);

    if (animated) {
      nodes
        .selectAll("text:nth-of-type(2)")
        .transition()
        .duration(750)
        .delay(300)
        .style("fill-opacity", 1);
    }

    // Add hover effects
    nodes
      .on("mouseover", function () {
        d3.select(this)
          .select("circle")
          .attr("r", nodeRadius * 1.5)
          .attr("fill", isDarkMode ? "#8ab4f8" : "#66a3ff");
      })
      .on("mouseout", function () {
        d3.select(this)
          .select("circle")
          .attr("r", nodeRadius)
          .attr("fill", () =>
            d3.select(this).classed("node--internal")
              ? nodeFillColor
              : isDarkMode
              ? "#fff"
              : "#999"
          );
      });
  }, [
    treeData,
    width,
    height,
    nodeFillColor,
    linkColor,
    nodeRadius,
    animated,
    isDarkMode,
  ]);

  return (
    <div className="json-tree-visualizer">
      <svg ref={svgRef}></svg>
    </div>
  );
};

export default TreeVisualizer;
