let wasm;

let cachedInt32ArrayMemory0 = null;

function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
}

function getArrayI32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let WASM_VECTOR_LEN = 0;

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
 * @param {Array<any>} blocks
 * @param {bigint | null | undefined} seed
 * @param {number} num_encoded_blocks
 * @returns {Array<any>}
 */
export function encode_file_blocks(blocks, seed, num_encoded_blocks) {
    const ret = wasm.encode_file_blocks(blocks, !isLikeNone(seed), isLikeNone(seed) ? BigInt(0) : seed, num_encoded_blocks);
    return ret;
}

export function init() {
    wasm.init();
}

const EncodedBlockFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_encodedblock_free(ptr >>> 0, 1));

export class EncodedBlock {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(EncodedBlock.prototype);
        obj.__wbg_ptr = ptr;
        EncodedBlockFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        EncodedBlockFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_encodedblock_free(ptr, 0);
    }
    /**
     * @param {bigint} seed
     * @param {number} degree
     * @param {Int32Array} data
     */
    constructor(seed, degree, data) {
        const ptr0 = passArray32ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.encodedblock_new(seed, degree, ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        EncodedBlockFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {bigint}
     */
    get seed() {
        const ret = wasm.encodedblock_seed(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get degree() {
        const ret = wasm.encodedblock_degree(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {Uint32Array}
     */
    get indices() {
        const ret = wasm.encodedblock_indices(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Int32Array}
     */
    get data() {
        const ret = wasm.encodedblock_data(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) EncodedBlock.prototype[Symbol.dispose] = EncodedBlock.prototype.free;

const LubyTransformDecoderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lubytransformdecoder_free(ptr >>> 0, 1));

export class LubyTransformDecoder {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LubyTransformDecoderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lubytransformdecoder_free(ptr, 0);
    }
    /**
     * @param {number} k
     * @param {number} block_size
     */
    constructor(k, block_size) {
        const ret = wasm.lubytransformdecoder_new(k, block_size);
        this.__wbg_ptr = ret >>> 0;
        LubyTransformDecoderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {bigint} seed
     * @param {number} degree
     * @param {Int32Array} data
     * @returns {number}
     */
    add_encoded_block(seed, degree, data) {
        const ptr0 = passArray32ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.lubytransformdecoder_add_encoded_block(this.__wbg_ptr, seed, degree, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    decoded_count() {
        const ret = wasm.lubytransformdecoder_decoded_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_complete() {
        const ret = wasm.lubytransformdecoder_is_complete(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {Array<any> | undefined}
     */
    get_all_decoded_blocks() {
        const ret = wasm.lubytransformdecoder_get_all_decoded_blocks(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    current_round() {
        const ret = wasm.lubytransformdecoder_current_round(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) LubyTransformDecoder.prototype[Symbol.dispose] = LubyTransformDecoder.prototype.free;

const LubyTransformEncoderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lubytransformencoder_free(ptr >>> 0, 1));

export class LubyTransformEncoder {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LubyTransformEncoderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lubytransformencoder_free(ptr, 0);
    }
    /**
     * @param {Array<any>} source_blocks
     * @param {bigint | null} [seed]
     */
    constructor(source_blocks, seed) {
        const ret = wasm.lubytransformencoder_new(source_blocks, !isLikeNone(seed), isLikeNone(seed) ? BigInt(0) : seed);
        this.__wbg_ptr = ret >>> 0;
        LubyTransformEncoderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {bigint | null} [seed]
     * @returns {EncodedBlock}
     */
    generate_block(seed) {
        const ret = wasm.lubytransformencoder_generate_block(this.__wbg_ptr, !isLikeNone(seed), isLikeNone(seed) ? BigInt(0) : seed);
        return EncodedBlock.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    source_block_count() {
        const ret = wasm.lubytransformencoder_source_block_count(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) LubyTransformEncoder.prototype[Symbol.dispose] = LubyTransformEncoder.prototype.free;

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_encodedblock_new = function(arg0) {
        const ret = EncodedBlock.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_get_0da715ceaecea5c8 = function(arg0, arg1) {
        const ret = arg0[arg1 >>> 0];
        return ret;
    };
    imports.wbg.__wbg_length_186546c51cd61acd = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_length_3fad871d8eb32a05 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_new_1f3a344cf3123716 = function() {
        const ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_newfromslice_32279e2fa6414ad4 = function(arg0, arg1) {
        const ret = new Int32Array(getArrayI32FromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_prototypesetcall_80abb47eb4faef59 = function(arg0, arg1, arg2) {
        Int32Array.prototype.set.call(getArrayI32FromWasm0(arg0, arg1), arg2);
    };
    imports.wbg.__wbg_push_330b2eb93e4e1212 = function(arg0, arg1) {
        const ret = arg0.push(arg1);
        return ret;
    };
    imports.wbg.__wbg_wbindgenthrow_451ec1a8469d7eb6 = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('luby_transform_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
