/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const __wbg_lubytransformencoder_free: (a: number, b: number) => void;
export const __wbg_lubytransformdecoder_free: (a: number, b: number) => void;
export const __wbg_encodedblock_free: (a: number, b: number) => void;
export const encodedblock_new: (a: bigint, b: number, c: number, d: number) => number;
export const encodedblock_seed: (a: number) => bigint;
export const encodedblock_degree: (a: number) => number;
export const encodedblock_indices: (a: number) => [number, number];
export const encodedblock_data: (a: number) => [number, number];
export const lubytransformencoder_new: (a: number, b: number, c: number, d: bigint) => number;
export const lubytransformencoder_generate_block: (a: number, b: number, c: bigint) => number;
export const lubytransformencoder_source_block_count: (a: number) => number;
export const lubytransformdecoder_new: (a: number, b: number) => number;
export const lubytransformdecoder_add_encoded_block: (a: number, b: bigint, c: number, d: number, e: number) => number;
export const lubytransformdecoder_decoded_count: (a: number) => number;
export const lubytransformdecoder_is_complete: (a: number) => number;
export const lubytransformdecoder_get_all_decoded_blocks: (a: number) => any;
export const lubytransformdecoder_current_round: (a: number) => number;
export const encode_file_blocks: (a: number, b: number, c: number, d: bigint, e: number) => any;
export const init: () => void;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_export_2: WebAssembly.Table;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __externref_table_alloc: () => number;
export const __wbindgen_start: () => void;
