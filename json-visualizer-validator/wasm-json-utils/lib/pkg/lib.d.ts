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
export function process_json_tree(json_str: string): string;
export function process_json(json_str: string): any;

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
  readonly process_json_tree: (a: number, b: number) => [number, number, number, number];
  readonly process_json: (a: number, b: number) => [number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
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
