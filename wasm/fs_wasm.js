
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}
/**
* @param {string} file_path
*/
export function open_test(file_path) {
    var ptr0 = passStringToWasm0(file_path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.open_test(ptr0, len0);
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

let cachegetUint16Memory0 = null;
function getUint16Memory0() {
    if (cachegetUint16Memory0 === null || cachegetUint16Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint16Memory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachegetUint16Memory0;
}

function passArray16ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 2);
    getUint16Memory0().set(arg, ptr / 2);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
*/
export class Archive {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_archive_free(ptr);
    }
}
/**
*/
export class CentralDirectoryEntry {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_centraldirectoryentry_free(ptr);
    }
}
/**
*/
export class DataDescriptor {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_datadescriptor_free(ptr);
    }
}
/**
*/
export class EndOfCentralDirectory {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_endofcentraldirectory_free(ptr);
    }
}
/**
*/
export class File {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_file_free(ptr);
    }
}
/**
*/
export class FileEntry {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_fileentry_free(ptr);
    }
}
/**
*/
export class LocalHeader {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_localheader_free(ptr);
    }
}
/**
*/
export class Zip {

    static __wrap(ptr) {
        const obj = Object.create(Zip.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_zip_free(ptr);
    }
    /**
    * @param {number} seed
    */
    constructor(seed) {
        var ret = wasm.zip_new(seed);
        return Zip.__wrap(ret);
    }
    /**
    * @param {string} file_path
    */
    open_test(file_path) {
        var ptr = this.ptr;
        this.ptr = 0;
        var ptr0 = passStringToWasm0(file_path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.zip_open_test(ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get_uuid() {
        try {
            wasm.zip_get_uuid(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    get_passwd() {
        try {
            wasm.zip_get_passwd(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {number}
    */
    get_compress_level() {
        var ret = wasm.zip_get_compress_level(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {string} passwd
    */
    set_passwd(passwd) {
        var ptr0 = passStringToWasm0(passwd, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.zip_set_passwd(this.ptr, ptr0, len0);
    }
    /**
    * @param {string} file_name
    * @param {BigInt} file_size
    * @param {Uint16Array} last_modified
    * @param {Uint8Array} file_raw
    */
    add_file(file_name, file_size, last_modified, file_raw) {
        var ptr0 = passStringToWasm0(file_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        uint64CvtShim[0] = file_size;
        const low1 = u32CvtShim[0];
        const high1 = u32CvtShim[1];
        var ptr2 = passArray16ToWasm0(last_modified, wasm.__wbindgen_malloc);
        var len2 = WASM_VECTOR_LEN;
        var ptr3 = passArray8ToWasm0(file_raw, wasm.__wbindgen_malloc);
        var len3 = WASM_VECTOR_LEN;
        wasm.zip_add_file(this.ptr, ptr0, len0, low1, high1, ptr2, len2, ptr3, len3);
    }
    /**
    * @returns {Uint8Array}
    */
    save() {
        wasm.zip_save(8, this.ptr);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getInt32Memory0()[8 / 4 + 1];
        var v0 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v0;
    }
    /**
    * @param {Archive} archive
    * @returns {Uint8Array}
    */
    check_archive(archive) {
        _assertClass(archive, Archive);
        var ptr0 = archive.ptr;
        archive.ptr = 0;
        wasm.zip_check_archive(8, this.ptr, ptr0);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getInt32Memory0()[8 / 4 + 1];
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v1;
    }
}
/**
*/
export class Zip64EndOfCentralDirectoryLocator {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_zip64endofcentraldirectorylocator_free(ptr);
    }
}
/**
*/
export class Zip64EndOfCentralDirectoryRecord {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_zip64endofcentraldirectoryrecord_free(ptr);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_log_3ac8e26236dda1cb = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

