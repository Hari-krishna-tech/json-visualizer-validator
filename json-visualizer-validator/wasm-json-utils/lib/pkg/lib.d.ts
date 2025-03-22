/* tslint:disable */
/* eslint-disable */
export function json_to_typescript(json_str: string): string;
export function json_to_java(json_str: string): string;
export function json_to_golang(json_str: string): string;
export function json_to_python(json_str: string): string;
export function yaml_to_typescript(yaml_str: string): string;
export function yaml_to_java(yaml_str: string): string;
export function yaml_to_python(yaml_str: string): string;
export function yaml_to_golang(yaml_str: string): string;
export function xml_to_typescript(xml: string): string;
export function xml_to_java(xml: string): string;
export function xml_to_golang(xml: string): string;
export function xml_to_python(xml: string): string;
export function csv_to_typescript(csv: string): string;
export function csv_to_java(csv: string): string;
export function csv_to_golang(csv: string): string;
export function csv_to_python(csv: string): string;
export function json_to_json_schema(json_str: string): string;
export function json_to_yaml(json_str: string): string;
export function json_to_xml(json_str: string): string;
export function json_to_csv(json_str: string): string;
export function yaml_to_json(yaml_str: string): string;
export function yaml_to_xml(yaml_str: string): string;
export function yaml_to_csv(yaml_str: string): string;
export function xml_to_json(xml_str: string): string;
export function xml_to_yaml(xml_str: string): string;
export function xml_to_csv(xml_str: string): string;
export function csv_to_json(csv_str: string): string;
export function csv_to_yaml(csv_str: string): string;
export function csv_to_xml(csv_str: string): string;
/**
 * Exposed WebAssembly function for processing YAML into a graph format.
 *
 * It parses the YAML string and then recursively builds a list of nodes and
 * links. The returned JSON follows the format:
 *
 * {
 *    "nodes": [ { "id": "...", "label": "...", "value": "...", "depth": ..., "parent": "..."?, "is_leaf": ... }, ... ],
 *    "links": [ { "source": "...", "target": "..." }, ... ]
 * }
 */
export function process_yaml_graph(yaml_str: string): string;
/**
 * Exposed WebAssembly function for processing YAML into a tree format.
 *
 * It parses the YAML string and converts it into a hierarchical tree where
 * each node has a "name" and, if a leaf, a "value". Composite values get a
 * "children" array. The returned JSON for an object looks like:
 *
 * {
 *    "name": "root",
 *    "children": [
 *       {"name": "category", "value": "\"Home Goods\""},
 *       ...,
 *       {"name": "specifications", "children": [ ... ] }
 *    ]
 * }
 */
export function process_yaml_tree(yaml_str: string): string;
/**
 * Exports a function to process XML into the graph format.
 * Returns a JSON string representing the graph or an error if the XML is invalid.
 */
export function process_xml_graph(xml: string): string;
/**
 * Exports a function to process XML into the tree format.
 * Returns a JSON string representing the tree or an error if the XML is invalid.
 */
export function process_xml_tree(xml: string): string;
/**
 * Process the CSV string and return a JSON string for graph visualization.
 *
 * This function:
 * 1. Parses the CSV (using the first record).
 * 2. Converts the record into a JSON object (attempting a JSON parse on each field).
 * 3. Recursively builds the graph representation.
 */
export function process_csv_graph(csv: string): string;
/**
 * Process the CSV string and return a JSON string for tree visualization.
 *
 * This function behaves like `process_csv_graph` except that
 * it builds a nested tree structure instead.
 */
export function process_csv_tree(csv: string): string;
export function process_json_tree(json_str: string): string;
export function process_json(json_str: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly json_to_typescript: (a: number, b: number) => [number, number, number, number];
  readonly json_to_java: (a: number, b: number) => [number, number, number, number];
  readonly json_to_golang: (a: number, b: number) => [number, number, number, number];
  readonly json_to_python: (a: number, b: number) => [number, number, number, number];
  readonly yaml_to_typescript: (a: number, b: number) => [number, number, number, number];
  readonly yaml_to_java: (a: number, b: number) => [number, number, number, number];
  readonly yaml_to_python: (a: number, b: number) => [number, number, number, number];
  readonly yaml_to_golang: (a: number, b: number) => [number, number, number, number];
  readonly xml_to_typescript: (a: number, b: number) => [number, number, number, number];
  readonly xml_to_java: (a: number, b: number) => [number, number, number, number];
  readonly xml_to_golang: (a: number, b: number) => [number, number, number, number];
  readonly xml_to_python: (a: number, b: number) => [number, number, number, number];
  readonly csv_to_typescript: (a: number, b: number) => [number, number, number, number];
  readonly csv_to_java: (a: number, b: number) => [number, number, number, number];
  readonly csv_to_golang: (a: number, b: number) => [number, number, number, number];
  readonly csv_to_python: (a: number, b: number) => [number, number, number, number];
  readonly json_to_json_schema: (a: number, b: number) => [number, number, number, number];
  readonly json_to_yaml: (a: number, b: number) => [number, number, number, number];
  readonly json_to_xml: (a: number, b: number) => [number, number, number, number];
  readonly json_to_csv: (a: number, b: number) => [number, number, number, number];
  readonly yaml_to_json: (a: number, b: number) => [number, number, number, number];
  readonly yaml_to_xml: (a: number, b: number) => [number, number, number, number];
  readonly yaml_to_csv: (a: number, b: number) => [number, number, number, number];
  readonly xml_to_json: (a: number, b: number) => [number, number, number, number];
  readonly xml_to_yaml: (a: number, b: number) => [number, number, number, number];
  readonly xml_to_csv: (a: number, b: number) => [number, number, number, number];
  readonly csv_to_json: (a: number, b: number) => [number, number, number, number];
  readonly csv_to_yaml: (a: number, b: number) => [number, number, number, number];
  readonly csv_to_xml: (a: number, b: number) => [number, number, number, number];
  readonly process_yaml_graph: (a: number, b: number) => [number, number, number, number];
  readonly process_yaml_tree: (a: number, b: number) => [number, number, number, number];
  readonly process_xml_graph: (a: number, b: number) => [number, number, number, number];
  readonly process_xml_tree: (a: number, b: number) => [number, number, number, number];
  readonly process_csv_graph: (a: number, b: number) => [number, number, number, number];
  readonly process_csv_tree: (a: number, b: number) => [number, number, number, number];
  readonly process_json_tree: (a: number, b: number) => [number, number, number, number];
  readonly process_json: (a: number, b: number) => [number, number, number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
