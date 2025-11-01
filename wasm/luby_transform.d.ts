/* tslint:disable */
/* eslint-disable */
export function encode_file_blocks(blocks: Array<any>, seed: bigint | null | undefined, num_encoded_blocks: number): Array<any>;
export function init(): void;
export class EncodedBlock {
  free(): void;
  [Symbol.dispose](): void;
  constructor(seed: bigint, degree: number, data: Int32Array);
  readonly seed: bigint;
  readonly degree: number;
  readonly indices: Uint32Array;
  readonly data: Int32Array;
}
export class LubyTransformDecoder {
  free(): void;
  [Symbol.dispose](): void;
  constructor(k: number, block_size: number);
  add_encoded_block(seed: bigint, degree: number, data: Int32Array): number;
  decoded_count(): number;
  is_complete(): boolean;
  get_all_decoded_blocks(): Array<any> | undefined;
  current_round(): number;
}
export class LubyTransformEncoder {
  free(): void;
  [Symbol.dispose](): void;
  constructor(source_blocks: Array<any>, seed?: bigint | null);
  generate_block(seed?: bigint | null): EncodedBlock;
  source_block_count(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_lubytransformencoder_free: (a: number, b: number) => void;
  readonly __wbg_lubytransformdecoder_free: (a: number, b: number) => void;
  readonly __wbg_encodedblock_free: (a: number, b: number) => void;
  readonly encodedblock_new: (a: bigint, b: number, c: number, d: number) => number;
  readonly encodedblock_seed: (a: number) => bigint;
  readonly encodedblock_degree: (a: number) => number;
  readonly encodedblock_indices: (a: number) => [number, number];
  readonly encodedblock_data: (a: number) => any;
  readonly lubytransformencoder_new: (a: any, b: number, c: bigint) => number;
  readonly lubytransformencoder_generate_block: (a: number, b: number, c: bigint) => number;
  readonly lubytransformencoder_source_block_count: (a: number) => number;
  readonly lubytransformdecoder_new: (a: number, b: number) => number;
  readonly lubytransformdecoder_add_encoded_block: (a: number, b: bigint, c: number, d: number, e: number) => number;
  readonly lubytransformdecoder_decoded_count: (a: number) => number;
  readonly lubytransformdecoder_is_complete: (a: number) => number;
  readonly lubytransformdecoder_get_all_decoded_blocks: (a: number) => any;
  readonly lubytransformdecoder_current_round: (a: number) => number;
  readonly encode_file_blocks: (a: any, b: number, c: bigint, d: number) => any;
  readonly init: () => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
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
