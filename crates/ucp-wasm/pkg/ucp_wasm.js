
let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

let WASM_VECTOR_LEN = 0;

const ContentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_content_free(ptr >>> 0, 1));

const DocumentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_document_free(ptr >>> 0, 1));

const IdMapperFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_idmapper_free(ptr >>> 0, 1));

const PromptBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_promptbuilder_free(ptr >>> 0, 1));

const PromptPresetsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_promptpresets_free(ptr >>> 0, 1));

const SnapshotManagerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_snapshotmanager_free(ptr >>> 0, 1));

const WasmAuditEntryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmauditentry_free(ptr >>> 0, 1));

const WasmClearResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmclearresult_free(ptr >>> 0, 1));

const WasmDeletedContentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmdeletedcontent_free(ptr >>> 0, 1));

const WasmEngineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmengine_free(ptr >>> 0, 1));

const WasmEngineConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmengineconfig_free(ptr >>> 0, 1));

const WasmMetricsRecorderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmetricsrecorder_free(ptr >>> 0, 1));

const WasmResourceLimitsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmresourcelimits_free(ptr >>> 0, 1));

const WasmTraversalConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmtraversalconfig_free(ptr >>> 0, 1));

const WasmTraversalEngineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmtraversalengine_free(ptr >>> 0, 1));

const WasmTraversalFilterFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmtraversalfilter_free(ptr >>> 0, 1));

const WasmUcpEventFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmucpevent_free(ptr >>> 0, 1));

const WasmValidationIssueFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmvalidationissue_free(ptr >>> 0, 1));

const WasmValidationPipelineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmvalidationpipeline_free(ptr >>> 0, 1));

const WasmValidationResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmvalidationresult_free(ptr >>> 0, 1));

const WasmWriteSectionResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmwritesectionresult_free(ptr >>> 0, 1));

/**
 * Content wrapper for WASM.
 */
class Content {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Content.prototype);
        obj.__wbg_ptr = ptr;
        ContentFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ContentFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_content_free(ptr, 0);
    }
    /**
     * Get size in bytes.
     * @returns {number}
     */
    get sizeBytes() {
        const ret = wasm.content_sizeBytes(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get the content type.
     * @returns {ContentType}
     */
    get contentType() {
        const ret = wasm.content_contentType(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create code content.
     * @param {string} language
     * @param {string} source
     * @returns {Content}
     */
    static code(language, source) {
        const ptr0 = passStringToWasm0(language, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(source, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.content_code(ptr0, len0, ptr1, len1);
        return Content.__wrap(ret);
    }
    /**
     * Create JSON content.
     * @param {any} value
     * @returns {Content}
     */
    static json(value) {
        const ret = wasm.content_json(value);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Content.__wrap(ret[0]);
    }
    /**
     * Create math content (LaTeX by default).
     * @param {string} expression
     * @param {boolean | null} [display_mode]
     * @param {string | null} [format]
     * @returns {Content}
     */
    static math(expression, display_mode, format) {
        const ptr0 = passStringToWasm0(expression, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(format) ? 0 : passStringToWasm0(format, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.content_math(ptr0, len0, isLikeNone(display_mode) ? 0xFFFFFF : display_mode ? 1 : 0, ptr1, len1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Content.__wrap(ret[0]);
    }
    /**
     * Create plain text content.
     * @param {string} text
     * @returns {Content}
     */
    static text(text) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.content_text(ptr0, len0);
        return Content.__wrap(ret);
    }
    /**
     * Create media content (image, audio, video, document).
     * @param {string} media_type
     * @param {string} url
     * @param {string | null} [alt_text]
     * @param {number | null} [width]
     * @param {number | null} [height]
     * @returns {Content}
     */
    static media(media_type, url, alt_text, width, height) {
        const ptr0 = passStringToWasm0(media_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(alt_text) ? 0 : passStringToWasm0(alt_text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.content_media(ptr0, len0, ptr1, len1, ptr2, len2, isLikeNone(width) ? 0x100000001 : (width) >>> 0, isLikeNone(height) ? 0x100000001 : (height) >>> 0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Content.__wrap(ret[0]);
    }
    /**
     * Create table content from rows.
     * @param {any} rows
     * @returns {Content}
     */
    static table(rows) {
        const ret = wasm.content_table(rows);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Content.__wrap(ret[0]);
    }
    /**
     * Create binary content.
     * @param {string} mime_type
     * @param {Uint8Array} data
     * @param {string | null} [encoding]
     * @returns {Content}
     */
    static binary(mime_type, data, encoding) {
        const ptr0 = passStringToWasm0(mime_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(encoding) ? 0 : passStringToWasm0(encoding, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.content_binary(ptr0, len0, ptr1, len1, ptr2, len2);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Content.__wrap(ret[0]);
    }
    /**
     * Get code content if this is a code block (returns object {language, source}).
     * @returns {any}
     */
    asCode() {
        const ret = wasm.content_asCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get JSON content if this is a JSON block.
     * @returns {any}
     */
    asJson() {
        const ret = wasm.content_asJson(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get math content if this is a math block (returns object {expression, displayMode, format}).
     * @returns {any}
     */
    asMath() {
        const ret = wasm.content_asMath(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get text content if this is a text block.
     * @returns {string | undefined}
     */
    asText() {
        const ret = wasm.content_asText(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Get media content if this is a media block (returns object {mediaType, url, altText}).
     * @returns {any}
     */
    asMedia() {
        const ret = wasm.content_asMedia(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get table content if this is a table block (returns object {columns, rows}).
     * @returns {any}
     */
    asTable() {
        const ret = wasm.content_asTable(this.__wbg_ptr);
        return ret;
    }
    /**
     * Check if empty.
     * @returns {boolean}
     */
    get isEmpty() {
        const ret = wasm.content_isEmpty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Create markdown text content.
     * @param {string} text
     * @returns {Content}
     */
    static markdown(text) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.content_markdown(ptr0, len0);
        return Content.__wrap(ret);
    }
    /**
     * Get the type tag string.
     * @returns {string}
     */
    get typeTag() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.content_typeTag(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get binary content if this is a binary block (returns object {mimeType, data}).
     * @returns {any}
     */
    asBinary() {
        const ret = wasm.content_asBinary(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create composite content (container for other blocks).
     * @param {string | null} [layout]
     * @param {string[] | null} [children]
     * @returns {Content}
     */
    static composite(layout, children) {
        var ptr0 = isLikeNone(layout) ? 0 : passStringToWasm0(layout, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(children) ? 0 : passArrayJsValueToWasm0(children, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.content_composite(ptr0, len0, ptr1, len1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Content.__wrap(ret[0]);
    }
}
if (Symbol.dispose) Content.prototype[Symbol.dispose] = Content.prototype.free;
exports.Content = Content;

/**
 * Content type enumeration.
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7}
 */
const ContentType = Object.freeze({
    Text: 0, "0": "Text",
    Code: 1, "1": "Code",
    Table: 2, "2": "Table",
    Math: 3, "3": "Math",
    Media: 4, "4": "Media",
    Json: 5, "5": "Json",
    Binary: 6, "6": "Binary",
    Composite: 7, "7": "Composite",
});
exports.ContentType = ContentType;

/**
 * A UCM document is a collection of blocks with hierarchical structure.
 */
class Document {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Document.prototype);
        obj.__wbg_ptr = ptr;
        DocumentFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DocumentFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_document_free(ptr, 0);
    }
    /**
     * Get created timestamp as ISO 8601 string.
     * @returns {string}
     */
    get createdAt() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.document_createdAt(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Edit a block's content.
     * @param {string} id
     * @param {string} content
     * @param {string | null} [role]
     */
    editBlock(id, content, role) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(role) ? 0 : passStringToWasm0(role, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.document_editBlock(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Move a block to a new parent.
     * @param {string} id
     * @param {string} new_parent_id
     * @param {number | null} [index]
     */
    moveBlock(id, new_parent_id, index) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(new_parent_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_moveBlock(this.__wbg_ptr, ptr0, len0, ptr1, len1, isLikeNone(index) ? 0x100000001 : (index) >>> 0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Remove a tag from a block.
     * @param {string} id
     * @param {string} tag
     * @returns {boolean}
     */
    removeTag(id, tag) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(tag, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_removeTag(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] !== 0;
    }
    /**
     * Get the total block count.
     * @returns {number}
     */
    blockCount() {
        const ret = wasm.document_blockCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get descendants of a block.
     * @param {string} id
     * @returns {Array<any>}
     */
    descendants(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_descendants(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Get the document description.
     * @returns {string | undefined}
     */
    get description() {
        const ret = wasm.document_description(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Find blocks by tag.
     * @param {string} tag
     * @returns {Array<any>}
     */
    findByTag(tag) {
        const ptr0 = passStringToWasm0(tag, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_findByTag(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * Check if one block is an ancestor of another.
     * @param {string} potential_ancestor
     * @param {string} block
     * @returns {boolean}
     */
    isAncestor(potential_ancestor, block) {
        const ptr0 = passStringToWasm0(potential_ancestor, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(block, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_isAncestor(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] !== 0;
    }
    /**
     * Get modified timestamp as ISO 8601 string.
     * @returns {string}
     */
    get modifiedAt() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.document_modifiedAt(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Remove an edge from one block to another.
     * @param {string} source_id
     * @param {EdgeType} edge_type
     * @param {string} target_id
     * @returns {boolean}
     */
    removeEdge(source_id, edge_type, target_id) {
        const ptr0 = passStringToWasm0(source_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(target_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_removeEdge(this.__wbg_ptr, ptr0, len0, edge_type, ptr1, len1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] !== 0;
    }
    /**
     * Add a block at a specific index.
     * @param {string} parent_id
     * @param {string} content
     * @param {number} index
     * @param {string | null} [role]
     * @param {string | null} [label]
     * @returns {string}
     */
    addBlockAt(parent_id, content, index, role, label) {
        let deferred6_0;
        let deferred6_1;
        try {
            const ptr0 = passStringToWasm0(parent_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            var ptr2 = isLikeNone(role) ? 0 : passStringToWasm0(role, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len2 = WASM_VECTOR_LEN;
            var ptr3 = isLikeNone(label) ? 0 : passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len3 = WASM_VECTOR_LEN;
            const ret = wasm.document_addBlockAt(this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2, ptr3, len3);
            var ptr5 = ret[0];
            var len5 = ret[1];
            if (ret[3]) {
                ptr5 = 0; len5 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred6_0 = ptr5;
            deferred6_1 = len5;
            return getStringFromWasm0(ptr5, len5);
        } finally {
            wasm.__wbindgen_free(deferred6_0, deferred6_1, 1);
        }
    }
    /**
     * Delete a block.
     * @param {string} id
     * @param {boolean | null} [cascade]
     * @returns {Array<any>}
     */
    deleteBlock(id, cascade) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_deleteBlock(this.__wbg_ptr, ptr0, len0, isLikeNone(cascade) ? 0xFFFFFF : cascade ? 1 : 0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Find blocks by semantic role.
     * @param {string} role
     * @returns {Array<any>}
     */
    findByRole(role) {
        const ptr0 = passStringToWasm0(role, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_findByRole(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * Find blocks by content type.
     * @param {string} content_type
     * @returns {Array<any>}
     */
    findByType(content_type) {
        const ptr0 = passStringToWasm0(content_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_findByType(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * Check if a block is reachable from root.
     * @param {string} id
     * @returns {boolean}
     */
    isReachable(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_isReachable(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] !== 0;
    }
    /**
     * Find a block by its label.
     * @param {string} label
     * @returns {string | undefined}
     */
    findByLabel(label) {
        const ptr0 = passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_findByLabel(this.__wbg_ptr, ptr0, len0);
        let v2;
        if (ret[0] !== 0) {
            v2 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v2;
    }
    /**
     * Get the index of a block among its siblings.
     * @param {string} id
     * @returns {number | undefined}
     */
    siblingIndex(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_siblingIndex(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] === 0x100000001 ? undefined : ret[0];
    }
    /**
     * Write markdown content into a section by block ID.
     * @param {string} section_id
     * @param {string} markdown
     * @param {number | null} [base_heading_level]
     * @returns {WasmWriteSectionResult}
     */
    writeSection(section_id, markdown, base_heading_level) {
        const ptr0 = passStringToWasm0(section_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(markdown, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_writeSection(this.__wbg_ptr, ptr0, len0, ptr1, len1, isLikeNone(base_heading_level) ? 0x100000001 : (base_heading_level) >>> 0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmWriteSectionResult.__wrap(ret[0]);
    }
    /**
     * Get incoming edges to a block.
     * @param {string} id
     * @returns {any}
     */
    incomingEdges(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_incomingEdges(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Get outgoing edges from a block.
     * @param {string} id
     * @returns {any}
     */
    outgoingEdges(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_outgoingEdges(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Get the path from root to a block (list of block IDs).
     * @param {string} id
     * @returns {Array<any>}
     */
    pathFromRoot(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_pathFromRoot(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Set the document description.
     * @param {string | null} [description]
     */
    set description(description) {
        var ptr0 = isLikeNone(description) ? 0 : passStringToWasm0(description, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.document_set_description(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Edit a block with specific content type.
     * @param {string} id
     * @param {Content} content
     * @param {string | null} [role]
     */
    editBlockContent(id, content, role) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(content, Content);
        var ptr1 = isLikeNone(role) ? 0 : passStringToWasm0(role, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_editBlockContent(this.__wbg_ptr, ptr0, len0, content.__wbg_ptr, ptr1, len1);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Add a block with specific content.
     * @param {string} parent_id
     * @param {Content} content
     * @param {string | null} [role]
     * @param {string | null} [label]
     * @returns {string}
     */
    addBlockWithContent(parent_id, content, role, label) {
        let deferred5_0;
        let deferred5_1;
        try {
            const ptr0 = passStringToWasm0(parent_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(content, Content);
            var ptr1 = isLikeNone(role) ? 0 : passStringToWasm0(role, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            var ptr2 = isLikeNone(label) ? 0 : passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len2 = WASM_VECTOR_LEN;
            const ret = wasm.document_addBlockWithContent(this.__wbg_ptr, ptr0, len0, content.__wbg_ptr, ptr1, len1, ptr2, len2);
            var ptr4 = ret[0];
            var len4 = ret[1];
            if (ret[3]) {
                ptr4 = 0; len4 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred5_0 = ptr4;
            deferred5_1 = len4;
            return getStringFromWasm0(ptr4, len4);
        } finally {
            wasm.__wbindgen_free(deferred5_0, deferred5_1, 1);
        }
    }
    /**
     * Get the document ID.
     * @returns {string}
     */
    get id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.document_id(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get the depth of a block from the root (root has depth 0).
     * @param {string} id
     * @returns {number}
     */
    depth(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_depth(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * Get the document title.
     * @returns {string | undefined}
     */
    get title() {
        const ret = wasm.document_title(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Get all blocks in the document.
     * @returns {any}
     */
    blocks() {
        const ret = wasm.document_blocks(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Create a new empty document.
     * @param {string | null} [title]
     */
    constructor(title) {
        var ptr0 = isLikeNone(title) ? 0 : passStringToWasm0(title, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        const ret = wasm.createDocument(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        DocumentFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Get parent of a block.
     * @param {string} child_id
     * @returns {string | undefined}
     */
    parent(child_id) {
        const ptr0 = passStringToWasm0(child_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_parent(this.__wbg_ptr, ptr0, len0);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        let v2;
        if (ret[0] !== 0) {
            v2 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v2;
    }
    /**
     * Add a tag to a block.
     * @param {string} id
     * @param {string} tag
     */
    addTag(id, tag) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(tag, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_addTag(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Get the root block ID.
     * @returns {string}
     */
    get rootId() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.document_rootId(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Serialize to JSON object.
     * @returns {any}
     */
    toJson() {
        const ret = wasm.document_toJson(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Get document version.
     * @returns {bigint}
     */
    get version() {
        const ret = wasm.document_version(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Add a code block.
     * @param {string} parent_id
     * @param {string} language
     * @param {string} source
     * @param {string | null} [label]
     * @returns {string}
     */
    addCode(parent_id, language, source, label) {
        let deferred6_0;
        let deferred6_1;
        try {
            const ptr0 = passStringToWasm0(parent_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(language, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(source, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len2 = WASM_VECTOR_LEN;
            var ptr3 = isLikeNone(label) ? 0 : passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len3 = WASM_VECTOR_LEN;
            const ret = wasm.document_addCode(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
            var ptr5 = ret[0];
            var len5 = ret[1];
            if (ret[3]) {
                ptr5 = 0; len5 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred6_0 = ptr5;
            deferred6_1 = len5;
            return getStringFromWasm0(ptr5, len5);
        } finally {
            wasm.__wbindgen_free(deferred6_0, deferred6_1, 1);
        }
    }
    /**
     * Add an edge from one block to another.
     * @param {string} source_id
     * @param {EdgeType} edge_type
     * @param {string} target_id
     */
    addEdge(source_id, edge_type, target_id) {
        const ptr0 = passStringToWasm0(source_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(target_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_addEdge(this.__wbg_ptr, ptr0, len0, edge_type, ptr1, len1);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Get children of a block.
     * @param {string} parent_id
     * @returns {Array<any>}
     */
    children(parent_id) {
        const ptr0 = passStringToWasm0(parent_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_children(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Get the siblings of a block (children of same parent, excluding self).
     * @param {string} id
     * @returns {Array<any>}
     */
    siblings(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_siblings(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Validate the document.
     * @returns {any}
     */
    validate() {
        const ret = wasm.document_validate(this.__wbg_ptr);
        return ret;
    }
    /**
     * Add a new text block.
     * @param {string} parent_id
     * @param {string} content
     * @param {string | null} [role]
     * @param {string | null} [label]
     * @returns {string}
     */
    addBlock(parent_id, content, role, label) {
        let deferred6_0;
        let deferred6_1;
        try {
            const ptr0 = passStringToWasm0(parent_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            var ptr2 = isLikeNone(role) ? 0 : passStringToWasm0(role, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len2 = WASM_VECTOR_LEN;
            var ptr3 = isLikeNone(label) ? 0 : passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len3 = WASM_VECTOR_LEN;
            const ret = wasm.document_addBlock(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
            var ptr5 = ret[0];
            var len5 = ret[1];
            if (ret[3]) {
                ptr5 = 0; len5 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred6_0 = ptr5;
            deferred6_1 = len5;
            return getStringFromWasm0(ptr5, len5);
        } finally {
            wasm.__wbindgen_free(deferred6_0, deferred6_1, 1);
        }
    }
    /**
     * Get all ancestors of a block (from parent to root).
     * @param {string} id
     * @returns {Array<any>}
     */
    ancestors(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_ancestors(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Get all block IDs in the document.
     * @returns {Array<any>}
     */
    blockIds() {
        const ret = wasm.document_blockIds(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get a block by ID (returns JSON representation).
     * @param {string} id
     * @returns {any}
     */
    getBlock(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.document_getBlock(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Set a block's label.
     * @param {string} id
     * @param {string | null} [label]
     */
    setLabel(id, label) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(label) ? 0 : passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.document_setLabel(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Set the document title.
     * @param {string | null} [title]
     */
    set title(title) {
        var ptr0 = isLikeNone(title) ? 0 : passStringToWasm0(title, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.document_set_title(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) Document.prototype[Symbol.dispose] = Document.prototype.free;
exports.Document = Document;

/**
 * Edge type enumeration.
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17}
 */
const EdgeType = Object.freeze({
    DerivedFrom: 0, "0": "DerivedFrom",
    Supersedes: 1, "1": "Supersedes",
    TransformedFrom: 2, "2": "TransformedFrom",
    References: 3, "3": "References",
    CitedBy: 4, "4": "CitedBy",
    LinksTo: 5, "5": "LinksTo",
    Supports: 6, "6": "Supports",
    Contradicts: 7, "7": "Contradicts",
    Elaborates: 8, "8": "Elaborates",
    Summarizes: 9, "9": "Summarizes",
    ParentOf: 10, "10": "ParentOf",
    ChildOf: 11, "11": "ChildOf",
    SiblingOf: 12, "12": "SiblingOf",
    PreviousSibling: 13, "13": "PreviousSibling",
    NextSibling: 14, "14": "NextSibling",
    VersionOf: 15, "15": "VersionOf",
    AlternativeOf: 16, "16": "AlternativeOf",
    TranslationOf: 17, "17": "TranslationOf",
});
exports.EdgeType = EdgeType;

/**
 * Bidirectional mapping between BlockIds and short numeric IDs.
 *
 * Useful for token-efficient LLM prompts by replacing long block IDs
 * with short numeric identifiers.
 */
class IdMapper {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(IdMapper.prototype);
        obj.__wbg_ptr = ptr;
        IdMapperFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IdMapperFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_idmapper_free(ptr, 0);
    }
    /**
     * Convert UCL commands from short numeric IDs back to full BlockIds.
     * @param {string} ucl
     * @returns {string}
     */
    expandUcl(ucl) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(ucl, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.idmapper_expandUcl(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Convert a string containing short IDs back to block IDs.
     * @param {string} text
     * @returns {string}
     */
    expandText(text) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.idmapper_expandText(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Convert UCL commands from long BlockIds to short numeric IDs.
     * @param {string} ucl
     * @returns {string}
     */
    shortenUcl(ucl) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(ucl, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.idmapper_shortenUcl(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Get BlockId for a short ID.
     * @param {number} short_id
     * @returns {string | undefined}
     */
    toBlockId(short_id) {
        const ret = wasm.idmapper_toBlockId(this.__wbg_ptr, short_id);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Get short ID for a BlockId.
     * @param {string} block_id
     * @returns {number | undefined}
     */
    toShortId(block_id) {
        const ptr0 = passStringToWasm0(block_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.idmapper_toShortId(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] === 0x100000001 ? undefined : ret[0];
    }
    /**
     * Convert a string containing block IDs to use short IDs.
     * @param {string} text
     * @returns {string}
     */
    shortenText(text) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.idmapper_shortenText(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Create a mapper from a document, assigning sequential IDs to all blocks.
     * @param {Document} doc
     * @returns {IdMapper}
     */
    static fromDocument(doc) {
        _assertClass(doc, Document);
        const ret = wasm.idmapper_fromDocument(doc.__wbg_ptr);
        return IdMapper.__wrap(ret);
    }
    /**
     * Get the mapping table as a string (useful for debugging).
     * @returns {string}
     */
    mappingTable() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.idmapper_mappingTable(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Generate a normalized document representation for LLM prompts.
     * @param {Document} doc
     * @returns {string}
     */
    documentToPrompt(doc) {
        let deferred1_0;
        let deferred1_1;
        try {
            _assertClass(doc, Document);
            const ret = wasm.idmapper_documentToPrompt(this.__wbg_ptr, doc.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Estimate token savings from using short IDs.
     * Returns { originalTokens, shortenedTokens, savings }.
     * @param {string} text
     * @returns {any}
     */
    estimateTokenSavings(text) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.idmapper_estimateTokenSavings(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * Create a new empty IdMapper.
     */
    constructor() {
        const ret = wasm.idmapper_new();
        this.__wbg_ptr = ret >>> 0;
        IdMapperFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Total number of mappings.
     * @returns {number}
     */
    get length() {
        const ret = wasm.idmapper_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Register a BlockId and get its short ID.
     * @param {string} block_id
     * @returns {number}
     */
    register(block_id) {
        const ptr0 = passStringToWasm0(block_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.idmapper_register(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
}
if (Symbol.dispose) IdMapper.prototype[Symbol.dispose] = IdMapper.prototype.free;
exports.IdMapper = IdMapper;

/**
 * Builder for constructing LLM prompts with specific capabilities.
 */
class PromptBuilder {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PromptBuilder.prototype);
        obj.__wbg_ptr = ptr;
        PromptBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PromptBuilderFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_promptbuilder_free(ptr, 0);
    }
    /**
     * Build a complete prompt with document context.
     * @param {string} document_description
     * @param {string} task
     * @returns {string}
     */
    buildPrompt(document_description, task) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(document_description, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(task, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.promptbuilder_buildPrompt(this.__wbg_ptr, ptr0, len0, ptr1, len1);
            deferred3_0 = ret[0];
            deferred3_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * Check if a capability is enabled.
     * @param {WasmUclCapability} cap
     * @returns {boolean}
     */
    hasCapability(cap) {
        const ret = wasm.promptbuilder_hasCapability(this.__wbg_ptr, cap);
        return ret !== 0;
    }
    /**
     * Enable short ID mode (for token efficiency).
     * @param {boolean} enabled
     * @returns {PromptBuilder}
     */
    withShortIds(enabled) {
        const ret = wasm.promptbuilder_withShortIds(this.__wbg_ptr, enabled);
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Add a single capability.
     * @param {WasmUclCapability} cap
     * @returns {PromptBuilder}
     */
    withCapability(cap) {
        const ret = wasm.promptbuilder_withCapability(this.__wbg_ptr, cap);
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Set task-specific context.
     * @param {string} context
     * @returns {PromptBuilder}
     */
    withTaskContext(context) {
        const ptr0 = passStringToWasm0(context, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.promptbuilder_withTaskContext(this.__wbg_ptr, ptr0, len0);
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Remove a capability.
     * @param {WasmUclCapability} cap
     * @returns {PromptBuilder}
     */
    withoutCapability(cap) {
        const ret = wasm.promptbuilder_withoutCapability(this.__wbg_ptr, cap);
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Build the system prompt.
     * @returns {string}
     */
    buildSystemPrompt() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.promptbuilder_buildSystemPrompt(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Set custom system context (prepended to prompt).
     * @param {string} context
     * @returns {PromptBuilder}
     */
    withSystemContext(context) {
        const ptr0 = passStringToWasm0(context, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.promptbuilder_withSystemContext(this.__wbg_ptr, ptr0, len0);
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Create a builder with all capabilities enabled.
     * @returns {PromptBuilder}
     */
    static withAllCapabilities() {
        const ret = wasm.promptbuilder_withAllCapabilities();
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Create a new prompt builder with no capabilities.
     */
    constructor() {
        const ret = wasm.promptbuilder_new();
        this.__wbg_ptr = ret >>> 0;
        PromptBuilderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Add a custom rule.
     * @param {string} rule
     * @returns {PromptBuilder}
     */
    withRule(rule) {
        const ptr0 = passStringToWasm0(rule, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.promptbuilder_withRule(this.__wbg_ptr, ptr0, len0);
        return PromptBuilder.__wrap(ret);
    }
}
if (Symbol.dispose) PromptBuilder.prototype[Symbol.dispose] = PromptBuilder.prototype.free;
exports.PromptBuilder = PromptBuilder;

/**
 * Preset prompt configurations for common use cases.
 */
class PromptPresets {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PromptPresetsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_promptpresets_free(ptr, 0);
    }
    /**
     * Full document editing (all except transactions).
     * @returns {PromptBuilder}
     */
    static fullEditing() {
        const ret = wasm.promptpresets_fullEditing();
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Basic editing only (EDIT, APPEND, DELETE).
     * @returns {PromptBuilder}
     */
    static basicEditing() {
        const ret = wasm.promptpresets_basicEditing();
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Token-efficient mode with short IDs.
     * @returns {PromptBuilder}
     */
    static tokenEfficient() {
        const ret = wasm.promptpresets_tokenEfficient();
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Version control focused.
     * @returns {PromptBuilder}
     */
    static versionControl() {
        const ret = wasm.promptpresets_versionControl();
        return PromptBuilder.__wrap(ret);
    }
    /**
     * Structure manipulation (MOVE, LINK).
     * @returns {PromptBuilder}
     */
    static structureManipulation() {
        const ret = wasm.promptpresets_structureManipulation();
        return PromptBuilder.__wrap(ret);
    }
}
if (Symbol.dispose) PromptPresets.prototype[Symbol.dispose] = PromptPresets.prototype.free;
exports.PromptPresets = PromptPresets;

/**
 * Manages document snapshots for versioning.
 */
class SnapshotManager {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SnapshotManagerFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_snapshotmanager_free(ptr, 0);
    }
    /**
     * Get information about a snapshot.
     * @param {string} name
     * @returns {any}
     */
    get(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.snapshotmanager_get(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * Create a new snapshot manager.
     * @param {number | null} [max_snapshots]
     */
    constructor(max_snapshots) {
        const ret = wasm.snapshotmanager_new(isLikeNone(max_snapshots) ? 0x100000001 : (max_snapshots) >>> 0);
        this.__wbg_ptr = ret >>> 0;
        SnapshotManagerFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * List all snapshots (most recent first).
     * @returns {any}
     */
    list() {
        const ret = wasm.snapshotmanager_list(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create a snapshot of a document.
     * @param {string} name
     * @param {Document} doc
     * @param {string | null} [description]
     * @returns {string}
     */
    create(name, doc, description) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(doc, Document);
            var ptr1 = isLikeNone(description) ? 0 : passStringToWasm0(description, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            const ret = wasm.snapshotmanager_create(this.__wbg_ptr, ptr0, len0, doc.__wbg_ptr, ptr1, len1);
            var ptr3 = ret[0];
            var len3 = ret[1];
            if (ret[3]) {
                ptr3 = 0; len3 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred4_0 = ptr3;
            deferred4_1 = len3;
            return getStringFromWasm0(ptr3, len3);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * Delete a snapshot.
     * @param {string} name
     * @returns {boolean}
     */
    delete(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.snapshotmanager_delete(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Check if a snapshot exists.
     * @param {string} name
     * @returns {boolean}
     */
    exists(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.snapshotmanager_exists(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Get snapshot count.
     * @returns {number}
     */
    get length() {
        const ret = wasm.idmapper_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Restore a document from a snapshot.
     * @param {string} name
     * @returns {Document}
     */
    restore(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.snapshotmanager_restore(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Document.__wrap(ret[0]);
    }
}
if (Symbol.dispose) SnapshotManager.prototype[Symbol.dispose] = SnapshotManager.prototype.free;
exports.SnapshotManager = SnapshotManager;

/**
 * Audit log entry.
 */
class WasmAuditEntry {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmAuditEntry.prototype);
        obj.__wbg_ptr = ptr;
        WasmAuditEntryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmAuditEntryFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmauditentry_free(ptr, 0);
    }
    /**
     * Get the document ID.
     * @returns {string}
     */
    get documentId() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmauditentry_documentId(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get the duration in milliseconds.
     * @returns {bigint}
     */
    get durationMs() {
        const ret = wasm.wasmauditentry_durationMs(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Set the duration in milliseconds.
     * @param {bigint} duration_ms
     * @returns {WasmAuditEntry}
     */
    withDuration(duration_ms) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.wasmauditentry_withDuration(ptr, duration_ms);
        return WasmAuditEntry.__wrap(ret);
    }
    /**
     * Create a new audit entry.
     * @param {string} operation
     * @param {string} document_id
     */
    constructor(operation, document_id) {
        const ptr0 = passStringToWasm0(operation, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(document_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmauditentry_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret >>> 0;
        WasmAuditEntryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Mark as failed.
     * @returns {WasmAuditEntry}
     */
    failed() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.wasmauditentry_failed(ptr);
        return WasmAuditEntry.__wrap(ret);
    }
    /**
     * Check if the operation was successful.
     * @returns {boolean}
     */
    get success() {
        const ret = wasm.wasmauditentry_success(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Convert to JSON object.
     * @returns {any}
     */
    toJson() {
        const ret = wasm.wasmauditentry_toJson(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the user ID if present.
     * @returns {string | undefined}
     */
    get userId() {
        const ret = wasm.wasmauditentry_userId(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Get the operation name.
     * @returns {string}
     */
    get operation() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmauditentry_operation(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get the timestamp as ISO 8601 string.
     * @returns {string}
     */
    get timestamp() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmauditentry_timestamp(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Set the user ID.
     * @param {string} user_id
     * @returns {WasmAuditEntry}
     */
    withUser(user_id) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(user_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmauditentry_withUser(ptr, ptr0, len0);
        return WasmAuditEntry.__wrap(ret);
    }
}
if (Symbol.dispose) WasmAuditEntry.prototype[Symbol.dispose] = WasmAuditEntry.prototype.free;
exports.WasmAuditEntry = WasmAuditEntry;

/**
 * Result of a section clear operation with undo support.
 */
class WasmClearResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmClearResult.prototype);
        obj.__wbg_ptr = ptr;
        WasmClearResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmClearResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmclearresult_free(ptr, 0);
    }
    /**
     * Get the IDs of removed blocks.
     * @returns {Array<any>}
     */
    get removedIds() {
        const ret = wasm.wasmclearresult_removedIds(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the deleted content for potential restoration.
     * @returns {WasmDeletedContent}
     */
    get deletedContent() {
        const ret = wasm.wasmclearresult_deletedContent(this.__wbg_ptr);
        return WasmDeletedContent.__wrap(ret);
    }
    /**
     * Get the number of removed blocks.
     * @returns {number}
     */
    get length() {
        const ret = wasm.wasmclearresult_length(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) WasmClearResult.prototype[Symbol.dispose] = WasmClearResult.prototype.free;
exports.WasmClearResult = WasmClearResult;

/**
 * Deleted content that can be restored.
 */
class WasmDeletedContent {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmDeletedContent.prototype);
        obj.__wbg_ptr = ptr;
        WasmDeletedContentFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmDeletedContentFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmdeletedcontent_free(ptr, 0);
    }
    /**
     * Get the deletion timestamp as ISO 8601 string.
     * @returns {string}
     */
    get deletedAt() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmdeletedcontent_deletedAt(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get the number of deleted blocks.
     * @returns {number}
     */
    get blockCount() {
        const ret = wasm.idmapper_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Serialize to JSON string for persistence.
     * @returns {string}
     */
    toJson() {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.wasmdeletedcontent_toJson(this.__wbg_ptr);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Check if there is any deleted content.
     * @returns {boolean}
     */
    get isEmpty() {
        const ret = wasm.wasmdeletedcontent_isEmpty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Get all block IDs in the deleted content.
     * @returns {Array<any>}
     */
    blockIds() {
        const ret = wasm.wasmdeletedcontent_blockIds(this.__wbg_ptr);
        return ret;
    }
    /**
     * Deserialize from JSON string.
     * @param {string} json_str
     * @returns {WasmDeletedContent}
     */
    static fromJson(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmdeletedcontent_fromJson(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmDeletedContent.__wrap(ret[0]);
    }
    /**
     * Get the parent block ID where this content was attached.
     * @returns {string}
     */
    get parentId() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmdeletedcontent_parentId(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) WasmDeletedContent.prototype[Symbol.dispose] = WasmDeletedContent.prototype.free;
exports.WasmDeletedContent = WasmDeletedContent;

/**
 * The main transformation engine with transaction support.
 */
class WasmEngine {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmEngineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmengine_free(ptr, 0);
    }
    /**
     * List all snapshots.
     * @returns {Array<any>}
     */
    listSnapshots() {
        const ret = wasm.wasmengine_listSnapshots(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create a snapshot.
     * @param {string} name
     * @param {Document} doc
     * @param {string | null} [description]
     */
    createSnapshot(name, doc, description) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(doc, Document);
        var ptr1 = isLikeNone(description) ? 0 : passStringToWasm0(description, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmengine_createSnapshot(this.__wbg_ptr, ptr0, len0, doc.__wbg_ptr, ptr1, len1);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Delete a snapshot.
     * @param {string} name
     * @returns {boolean}
     */
    deleteSnapshot(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmengine_deleteSnapshot(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Restore from a snapshot.
     * @param {string} name
     * @returns {Document}
     */
    restoreSnapshot(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmengine_restoreSnapshot(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Document.__wrap(ret[0]);
    }
    /**
     * Begin a new transaction.
     * @returns {string}
     */
    beginTransaction() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmengine_beginTransaction(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Rollback a transaction.
     * @param {string} txn_id
     */
    rollbackTransaction(txn_id) {
        const ptr0 = passStringToWasm0(txn_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmengine_rollbackTransaction(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Begin a named transaction.
     * @param {string} name
     * @returns {string}
     */
    beginNamedTransaction(name) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.wasmengine_beginNamedTransaction(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {WasmEngineConfig | null} [config]
     */
    constructor(config) {
        let ptr0 = 0;
        if (!isLikeNone(config)) {
            _assertClass(config, WasmEngineConfig);
            ptr0 = config.__destroy_into_raw();
        }
        const ret = wasm.wasmengine_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        WasmEngineFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Validate a document.
     * @param {Document} doc
     * @returns {WasmValidationResult}
     */
    validate(doc) {
        _assertClass(doc, Document);
        const ret = wasm.wasmengine_validate(this.__wbg_ptr, doc.__wbg_ptr);
        return WasmValidationResult.__wrap(ret);
    }
}
if (Symbol.dispose) WasmEngine.prototype[Symbol.dispose] = WasmEngine.prototype.free;
exports.WasmEngine = WasmEngine;

/**
 * Engine configuration.
 */
class WasmEngineConfig {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmEngineConfigFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmengineconfig_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get maxBatchSize() {
        const ret = wasm.wasmengineconfig_maxBatchSize(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    get enableSnapshots() {
        const ret = wasm.wasmengineconfig_enableSnapshots(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get enableTransactions() {
        const ret = wasm.wasmengineconfig_enableTransactions(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get validateOnOperation() {
        const ret = wasm.wasmengineconfig_validateOnOperation(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean | null} [validate_on_operation]
     * @param {number | null} [max_batch_size]
     * @param {boolean | null} [enable_transactions]
     * @param {boolean | null} [enable_snapshots]
     */
    constructor(validate_on_operation, max_batch_size, enable_transactions, enable_snapshots) {
        const ret = wasm.wasmengineconfig_new(isLikeNone(validate_on_operation) ? 0xFFFFFF : validate_on_operation ? 1 : 0, isLikeNone(max_batch_size) ? 0x100000001 : (max_batch_size) >>> 0, isLikeNone(enable_transactions) ? 0xFFFFFF : enable_transactions ? 1 : 0, isLikeNone(enable_snapshots) ? 0xFFFFFF : enable_snapshots ? 1 : 0);
        this.__wbg_ptr = ret >>> 0;
        WasmEngineConfigFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmEngineConfig.prototype[Symbol.dispose] = WasmEngineConfig.prototype.free;
exports.WasmEngineConfig = WasmEngineConfig;

/**
 * Simple metrics recorder.
 */
class WasmMetricsRecorder {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMetricsRecorderFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmetricsrecorder_free(ptr, 0);
    }
    /**
     * Get blocks created count.
     * @returns {bigint}
     */
    get blocksCreated() {
        const ret = wasm.wasmmetricsrecorder_blocksCreated(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Get blocks deleted count.
     * @returns {bigint}
     */
    get blocksDeleted() {
        const ret = wasm.wasmmetricsrecorder_blocksDeleted(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Record a snapshot creation.
     */
    recordSnapshot() {
        wasm.wasmmetricsrecorder_recordSnapshot(this.__wbg_ptr);
    }
    /**
     * Get total operations count.
     * @returns {bigint}
     */
    get operationsTotal() {
        const ret = wasm.wasmauditentry_durationMs(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Record an operation.
     * @param {boolean} success
     */
    recordOperation(success) {
        wasm.wasmmetricsrecorder_recordOperation(this.__wbg_ptr, success);
    }
    /**
     * Get failed operations count.
     * @returns {bigint}
     */
    get operationsFailed() {
        const ret = wasm.wasmmetricsrecorder_operationsFailed(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Get snapshots created count.
     * @returns {bigint}
     */
    get snapshotsCreated() {
        const ret = wasm.wasmmetricsrecorder_snapshotsCreated(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Record a block creation.
     */
    recordBlockCreated() {
        wasm.wasmmetricsrecorder_recordBlockCreated(this.__wbg_ptr);
    }
    /**
     * Record a block deletion.
     */
    recordBlockDeleted() {
        wasm.wasmmetricsrecorder_recordBlockDeleted(this.__wbg_ptr);
    }
    /**
     * Create a new metrics recorder.
     */
    constructor() {
        const ret = wasm.wasmmetricsrecorder_new();
        this.__wbg_ptr = ret >>> 0;
        WasmMetricsRecorderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Convert to JSON object.
     * @returns {any}
     */
    toJson() {
        const ret = wasm.wasmmetricsrecorder_toJson(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmMetricsRecorder.prototype[Symbol.dispose] = WasmMetricsRecorder.prototype.free;
exports.WasmMetricsRecorder = WasmMetricsRecorder;

/**
 * Resource limits for validation.
 */
class WasmResourceLimits {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmResourceLimits.prototype);
        obj.__wbg_ptr = ptr;
        WasmResourceLimitsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmResourceLimitsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmresourcelimits_free(ptr, 0);
    }
    /**
     * Create default resource limits.
     * @returns {WasmResourceLimits}
     */
    static defaultLimits() {
        const ret = wasm.wasmresourcelimits_defaultLimits();
        return WasmResourceLimits.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get maxBlockSize() {
        const ret = wasm.wasmresourcelimits_maxBlockSize(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get maxBlockCount() {
        const ret = wasm.wasmresourcelimits_maxBlockCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get maxDocumentSize() {
        const ret = wasm.wasmengineconfig_maxBatchSize(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get maxNestingDepth() {
        const ret = wasm.wasmresourcelimits_maxNestingDepth(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get maxEdgesPerBlock() {
        const ret = wasm.idmapper_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number | null} [max_document_size]
     * @param {number | null} [max_block_count]
     * @param {number | null} [max_block_size]
     * @param {number | null} [max_nesting_depth]
     * @param {number | null} [max_edges_per_block]
     */
    constructor(max_document_size, max_block_count, max_block_size, max_nesting_depth, max_edges_per_block) {
        const ret = wasm.wasmresourcelimits_new(isLikeNone(max_document_size) ? 0x100000001 : (max_document_size) >>> 0, isLikeNone(max_block_count) ? 0x100000001 : (max_block_count) >>> 0, isLikeNone(max_block_size) ? 0x100000001 : (max_block_size) >>> 0, isLikeNone(max_nesting_depth) ? 0x100000001 : (max_nesting_depth) >>> 0, isLikeNone(max_edges_per_block) ? 0x100000001 : (max_edges_per_block) >>> 0);
        this.__wbg_ptr = ret >>> 0;
        WasmResourceLimitsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Convert to JSON object.
     * @returns {any}
     */
    toJson() {
        const ret = wasm.wasmresourcelimits_toJson(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmResourceLimits.prototype[Symbol.dispose] = WasmResourceLimits.prototype.free;
exports.WasmResourceLimits = WasmResourceLimits;

/**
 * Traversal configuration.
 */
class WasmTraversalConfig {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmTraversalConfigFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtraversalconfig_free(ptr, 0);
    }
    /**
     * @param {number | null} [max_depth]
     * @param {number | null} [max_nodes]
     * @param {boolean | null} [include_orphans]
     */
    constructor(max_depth, max_nodes, include_orphans) {
        const ret = wasm.wasmtraversalconfig_new(isLikeNone(max_depth) ? 0x100000001 : (max_depth) >>> 0, isLikeNone(max_nodes) ? 0x100000001 : (max_nodes) >>> 0, isLikeNone(include_orphans) ? 0xFFFFFF : include_orphans ? 1 : 0);
        this.__wbg_ptr = ret >>> 0;
        WasmTraversalConfigFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get maxDepth() {
        const ret = wasm.wasmengineconfig_maxBatchSize(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get maxNodes() {
        const ret = wasm.wasmresourcelimits_maxBlockCount(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) WasmTraversalConfig.prototype[Symbol.dispose] = WasmTraversalConfig.prototype.free;
exports.WasmTraversalConfig = WasmTraversalConfig;

/**
 * Graph traversal engine for UCM documents.
 */
class WasmTraversalEngine {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmTraversalEngineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtraversalengine_free(ptr, 0);
    }
    /**
     * Find all paths between two nodes.
     * @param {Document} doc
     * @param {string} from_id
     * @param {string} to_id
     * @param {number | null} [max_paths]
     * @returns {Array<any>}
     */
    findPaths(doc, from_id, to_id, max_paths) {
        _assertClass(doc, Document);
        const ptr0 = passStringToWasm0(from_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(to_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmtraversalengine_findPaths(this.__wbg_ptr, doc.__wbg_ptr, ptr0, len0, ptr1, len1, isLikeNone(max_paths) ? 0x100000001 : (max_paths) >>> 0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Get the path from a node to the root.
     * @param {Document} doc
     * @param {string} node_id
     * @returns {Array<any>}
     */
    pathToRoot(doc, node_id) {
        _assertClass(doc, Document);
        const ptr0 = passStringToWasm0(node_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmtraversalengine_pathToRoot(this.__wbg_ptr, doc.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * @param {WasmTraversalConfig | null} [config]
     */
    constructor(config) {
        let ptr0 = 0;
        if (!isLikeNone(config)) {
            _assertClass(config, WasmTraversalConfig);
            ptr0 = config.__destroy_into_raw();
        }
        const ret = wasm.wasmtraversalengine_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        WasmTraversalEngineFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Expand a node to get its immediate children.
     * @param {Document} doc
     * @param {string} node_id
     * @returns {any}
     */
    expand(doc, node_id) {
        _assertClass(doc, Document);
        const ptr0 = passStringToWasm0(node_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmtraversalengine_expand(this.__wbg_ptr, doc.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Navigate from a starting point in a specific direction.
     * @param {Document} doc
     * @param {string} direction
     * @param {string | null} [start_id]
     * @param {number | null} [depth]
     * @param {WasmTraversalFilter | null} [filter]
     * @returns {any}
     */
    navigate(doc, direction, start_id, depth, filter) {
        _assertClass(doc, Document);
        const ptr0 = passStringToWasm0(direction, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(start_id) ? 0 : passStringToWasm0(start_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        let ptr2 = 0;
        if (!isLikeNone(filter)) {
            _assertClass(filter, WasmTraversalFilter);
            ptr2 = filter.__destroy_into_raw();
        }
        const ret = wasm.wasmtraversalengine_navigate(this.__wbg_ptr, doc.__wbg_ptr, ptr0, len0, ptr1, len1, isLikeNone(depth) ? 0x100000001 : (depth) >>> 0, ptr2);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
}
if (Symbol.dispose) WasmTraversalEngine.prototype[Symbol.dispose] = WasmTraversalEngine.prototype.free;
exports.WasmTraversalEngine = WasmTraversalEngine;

/**
 * Traversal filter for filtering blocks during traversal.
 */
class WasmTraversalFilter {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmTraversalFilterFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtraversalfilter_free(ptr, 0);
    }
    /**
     * @param {string[] | null} [include_roles]
     * @param {string[] | null} [exclude_roles]
     * @param {string[] | null} [include_tags]
     * @param {string[] | null} [exclude_tags]
     * @param {string | null} [content_pattern]
     */
    constructor(include_roles, exclude_roles, include_tags, exclude_tags, content_pattern) {
        var ptr0 = isLikeNone(include_roles) ? 0 : passArrayJsValueToWasm0(include_roles, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(exclude_roles) ? 0 : passArrayJsValueToWasm0(exclude_roles, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(include_tags) ? 0 : passArrayJsValueToWasm0(include_tags, wasm.__wbindgen_malloc);
        var len2 = WASM_VECTOR_LEN;
        var ptr3 = isLikeNone(exclude_tags) ? 0 : passArrayJsValueToWasm0(exclude_tags, wasm.__wbindgen_malloc);
        var len3 = WASM_VECTOR_LEN;
        var ptr4 = isLikeNone(content_pattern) ? 0 : passStringToWasm0(content_pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len4 = WASM_VECTOR_LEN;
        const ret = wasm.wasmtraversalfilter_new(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
        this.__wbg_ptr = ret >>> 0;
        WasmTraversalFilterFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmTraversalFilter.prototype[Symbol.dispose] = WasmTraversalFilter.prototype.free;
exports.WasmTraversalFilter = WasmTraversalFilter;

/**
 * UCL command capability enumeration.
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6}
 */
const WasmUclCapability = Object.freeze({
    Edit: 0, "0": "Edit",
    Append: 1, "1": "Append",
    Move: 2, "2": "Move",
    Delete: 3, "3": "Delete",
    Link: 4, "4": "Link",
    Snapshot: 5, "5": "Snapshot",
    Transaction: 6, "6": "Transaction",
});
exports.WasmUclCapability = WasmUclCapability;

/**
 * UCP event wrapper for WASM.
 */
class WasmUcpEvent {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmUcpEvent.prototype);
        obj.__wbg_ptr = ptr;
        WasmUcpEventFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmUcpEventFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmucpevent_free(ptr, 0);
    }
    /**
     * Get the event type.
     * @returns {string}
     */
    get eventType() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmucpevent_eventType(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Create a block added event.
     * @param {string} document_id
     * @param {string} block_id
     * @param {string} parent_id
     * @param {string} content_type
     * @returns {WasmUcpEvent}
     */
    static blockAdded(document_id, block_id, parent_id, content_type) {
        const ptr0 = passStringToWasm0(document_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(block_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(parent_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(content_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.wasmucpevent_blockAdded(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        return WasmUcpEvent.__wrap(ret);
    }
    /**
     * Get the document ID if present.
     * @returns {string | undefined}
     */
    get documentId() {
        const ret = wasm.wasmucpevent_documentId(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Create a block deleted event.
     * @param {string} document_id
     * @param {string} block_id
     * @param {boolean} cascade
     * @returns {WasmUcpEvent}
     */
    static blockDeleted(document_id, block_id, cascade) {
        const ptr0 = passStringToWasm0(document_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(block_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmucpevent_blockDeleted(ptr0, len0, ptr1, len1, cascade);
        return WasmUcpEvent.__wrap(ret);
    }
    /**
     * Create a document created event.
     * @param {string} document_id
     * @returns {WasmUcpEvent}
     */
    static documentCreated(document_id) {
        const ptr0 = passStringToWasm0(document_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmucpevent_documentCreated(ptr0, len0);
        return WasmUcpEvent.__wrap(ret);
    }
    /**
     * Create a snapshot created event.
     * @param {string} document_id
     * @param {string} snapshot_name
     * @returns {WasmUcpEvent}
     */
    static snapshotCreated(document_id, snapshot_name) {
        const ptr0 = passStringToWasm0(document_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(snapshot_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmucpevent_snapshotCreated(ptr0, len0, ptr1, len1);
        return WasmUcpEvent.__wrap(ret);
    }
    /**
     * Get event details as JSON string.
     * @returns {string}
     */
    get details() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmucpevent_details(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Convert to JSON object.
     * @returns {any}
     */
    toJson() {
        const ret = wasm.wasmucpevent_toJson(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the timestamp as ISO 8601 string.
     * @returns {string}
     */
    get timestamp() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmucpevent_timestamp(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) WasmUcpEvent.prototype[Symbol.dispose] = WasmUcpEvent.prototype.free;
exports.WasmUcpEvent = WasmUcpEvent;

/**
 * A single validation issue.
 */
class WasmValidationIssue {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmValidationIssueFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmvalidationissue_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get code() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmvalidationissue_code(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmvalidationissue_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Convert to JSON object.
     * @returns {any}
     */
    toJson() {
        const ret = wasm.wasmvalidationissue_toJson(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get severity() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmvalidationissue_severity(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) WasmValidationIssue.prototype[Symbol.dispose] = WasmValidationIssue.prototype.free;
exports.WasmValidationIssue = WasmValidationIssue;

/**
 * Validation pipeline with configurable resource limits.
 */
class WasmValidationPipeline {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmValidationPipelineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmvalidationpipeline_free(ptr, 0);
    }
    /**
     * @param {WasmResourceLimits | null} [limits]
     */
    constructor(limits) {
        let ptr0 = 0;
        if (!isLikeNone(limits)) {
            _assertClass(limits, WasmResourceLimits);
            ptr0 = limits.__destroy_into_raw();
        }
        const ret = wasm.wasmvalidationpipeline_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        WasmValidationPipelineFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Validate a document.
     * @param {Document} doc
     * @returns {WasmValidationResult}
     */
    validate(doc) {
        _assertClass(doc, Document);
        const ret = wasm.wasmvalidationpipeline_validate(this.__wbg_ptr, doc.__wbg_ptr);
        return WasmValidationResult.__wrap(ret);
    }
}
if (Symbol.dispose) WasmValidationPipeline.prototype[Symbol.dispose] = WasmValidationPipeline.prototype.free;
exports.WasmValidationPipeline = WasmValidationPipeline;

/**
 * Validation result.
 */
class WasmValidationResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmValidationResult.prototype);
        obj.__wbg_ptr = ptr;
        WasmValidationResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmValidationResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmvalidationresult_free(ptr, 0);
    }
    /**
     * Get error count.
     * @returns {number}
     */
    get errorCount() {
        const ret = wasm.wasmvalidationresult_errorCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get warning count.
     * @returns {number}
     */
    get warningCount() {
        const ret = wasm.wasmvalidationresult_warningCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    get valid() {
        const ret = wasm.wasmvalidationresult_valid(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {Array<any>}
     */
    get issues() {
        const ret = wasm.wasmvalidationresult_issues(this.__wbg_ptr);
        return ret;
    }
    /**
     * Convert to JSON object.
     * @returns {any}
     */
    toJson() {
        const ret = wasm.wasmvalidationresult_toJson(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmValidationResult.prototype[Symbol.dispose] = WasmValidationResult.prototype.free;
exports.WasmValidationResult = WasmValidationResult;

/**
 * Result of writing markdown into a section.
 */
class WasmWriteSectionResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmWriteSectionResult.prototype);
        obj.__wbg_ptr = ptr;
        WasmWriteSectionResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmWriteSectionResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmwritesectionresult_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get sectionId() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmwritesectionresult_sectionId(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {Array<any>}
     */
    get blocksAdded() {
        const ret = wasm.wasmwritesectionresult_blocksAdded(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get blocksRemoved() {
        const ret = wasm.wasmwritesectionresult_blocksRemoved(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get success() {
        const ret = wasm.wasmwritesectionresult_success(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) WasmWriteSectionResult.prototype[Symbol.dispose] = WasmWriteSectionResult.prototype.free;
exports.WasmWriteSectionResult = WasmWriteSectionResult;

/**
 * Clear a section's content with undo support.
 * @param {Document} doc
 * @param {string} section_id
 * @returns {WasmClearResult}
 */
function clearSectionWithUndo(doc, section_id) {
    _assertClass(doc, Document);
    const ptr0 = passStringToWasm0(section_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.clearSectionWithUndo(doc.__wbg_ptr, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return WasmClearResult.__wrap(ret[0]);
}
exports.clearSectionWithUndo = clearSectionWithUndo;

/**
 * Create a new empty document.
 * @param {string | null} [title]
 * @returns {Document}
 */
function createDocument(title) {
    var ptr0 = isLikeNone(title) ? 0 : passStringToWasm0(title, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.createDocument(ptr0, len0);
    return Document.__wrap(ret);
}
exports.createDocument = createDocument;

/**
 * Execute UCL commands on a document.
 * @param {Document} doc
 * @param {string} ucl
 * @returns {Array<any>}
 */
function executeUcl(doc, ucl) {
    _assertClass(doc, Document);
    const ptr0 = passStringToWasm0(ucl, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.executeUcl(doc.__wbg_ptr, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}
exports.executeUcl = executeUcl;

/**
 * Find a section by path (e.g., "Introduction > Getting Started").
 * @param {Document} doc
 * @param {string} path
 * @returns {string | undefined}
 */
function findSectionByPath(doc, path) {
    _assertClass(doc, Document);
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.findSectionByPath(doc.__wbg_ptr, ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getStringFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}
exports.findSectionByPath = findSectionByPath;

/**
 * Get all sections (heading blocks) in the document.
 * @param {Document} doc
 * @returns {any}
 */
function getAllSections(doc) {
    _assertClass(doc, Document);
    const ret = wasm.getAllSections(doc.__wbg_ptr);
    return ret;
}
exports.getAllSections = getAllSections;

/**
 * Get the depth of a section in the document hierarchy.
 * @param {Document} doc
 * @param {string} section_id
 * @returns {number | undefined}
 */
function getSectionDepth(doc, section_id) {
    _assertClass(doc, Document);
    const ptr0 = passStringToWasm0(section_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.getSectionDepth(doc.__wbg_ptr, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] === 0x100000001 ? undefined : ret[0];
}
exports.getSectionDepth = getSectionDepth;

/**
 * Initialize panic hook for better error messages in WASM.
 */
function init() {
    wasm.init();
}
exports.init = init;

/**
 * Parse HTML into a Document.
 * @param {string} html
 * @returns {Document}
 */
function parseHtml(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.parseHtml(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return Document.__wrap(ret[0]);
}
exports.parseHtml = parseHtml;

/**
 * Parse markdown into a Document.
 * @param {string} markdown
 * @returns {Document}
 */
function parseMarkdown(markdown) {
    const ptr0 = passStringToWasm0(markdown, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.parseMarkdown(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return Document.__wrap(ret[0]);
}
exports.parseMarkdown = parseMarkdown;

/**
 * Render a Document to markdown.
 * @param {Document} doc
 * @returns {string}
 */
function renderMarkdown(doc) {
    let deferred2_0;
    let deferred2_1;
    try {
        _assertClass(doc, Document);
        const ret = wasm.renderMarkdown(doc.__wbg_ptr);
        var ptr1 = ret[0];
        var len1 = ret[1];
        if (ret[3]) {
            ptr1 = 0; len1 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred2_0 = ptr1;
        deferred2_1 = len1;
        return getStringFromWasm0(ptr1, len1);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}
exports.renderMarkdown = renderMarkdown;

/**
 * Restore previously deleted section content.
 * @param {Document} doc
 * @param {WasmDeletedContent} deleted
 * @returns {Array<any>}
 */
function restoreDeletedSection(doc, deleted) {
    _assertClass(doc, Document);
    _assertClass(deleted, WasmDeletedContent);
    const ret = wasm.restoreDeletedSection(doc.__wbg_ptr, deleted.__wbg_ptr);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}
exports.restoreDeletedSection = restoreDeletedSection;

/**
 * Get the library version.
 * @returns {string}
 */
function version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}
exports.version = version;

/**
 * Write markdown content into a section, replacing its children.
 * @param {Document} doc
 * @param {string} section_id
 * @param {string} markdown
 * @param {number | null} [base_heading_level]
 * @returns {WasmWriteSectionResult}
 */
function writeSection(doc, section_id, markdown, base_heading_level) {
    _assertClass(doc, Document);
    const ptr0 = passStringToWasm0(section_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(markdown, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.writeSection(doc.__wbg_ptr, ptr0, len0, ptr1, len1, isLikeNone(base_heading_level) ? 0x100000001 : (base_heading_level) >>> 0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return WasmWriteSectionResult.__wrap(ret[0]);
}
exports.writeSection = writeSection;

exports.__wbg_Error_52673b7de5a0ca89 = function(arg0, arg1) {
    const ret = Error(getStringFromWasm0(arg0, arg1));
    return ret;
};

exports.__wbg_String_8f0eb39a4a4c2f66 = function(arg0, arg1) {
    const ret = String(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

exports.__wbg___wbindgen_bigint_get_as_i64_6e32f5e6aff02e1d = function(arg0, arg1) {
    const v = arg1;
    const ret = typeof(v) === 'bigint' ? v : undefined;
    getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};

exports.__wbg___wbindgen_boolean_get_dea25b33882b895b = function(arg0) {
    const v = arg0;
    const ret = typeof(v) === 'boolean' ? v : undefined;
    return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
};

exports.__wbg___wbindgen_debug_string_adfb662ae34724b6 = function(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

exports.__wbg___wbindgen_in_0d3e1e8f0c669317 = function(arg0, arg1) {
    const ret = arg0 in arg1;
    return ret;
};

exports.__wbg___wbindgen_is_bigint_0e1a2e3f55cfae27 = function(arg0) {
    const ret = typeof(arg0) === 'bigint';
    return ret;
};

exports.__wbg___wbindgen_is_function_8d400b8b1af978cd = function(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
};

exports.__wbg___wbindgen_is_object_ce774f3490692386 = function(arg0) {
    const val = arg0;
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};

exports.__wbg___wbindgen_is_string_704ef9c8fc131030 = function(arg0) {
    const ret = typeof(arg0) === 'string';
    return ret;
};

exports.__wbg___wbindgen_jsval_eq_b6101cc9cef1fe36 = function(arg0, arg1) {
    const ret = arg0 === arg1;
    return ret;
};

exports.__wbg___wbindgen_jsval_loose_eq_766057600fdd1b0d = function(arg0, arg1) {
    const ret = arg0 == arg1;
    return ret;
};

exports.__wbg___wbindgen_number_get_9619185a74197f95 = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};

exports.__wbg___wbindgen_string_get_a2a31e16edf96e42 = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

exports.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

exports.__wbg_call_abb4ff46ce38be40 = function() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

exports.__wbg_done_62ea16af4ce34b24 = function(arg0) {
    const ret = arg0.done;
    return ret;
};

exports.__wbg_entries_83c79938054e065f = function(arg0) {
    const ret = Object.entries(arg0);
    return ret;
};

exports.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

exports.__wbg_getRandomValues_1c61fac11405ffdc = function() { return handleError(function (arg0, arg1) {
    globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
}, arguments) };

exports.__wbg_getTime_ad1e9878a735af08 = function(arg0) {
    const ret = arg0.getTime();
    return ret;
};

exports.__wbg_get_6b7bd52aca3f9671 = function(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
};

exports.__wbg_get_af9dab7e9603ea93 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(arg0, arg1);
    return ret;
}, arguments) };

exports.__wbg_instanceof_ArrayBuffer_f3320d2419cd0355 = function(arg0) {
    let result;
    try {
        result = arg0 instanceof ArrayBuffer;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

exports.__wbg_instanceof_Map_084be8da74364158 = function(arg0) {
    let result;
    try {
        result = arg0 instanceof Map;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

exports.__wbg_instanceof_Uint8Array_da54ccc9d3e09434 = function(arg0) {
    let result;
    try {
        result = arg0 instanceof Uint8Array;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

exports.__wbg_isArray_51fd9e6422c0a395 = function(arg0) {
    const ret = Array.isArray(arg0);
    return ret;
};

exports.__wbg_isSafeInteger_ae7d3f054d55fa16 = function(arg0) {
    const ret = Number.isSafeInteger(arg0);
    return ret;
};

exports.__wbg_iterator_27b7c8b35ab3e86b = function() {
    const ret = Symbol.iterator;
    return ret;
};

exports.__wbg_length_22ac23eaec9d8053 = function(arg0) {
    const ret = arg0.length;
    return ret;
};

exports.__wbg_length_d45040a40c570362 = function(arg0) {
    const ret = arg0.length;
    return ret;
};

exports.__wbg_new_0_23cedd11d9b40c9d = function() {
    const ret = new Date();
    return ret;
};

exports.__wbg_new_1ba21ce319a06297 = function() {
    const ret = new Object();
    return ret;
};

exports.__wbg_new_25f239778d6112b9 = function() {
    const ret = new Array();
    return ret;
};

exports.__wbg_new_6421f6084cc5bc5a = function(arg0) {
    const ret = new Uint8Array(arg0);
    return ret;
};

exports.__wbg_new_8a6f238a6ece86ea = function() {
    const ret = new Error();
    return ret;
};

exports.__wbg_new_b546ae120718850e = function() {
    const ret = new Map();
    return ret;
};

exports.__wbg_new_from_slice_f9c22b9153b26992 = function(arg0, arg1) {
    const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
    return ret;
};

exports.__wbg_next_138a17bbf04e926c = function(arg0) {
    const ret = arg0.next;
    return ret;
};

exports.__wbg_next_3cfe5c0fe2a4cc53 = function() { return handleError(function (arg0) {
    const ret = arg0.next();
    return ret;
}, arguments) };

exports.__wbg_prototypesetcall_dfe9b766cdc1f1fd = function(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
};

exports.__wbg_push_7d9be8f38fc13975 = function(arg0, arg1) {
    const ret = arg0.push(arg1);
    return ret;
};

exports.__wbg_set_3f1d0b984ed272ed = function(arg0, arg1, arg2) {
    arg0[arg1] = arg2;
};

exports.__wbg_set_781438a03c0c3c81 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(arg0, arg1, arg2);
    return ret;
}, arguments) };

exports.__wbg_set_7df433eea03a5c14 = function(arg0, arg1, arg2) {
    arg0[arg1 >>> 0] = arg2;
};

exports.__wbg_set_efaaf145b9377369 = function(arg0, arg1, arg2) {
    const ret = arg0.set(arg1, arg2);
    return ret;
};

exports.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

exports.__wbg_value_57b7b035e117f7ee = function(arg0) {
    const ret = arg0.value;
    return ret;
};

exports.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

exports.__wbindgen_cast_4625c577ab2ec9ee = function(arg0) {
    // Cast intrinsic for `U64 -> Externref`.
    const ret = BigInt.asUintN(64, arg0);
    return ret;
};

exports.__wbindgen_cast_9ae0607507abb057 = function(arg0) {
    // Cast intrinsic for `I64 -> Externref`.
    const ret = arg0;
    return ret;
};

exports.__wbindgen_cast_d6cd19b81560fd6e = function(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return ret;
};

exports.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
};

const wasmPath = `${__dirname}/ucp_wasm_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = exports.__wasm = new WebAssembly.Instance(wasmModule, imports).exports;

wasm.__wbindgen_start();
