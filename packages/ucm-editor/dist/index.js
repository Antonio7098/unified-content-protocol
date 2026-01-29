var __defProp = Object.defineProperty;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => __defNormalProp(obj, typeof key !== "symbol" ? key + "" : key, value);
import { useRef, useCallback, useSyncExternalStore, useEffect, useState, useMemo } from "react";
import { jsxs, jsx, Fragment } from "react/jsx-runtime";
const ErrorCode = {
  // Block errors (E001-E009)
  BLOCK_NOT_FOUND: "UCM_E001",
  INVALID_BLOCK_CONTENT: "UCM_E002",
  BLOCK_TYPE_MISMATCH: "UCM_E003",
  // Selection errors (E010-E019)
  INVALID_SELECTION: "UCM_E010",
  SELECTION_TARGET_NOT_FOUND: "UCM_E011",
  // Edit errors (E020-E029)
  EDIT_FAILED: "UCM_E020",
  CONCURRENT_EDIT_CONFLICT: "UCM_E021",
  READONLY_BLOCK: "UCM_E023",
  // Drag/Drop errors (E030-E039)
  INVALID_DROP_TARGET: "UCM_E030",
  CYCLE_WOULD_BE_CREATED: "UCM_E031",
  // History errors (E040-E049)
  NO_UNDO_AVAILABLE: "UCM_E040",
  NO_REDO_AVAILABLE: "UCM_E041",
  // Diff errors (E050-E059)
  SNAPSHOT_NOT_FOUND: "UCM_E050",
  INCOMPATIBLE_SNAPSHOTS: "UCM_E051",
  // Graph errors (E060-E069)
  LAYOUT_FAILED: "UCM_E060",
  // Document errors (E070-E079)
  DOCUMENT_NOT_LOADED: "UCM_E070",
  DOCUMENT_TOO_LARGE: "UCM_E072",
  // Internal errors (E090-E099)
  INTERNAL_ERROR: "UCM_E090",
  INVALID_STATE: "UCM_E091",
  NOT_IMPLEMENTED: "UCM_E092"
};
class EditorError extends Error {
  constructor(options) {
    super(options.message);
    __publicField(this, "code");
    __publicField(this, "category");
    __publicField(this, "severity");
    __publicField(this, "data");
    __publicField(this, "suggestion");
    __publicField(this, "timestamp");
    this.name = "EditorError";
    this.code = options.code;
    this.category = options.category;
    this.severity = options.severity ?? "error";
    this.data = options.data ?? {};
    this.suggestion = options.suggestion;
    this.timestamp = /* @__PURE__ */ new Date();
    if (options.cause) {
      this.cause = options.cause;
    }
    const captureStackTrace = Error.captureStackTrace;
    if (captureStackTrace) {
      captureStackTrace(
        this,
        EditorError
      );
    }
  }
  /**
   * Create a JSON representation for logging/serialization.
   */
  toJSON() {
    return {
      name: this.name,
      code: this.code,
      message: this.message,
      category: this.category,
      severity: this.severity,
      data: this.data,
      suggestion: this.suggestion,
      timestamp: this.timestamp.toISOString(),
      stack: this.stack
    };
  }
  /**
   * Format for display to users.
   */
  toUserMessage() {
    const parts = [`[${this.code}] ${this.message}`];
    if (this.suggestion) {
      parts.push(`Suggestion: ${this.suggestion}`);
    }
    return parts.join("\n");
  }
}
const Errors = {
  // Block errors
  blockNotFound(blockId) {
    return new EditorError({
      code: ErrorCode.BLOCK_NOT_FOUND,
      message: `Block not found: ${blockId}`,
      category: "block",
      data: { blockId },
      suggestion: "Verify the block ID exists in the document"
    });
  },
  invalidBlockContent(blockId, reason) {
    return new EditorError({
      code: ErrorCode.INVALID_BLOCK_CONTENT,
      message: `Invalid content for block ${blockId}: ${reason}`,
      category: "block",
      data: { blockId },
      suggestion: "Check the content format matches the block type"
    });
  },
  blockTypeMismatch(blockId, expected, actual) {
    return new EditorError({
      code: ErrorCode.BLOCK_TYPE_MISMATCH,
      message: `Block ${blockId} type mismatch: expected ${expected}, got ${actual}`,
      category: "block",
      data: { blockId, expectedType: expected, actualType: actual },
      suggestion: "Use the correct editor for this block type"
    });
  },
  // Selection errors
  invalidSelection(reason) {
    return new EditorError({
      code: ErrorCode.INVALID_SELECTION,
      message: `Invalid selection: ${reason}`,
      category: "selection",
      suggestion: "Click on a block to select it"
    });
  },
  selectionTargetNotFound(targetId) {
    return new EditorError({
      code: ErrorCode.SELECTION_TARGET_NOT_FOUND,
      message: `Selection target not found: ${targetId}`,
      category: "selection",
      data: { targetId }
    });
  },
  // Edit errors
  editFailed(blockId, reason) {
    return new EditorError({
      code: ErrorCode.EDIT_FAILED,
      message: `Edit failed for block ${blockId}: ${reason}`,
      category: "edit",
      data: { blockId, reason }
    });
  },
  concurrentEditConflict(blockId, version) {
    return new EditorError({
      code: ErrorCode.CONCURRENT_EDIT_CONFLICT,
      message: `Concurrent edit conflict on block ${blockId}`,
      category: "edit",
      data: { blockId, conflictingVersion: version },
      suggestion: "Refresh the document and try again"
    });
  },
  readonlyBlock(blockId) {
    return new EditorError({
      code: ErrorCode.READONLY_BLOCK,
      message: `Block ${blockId} is read-only`,
      category: "edit",
      data: { blockId },
      severity: "warning"
    });
  },
  // Drag/Drop errors
  invalidDropTarget(sourceId, targetId, reason) {
    return new EditorError({
      code: ErrorCode.INVALID_DROP_TARGET,
      message: `Cannot drop ${sourceId} onto ${targetId}: ${reason}`,
      category: "drag",
      data: { sourceId, targetId, reason }
    });
  },
  cycleWouldBeCreated(sourceId, targetId) {
    return new EditorError({
      code: ErrorCode.CYCLE_WOULD_BE_CREATED,
      message: `Moving ${sourceId} to ${targetId} would create a cycle`,
      category: "drag",
      data: { sourceId, targetId },
      suggestion: "Choose a different drop target"
    });
  },
  // History errors
  noUndoAvailable() {
    return new EditorError({
      code: ErrorCode.NO_UNDO_AVAILABLE,
      message: "No undo action available",
      category: "history",
      severity: "info",
      data: { action: "undo" }
    });
  },
  noRedoAvailable() {
    return new EditorError({
      code: ErrorCode.NO_REDO_AVAILABLE,
      message: "No redo action available",
      category: "history",
      severity: "info",
      data: { action: "redo" }
    });
  },
  // Diff errors
  snapshotNotFound(snapshotId) {
    return new EditorError({
      code: ErrorCode.SNAPSHOT_NOT_FOUND,
      message: `Snapshot not found: ${snapshotId}`,
      category: "diff",
      data: { snapshotId1: snapshotId }
    });
  },
  incompatibleSnapshots(id1, id2, reason) {
    return new EditorError({
      code: ErrorCode.INCOMPATIBLE_SNAPSHOTS,
      message: `Snapshots ${id1} and ${id2} are incompatible: ${reason}`,
      category: "diff",
      data: { snapshotId1: id1, snapshotId2: id2, reason }
    });
  },
  // Graph errors
  layoutFailed(layout, reason) {
    return new EditorError({
      code: ErrorCode.LAYOUT_FAILED,
      message: `Layout computation failed (${layout}): ${reason}`,
      category: "graph",
      data: { layout, reason }
    });
  },
  // Document errors
  documentNotLoaded() {
    return new EditorError({
      code: ErrorCode.DOCUMENT_NOT_LOADED,
      message: "No document is currently loaded",
      category: "document",
      suggestion: "Load a document before performing this action"
    });
  },
  documentTooLarge(blockCount, maxBlocks) {
    return new EditorError({
      code: ErrorCode.DOCUMENT_TOO_LARGE,
      message: `Document has ${blockCount} blocks, maximum is ${maxBlocks}`,
      category: "document",
      data: { blockCount, maxBlocks },
      suggestion: "Split the document into smaller parts"
    });
  },
  // Internal errors
  internalError(message, component) {
    return new EditorError({
      code: ErrorCode.INTERNAL_ERROR,
      message: `Internal error: ${message}`,
      category: "internal",
      data: { component }
    });
  },
  invalidState(message) {
    return new EditorError({
      code: ErrorCode.INVALID_STATE,
      message: `Invalid state: ${message}`,
      category: "internal"
    });
  },
  notImplemented(feature) {
    return new EditorError({
      code: ErrorCode.NOT_IMPLEMENTED,
      message: `Not implemented: ${feature}`,
      category: "internal",
      severity: "warning"
    });
  }
};
function ok(value) {
  return { ok: true, value };
}
function err(error) {
  return { ok: false, error };
}
function unwrap(result) {
  if (result.ok) {
    return result.value;
  }
  throw result.error;
}
function unwrapOr(result, defaultValue) {
  return result.ok ? result.value : defaultValue;
}
function map(result, fn) {
  if (result.ok) {
    return ok(fn(result.value));
  }
  return result;
}
function andThen(result, fn) {
  if (result.ok) {
    return fn(result.value);
  }
  return result;
}
const DEFAULT_EDITOR_CONFIG = {
  virtualizationThreshold: 1e3,
  autoSaveDelay: 1e3,
  maxHistoryEntries: 100,
  enableKeyboardShortcuts: true,
  enableDragDrop: true,
  showBlockIds: false,
  defaultGraphLayout: "hierarchical",
  logLevel: "info"
};
class SimpleEventEmitter {
  constructor() {
    __publicField(this, "handlers", /* @__PURE__ */ new Map());
    __publicField(this, "onceHandlers", /* @__PURE__ */ new Map());
  }
  on(type, handler) {
    if (!this.handlers.has(type)) {
      this.handlers.set(type, /* @__PURE__ */ new Set());
    }
    this.handlers.get(type).add(handler);
    return () => this.off(type, handler);
  }
  once(type, handler) {
    if (!this.onceHandlers.has(type)) {
      this.onceHandlers.set(type, /* @__PURE__ */ new Set());
    }
    this.onceHandlers.get(type).add(handler);
    return () => {
      const handlers = this.onceHandlers.get(type);
      if (handlers) {
        handlers.delete(handler);
      }
    };
  }
  off(type, handler) {
    const handlers = this.handlers.get(type);
    if (handlers) {
      handlers.delete(handler);
    }
    const onceHandlers = this.onceHandlers.get(type);
    if (onceHandlers) {
      onceHandlers.delete(handler);
    }
  }
  emit(type, data) {
    const event = {
      type,
      timestamp: /* @__PURE__ */ new Date(),
      data
    };
    const handlers = this.handlers.get(type);
    if (handlers) {
      handlers.forEach((handler) => {
        try {
          handler(event);
        } catch (error) {
          console.error(`Error in event handler for ${type}:`, error);
        }
      });
    }
    const onceHandlers = this.onceHandlers.get(type);
    if (onceHandlers) {
      onceHandlers.forEach((handler) => {
        try {
          handler(event);
        } catch (error) {
          console.error(`Error in once handler for ${type}:`, error);
        }
      });
      this.onceHandlers.delete(type);
    }
  }
  clear() {
    this.handlers.clear();
    this.onceHandlers.clear();
  }
  /**
   * Get the number of handlers for a specific event type.
   */
  listenerCount(type) {
    var _a, _b;
    const handlers = ((_a = this.handlers.get(type)) == null ? void 0 : _a.size) ?? 0;
    const onceHandlers = ((_b = this.onceHandlers.get(type)) == null ? void 0 : _b.size) ?? 0;
    return handlers + onceHandlers;
  }
  /**
   * Get all registered event types.
   */
  eventTypes() {
    const types = /* @__PURE__ */ new Set();
    this.handlers.forEach((_, type) => types.add(type));
    this.onceHandlers.forEach((_, type) => types.add(type));
    return Array.from(types);
  }
}
const LOG_LEVEL_PRIORITY = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3
};
const LEVEL_COLORS = {
  debug: "\x1B[36m",
  // Cyan
  info: "\x1B[32m",
  // Green
  warn: "\x1B[33m",
  // Yellow
  error: "\x1B[31m"
  // Red
};
const RESET = "\x1B[0m";
function formatLogEntry(entry, useColors) {
  const time = entry.timestamp.toISOString().slice(11, 23);
  const level = entry.level.toUpperCase().padEnd(5);
  const context = entry.context ? `[${entry.context}]` : "";
  if (useColors) {
    const color = LEVEL_COLORS[entry.level];
    return `${color}${time} ${level}${RESET} ${context} ${entry.message}`;
  }
  return `${time} ${level} ${context} ${entry.message}`;
}
const consoleHandler = (entry) => {
  const formatted = formatLogEntry(entry, typeof window === "undefined");
  const args = [formatted];
  if (entry.data && Object.keys(entry.data).length > 0) {
    args.push(entry.data);
  }
  if (entry.error) {
    args.push(entry.error);
  }
  switch (entry.level) {
    case "debug":
      console.debug(...args);
      break;
    case "info":
      console.info(...args);
      break;
    case "warn":
      console.warn(...args);
      break;
    case "error":
      console.error(...args);
      break;
  }
};
function createBufferHandler(maxEntries = 1e3) {
  const entries = [];
  return {
    handler: (entry) => {
      entries.push(entry);
      if (entries.length > maxEntries) {
        entries.shift();
      }
    },
    getEntries: () => [...entries],
    clear: () => {
      entries.length = 0;
    }
  };
}
const DEFAULT_CONFIG = {
  level: "info",
  handlers: [consoleHandler]
};
class Logger {
  constructor(config = {}) {
    __publicField(this, "config");
    __publicField(this, "context");
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.context = config.context;
  }
  /**
   * Create a child logger with additional context.
   */
  child(context) {
    const childContext = this.context ? `${this.context}:${context}` : context;
    return new Logger({
      ...this.config,
      context: childContext
    });
  }
  /**
   * Set the log level.
   */
  setLevel(level) {
    this.config.level = level;
  }
  /**
   * Get the current log level.
   */
  getLevel() {
    return this.config.level;
  }
  /**
   * Add a log handler.
   */
  addHandler(handler) {
    this.config.handlers.push(handler);
  }
  /**
   * Remove a log handler.
   */
  removeHandler(handler) {
    const index = this.config.handlers.indexOf(handler);
    if (index !== -1) {
      this.config.handlers.splice(index, 1);
    }
  }
  /**
   * Clear all handlers.
   */
  clearHandlers() {
    this.config.handlers = [];
  }
  /**
   * Check if a level would be logged.
   */
  isLevelEnabled(level) {
    return LOG_LEVEL_PRIORITY[level] >= LOG_LEVEL_PRIORITY[this.config.level];
  }
  /**
   * Log a debug message.
   */
  debug(message, data) {
    this.log("debug", message, data);
  }
  /**
   * Log an info message.
   */
  info(message, data) {
    this.log("info", message, data);
  }
  /**
   * Log a warning message.
   */
  warn(message, data) {
    this.log("warn", message, data);
  }
  /**
   * Log an error message.
   */
  error(message, error, data) {
    if (error instanceof Error) {
      this.log("error", message, data, error);
    } else {
      this.log("error", message, error);
    }
  }
  /**
   * Log a message with timing.
   */
  time(label, fn) {
    const start = performance.now();
    try {
      const result = fn();
      const duration = performance.now() - start;
      this.debug(`${label} completed`, { durationMs: duration.toFixed(2) });
      return result;
    } catch (error) {
      const duration = performance.now() - start;
      this.error(`${label} failed`, error instanceof Error ? error : void 0, {
        durationMs: duration.toFixed(2)
      });
      throw error;
    }
  }
  /**
   * Log a message with async timing.
   */
  async timeAsync(label, fn) {
    const start = performance.now();
    try {
      const result = await fn();
      const duration = performance.now() - start;
      this.debug(`${label} completed`, { durationMs: duration.toFixed(2) });
      return result;
    } catch (error) {
      const duration = performance.now() - start;
      this.error(`${label} failed`, error instanceof Error ? error : void 0, {
        durationMs: duration.toFixed(2)
      });
      throw error;
    }
  }
  /**
   * Create a group of related log messages.
   */
  group(label) {
    this.debug(`${label} started`);
    const start = performance.now();
    return {
      end: () => {
        const duration = performance.now() - start;
        this.debug(`${label} ended`, { durationMs: duration.toFixed(2) });
      }
    };
  }
  log(level, message, data, error) {
    if (!this.isLevelEnabled(level)) {
      return;
    }
    const entry = {
      level,
      message,
      timestamp: /* @__PURE__ */ new Date(),
      context: this.context,
      data,
      error
    };
    for (const handler of this.config.handlers) {
      try {
        handler(entry);
      } catch (e) {
        console.error("Log handler error:", e);
      }
    }
  }
}
const logger$2 = new Logger({ context: "UCMEditor" });
function configureLogger(config) {
  if (config.level) {
    logger$2.setLevel(config.level);
  }
  if (config.handlers) {
    logger$2.clearHandlers();
    config.handlers.forEach((h) => logger$2.addHandler(h));
  }
}
let blockCounter = 0;
function generateId() {
  blockCounter++;
  return `blk_${blockCounter.toString(16).padStart(12, "0")}`;
}
function createDocument(title) {
  const rootId = generateId();
  const now = /* @__PURE__ */ new Date();
  const root = {
    id: rootId,
    content: "",
    type: "text",
    tags: [],
    children: [],
    edges: []
  };
  return {
    id: `doc_${Date.now().toString(16)}`,
    root: rootId,
    blocks: /* @__PURE__ */ new Map([[rootId, root]]),
    metadata: {
      title,
      authors: [],
      createdAt: now,
      modifiedAt: now,
      custom: {}
    },
    version: 1
  };
}
function addBlock(doc, parentId, content, options = {}) {
  const parent = doc.blocks.get(parentId);
  if (!parent)
    throw new Error(`Parent block not found: ${parentId}`);
  const id = generateId();
  const block = {
    id,
    content,
    type: options.type ?? "text",
    role: options.role,
    label: options.label,
    tags: [],
    children: [],
    edges: [],
    metadata: {
      tags: [],
      createdAt: /* @__PURE__ */ new Date(),
      modifiedAt: /* @__PURE__ */ new Date(),
      custom: {}
    }
  };
  if (options.role) {
    block.metadata.semanticRole = options.role;
  }
  doc.blocks.set(id, block);
  parent.children.push(id);
  return id;
}
function editBlock(doc, id, content) {
  const block = doc.blocks.get(id);
  if (!block)
    throw new Error(`Block not found: ${id}`);
  block.content = content;
}
function moveBlock(doc, id, newParentId, index) {
  if (id === doc.root)
    throw new Error("Cannot move the root block");
  const block = doc.blocks.get(id);
  if (!block)
    throw new Error(`Block not found: ${id}`);
  const newParent = doc.blocks.get(newParentId);
  if (!newParent)
    throw new Error(`Parent block not found: ${newParentId}`);
  if (newParentId === id || isDescendant(doc, id, newParentId)) {
    throw new Error("Cannot move a block into itself or its descendants");
  }
  const oldParentId = findParent$1(doc, id);
  if (!oldParentId)
    throw new Error(`Parent block not found for: ${id}`);
  const oldParent = doc.blocks.get(oldParentId);
  const childIndex = oldParent.children.indexOf(id);
  if (childIndex === -1)
    throw new Error(`Block ${id} not linked to parent ${oldParentId}`);
  oldParent.children.splice(childIndex, 1);
  if (index === void 0 || index < 0 || index > newParent.children.length) {
    newParent.children.push(id);
  } else {
    newParent.children.splice(index, 0, id);
  }
}
function deleteBlock(doc, id, options = {}) {
  if (id === doc.root)
    throw new Error("Cannot delete the root block");
  const block = doc.blocks.get(id);
  if (!block)
    throw new Error(`Block not found: ${id}`);
  const parentId = findParent$1(doc, id);
  if (!parentId)
    throw new Error(`Parent block not found for: ${id}`);
  const parent = doc.blocks.get(parentId);
  const idx = parent.children.indexOf(id);
  if (idx === -1)
    throw new Error(`Block ${id} not linked to parent ${parentId}`);
  parent.children.splice(idx, 1);
  if (options.cascade ?? true) {
    for (const childId of block.children) {
      deleteSubtree(doc, childId);
    }
  } else {
    parent.children.splice(idx, 0, ...block.children);
  }
  doc.blocks.delete(id);
}
function findParent$1(doc, id) {
  for (const [candidateId, block] of doc.blocks.entries()) {
    if (block.children.includes(id)) {
      return candidateId;
    }
  }
  return void 0;
}
function isDescendant(doc, ancestorId, candidateId) {
  const ancestor = doc.blocks.get(ancestorId);
  if (!ancestor)
    return false;
  const queue = [...ancestor.children];
  while (queue.length > 0) {
    const current = queue.shift();
    if (current === candidateId)
      return true;
    const block = doc.blocks.get(current);
    if (block)
      queue.push(...block.children);
  }
  return false;
}
function deleteSubtree(doc, id) {
  const block = doc.blocks.get(id);
  if (!block)
    return;
  for (const child of block.children) {
    deleteSubtree(doc, child);
  }
  doc.blocks.delete(id);
}
function createEdge(edgeType, target, metadata = {}) {
  return {
    edgeType,
    target,
    metadata,
    createdAt: /* @__PURE__ */ new Date()
  };
}
function addEdge(doc, sourceId, edgeType, targetId, metadata = {}) {
  const source = doc.blocks.get(sourceId);
  if (!source)
    throw new Error(`Source block not found: ${sourceId}`);
  if (!doc.blocks.has(targetId))
    throw new Error(`Target block not found: ${targetId}`);
  const edge = createEdge(edgeType, targetId, metadata);
  source.edges.push(edge);
  touchDocument(doc);
}
function removeEdge(doc, sourceId, edgeType, targetId) {
  const source = doc.blocks.get(sourceId);
  if (!source)
    throw new Error(`Source block not found: ${sourceId}`);
  const initialLength = source.edges.length;
  source.edges = source.edges.filter((e) => !(e.edgeType === edgeType && e.target === targetId));
  const removed = source.edges.length < initialLength;
  if (removed)
    touchDocument(doc);
  return removed;
}
function touchDocument(doc) {
  if (doc.metadata) {
    doc.metadata.modifiedAt = /* @__PURE__ */ new Date();
  }
  doc.version++;
}
const DEFAULT_LIMITS = {
  maxDocumentSize: 50 * 1024 * 1024,
  // 50MB
  maxBlockCount: 1e5,
  maxBlockSize: 5 * 1024 * 1024,
  // 5MB
  maxNestingDepth: 50,
  maxEdgesPerBlock: 1e3
};
function validateDocument(doc, limits = DEFAULT_LIMITS) {
  const issues = [];
  if (doc.blocks.size > limits.maxBlockCount) {
    issues.push({
      severity: "error",
      code: "E400",
      message: `Document has ${doc.blocks.size} blocks, max is ${limits.maxBlockCount}`
    });
  }
  if (hasCycles(doc)) {
    issues.push({
      severity: "error",
      code: "E201",
      message: "Document structure contains a cycle"
    });
  }
  const maxDepth = computeMaxDepth(doc);
  if (maxDepth > limits.maxNestingDepth) {
    issues.push({
      severity: "error",
      code: "E403",
      message: `Max nesting depth is ${limits.maxNestingDepth}, document has ${maxDepth}`
    });
  }
  for (const [id, block] of doc.blocks.entries()) {
    const blockIssues = validateBlock(doc, block, limits);
    issues.push(...blockIssues);
  }
  const orphans = findOrphans(doc);
  for (const orphan of orphans) {
    issues.push({
      severity: "warning",
      code: "E203",
      message: `Block ${orphan} is unreachable from root`,
      blockId: orphan
    });
  }
  const hasErrors = issues.some((i) => i.severity === "error");
  return { valid: !hasErrors, issues };
}
function validateBlock(doc, block, limits) {
  const issues = [];
  const size = new TextEncoder().encode(block.content).length;
  if (size > limits.maxBlockSize) {
    issues.push({
      severity: "error",
      code: "E402",
      message: `Block ${block.id} has size ${size}, max is ${limits.maxBlockSize}`,
      blockId: block.id
    });
  }
  if (block.edges.length > limits.maxEdgesPerBlock) {
    issues.push({
      severity: "error",
      code: "E404",
      message: `Block ${block.id} has ${block.edges.length} edges, max is ${limits.maxEdgesPerBlock}`,
      blockId: block.id
    });
  }
  for (const edge of block.edges) {
    if (!doc.blocks.has(edge.target)) {
      issues.push({
        severity: "error",
        code: "E001",
        message: `Block ${block.id} has edge to non-existent block ${edge.target}`,
        blockId: block.id
      });
    }
  }
  return issues;
}
function hasCycles(doc) {
  const visited = /* @__PURE__ */ new Set();
  const recStack = /* @__PURE__ */ new Set();
  function dfs(nodeId) {
    visited.add(nodeId);
    recStack.add(nodeId);
    const node = doc.blocks.get(nodeId);
    if (node) {
      for (const child of node.children) {
        if (!visited.has(child)) {
          if (dfs(child))
            return true;
        } else if (recStack.has(child)) {
          return true;
        }
      }
    }
    recStack.delete(nodeId);
    return false;
  }
  return dfs(doc.root);
}
function computeMaxDepth(doc) {
  function depthFrom(nodeId, current) {
    const node = doc.blocks.get(nodeId);
    if (!node || node.children.length === 0)
      return current;
    return Math.max(...node.children.map((c) => depthFrom(c, current + 1)));
  }
  return depthFrom(doc.root, 1);
}
function findOrphans(doc) {
  const reachable = /* @__PURE__ */ new Set([doc.root]);
  const queue = [doc.root];
  while (queue.length > 0) {
    const current = queue.shift();
    const block = doc.blocks.get(current);
    if (block) {
      for (const child of block.children) {
        if (!reachable.has(child)) {
          reachable.add(child);
          queue.push(child);
        }
      }
    }
  }
  return Array.from(doc.blocks.keys()).filter((id) => !reachable.has(id));
}
function serializeDocument(doc) {
  const serializable = {
    id: doc.id,
    root: doc.root,
    blocks: Array.from(doc.blocks.entries()).map(([id, block]) => [id, {
      ...block,
      edges: block.edges.map((e) => ({ ...e, createdAt: e.createdAt.toISOString() }))
    }]),
    metadata: doc.metadata ? {
      ...doc.metadata,
      createdAt: doc.metadata.createdAt.toISOString(),
      modifiedAt: doc.metadata.modifiedAt.toISOString()
    } : void 0,
    version: doc.version
  };
  return JSON.stringify(serializable);
}
function deserializeDocument(data) {
  const parsed = JSON.parse(data);
  const blocks = /* @__PURE__ */ new Map();
  for (const [id, blockData] of parsed.blocks) {
    blocks.set(id, {
      ...blockData,
      edges: blockData.edges.map((e) => ({
        ...e,
        createdAt: new Date(e.createdAt)
      }))
    });
  }
  return {
    id: parsed.id,
    root: parsed.root,
    blocks,
    metadata: parsed.metadata ? {
      ...parsed.metadata,
      createdAt: new Date(parsed.metadata.createdAt),
      modifiedAt: new Date(parsed.metadata.modifiedAt)
    } : void 0,
    version: parsed.version
  };
}
class SnapshotManager {
  constructor(maxSnapshots = 100) {
    __publicField(this, "snapshots", /* @__PURE__ */ new Map());
    __publicField(this, "maxSnapshots");
    this.maxSnapshots = maxSnapshots;
  }
  /** Create a snapshot of the document */
  create(name, doc, description) {
    if (this.snapshots.size >= this.maxSnapshots) {
      this.evictOldest();
    }
    const snapshot = {
      id: name,
      description,
      createdAt: /* @__PURE__ */ new Date(),
      documentVersion: doc.version,
      data: serializeDocument(doc)
    };
    this.snapshots.set(name, snapshot);
    return name;
  }
  /** Restore a document from a snapshot */
  restore(name) {
    const snapshot = this.snapshots.get(name);
    if (!snapshot)
      throw new Error(`Snapshot not found: ${name}`);
    return deserializeDocument(snapshot.data);
  }
  /** Get a snapshot by name */
  get(name) {
    return this.snapshots.get(name);
  }
  /** Get snapshot info without loading full data */
  getInfo(name) {
    const snapshot = this.snapshots.get(name);
    if (!snapshot)
      return void 0;
    let blockCount = 0;
    try {
      const parsed = JSON.parse(snapshot.data);
      blockCount = Object.keys(parsed.blocks ?? {}).length;
    } catch {
      blockCount = 0;
    }
    return {
      id: snapshot.id,
      description: snapshot.description,
      createdAt: snapshot.createdAt,
      documentVersion: snapshot.documentVersion,
      blockCount
    };
  }
  /** List all snapshots (newest first) */
  list() {
    return Array.from(this.snapshots.values()).sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime());
  }
  /** Delete a snapshot */
  delete(name) {
    return this.snapshots.delete(name);
  }
  /** Check if a snapshot exists */
  exists(name) {
    return this.snapshots.has(name);
  }
  /** Get snapshot count */
  count() {
    return this.snapshots.size;
  }
  evictOldest() {
    let oldest;
    for (const snapshot of this.snapshots.values()) {
      if (!oldest || snapshot.createdAt < oldest.createdAt) {
        oldest = snapshot;
      }
    }
    if (oldest)
      this.snapshots.delete(oldest.id);
  }
}
function createInitialSelection() {
  return {
    type: "none"
  };
}
function createInitialDragState() {
  return {
    isDragging: false
  };
}
function createInitialGraphState() {
  return {
    layout: "hierarchical",
    nodes: /* @__PURE__ */ new Map(),
    edges: [],
    viewport: { x: 0, y: 0, zoom: 1 },
    selectedNodeId: void 0,
    hoveredNodeId: void 0,
    showEdgeLabels: true,
    edgeFilter: []
  };
}
function createInitialDiffState() {
  return {
    isComparing: false,
    leftSnapshotId: void 0,
    rightSnapshotId: void 0,
    diff: void 0,
    selectedChangeId: void 0,
    showUnchanged: false,
    viewMode: "unified"
  };
}
function createInitialHistoryState(maxEntries) {
  return {
    entries: [],
    currentIndex: -1,
    maxEntries,
    canUndo: false,
    canRedo: false
  };
}
function createInitialState(config) {
  return {
    document: null,
    documentId: null,
    isLoading: false,
    isDirty: false,
    lastSaved: void 0,
    view: "document",
    mode: "view",
    selection: createInitialSelection(),
    editingBlockId: null,
    editState: "idle",
    pendingContent: null,
    drag: createInitialDragState(),
    graph: createInitialGraphState(),
    diff: createInitialDiffState(),
    history: createInitialHistoryState(config.maxHistoryEntries),
    config,
    lastError: null
  };
}
function createEditorStore(initialConfig = {}) {
  const config = { ...DEFAULT_EDITOR_CONFIG, ...initialConfig };
  const logger2 = new Logger({ context: "EditorStore", level: config.logLevel });
  const events = new SimpleEventEmitter();
  const snapshotManager = new SnapshotManager(config.maxHistoryEntries);
  let state = createInitialState(config);
  const listeners = /* @__PURE__ */ new Set();
  function setState(updater) {
    const prevState = state;
    const updates = typeof updater === "function" ? updater(state) : updater;
    state = { ...state, ...updates };
    listeners.forEach((listener) => {
      try {
        listener(state, prevState);
      } catch (error) {
        logger2.error("Listener error", error instanceof Error ? error : void 0);
      }
    });
  }
  function emitEvent(type, data) {
    events.emit(type, data);
  }
  function handleError(error) {
    logger2.error(error.message, error);
    setState({ lastError: error });
    if (error instanceof EditorError) {
      emitEvent("error:occurred", {
        code: error.code,
        message: error.message,
        category: error.category,
        severity: error.severity,
        data: error.data
      });
    }
  }
  function pushHistory(description, operations) {
    if (!state.document) return;
    const snapshotId = `snapshot_${Date.now()}`;
    snapshotManager.create(snapshotId, state.document, description);
    const entry = {
      id: `entry_${Date.now()}`,
      timestamp: /* @__PURE__ */ new Date(),
      description,
      snapshotId,
      operations
    };
    const newEntries = [...state.history.entries.slice(0, state.history.currentIndex + 1), entry];
    if (newEntries.length > state.history.maxEntries) {
      newEntries.shift();
    }
    setState({
      history: {
        ...state.history,
        entries: newEntries,
        currentIndex: newEntries.length - 1,
        canUndo: newEntries.length > 0,
        canRedo: false
      },
      isDirty: true
    });
    emitEvent("history:snapshot_created", {
      entryId: entry.id,
      description,
      snapshotId,
      operationCount: operations.length
    });
  }
  function loadDocument(doc) {
    var _a;
    logger2.info("Loading document", { documentId: doc.id, blockCount: doc.blocks.size });
    setState({
      document: doc,
      documentId: doc.id,
      isLoading: false,
      isDirty: false,
      selection: createInitialSelection(),
      editingBlockId: null,
      editState: "idle",
      pendingContent: null,
      history: createInitialHistoryState(state.config.maxHistoryEntries),
      lastError: null
    });
    snapshotManager.create("initial", doc, "Initial state");
    emitEvent("document:loaded", {
      documentId: doc.id,
      title: (_a = doc.metadata) == null ? void 0 : _a.title,
      blockCount: doc.blocks.size,
      version: doc.version
    });
  }
  function createDocument$1(title) {
    logger2.info("Creating new document", { title });
    const doc = createDocument(title);
    loadDocument(doc);
    emitEvent("document:created", {
      documentId: doc.id,
      title
    });
  }
  async function saveDocument() {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    logger2.info("Saving document", { documentId: state.documentId });
    setState({ isLoading: true });
    try {
      const validation = validateDocument(state.document);
      if (!validation.valid) {
        const errors = validation.issues.filter((i) => i.severity === "error");
        if (errors.length > 0) {
          throw new EditorError({
            code: "UCM_E071",
            message: "Document validation failed",
            category: "document",
            data: { validationErrors: errors.map((e) => e.message) }
          });
        }
      }
      setState({
        isLoading: false,
        isDirty: false,
        lastSaved: /* @__PURE__ */ new Date()
      });
      emitEvent("document:saved", {
        documentId: state.documentId,
        version: state.document.version
      });
    } catch (error) {
      setState({ isLoading: false });
      handleError(error instanceof Error ? error : Errors.internalError(String(error)));
      throw error;
    }
  }
  function setView(view) {
    const previousView = state.view;
    logger2.debug("Setting view", { view, previousView });
    setState({ view });
    emitEvent("view:changed", { view, previousView });
  }
  function setMode(mode) {
    const previousMode = state.mode;
    logger2.debug("Setting mode", { mode, previousMode });
    setState({ mode });
    emitEvent("mode:changed", { mode, previousMode });
  }
  function addBlock$1(parentId, content, type) {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    logger2.debug("Adding block", { parentId, contentLength: content.length, type });
    const id = addBlock(state.document, parentId, content, { type });
    pushHistory(`Add block`, [{ type: "add_block", blockId: id, data: { parentId, content, type } }]);
    emitEvent("block:added", { blockId: id, parentId, content, type });
    setState({ document: { ...state.document } });
    return id;
  }
  function editBlockContent(blockId, content) {
    if (!state.document) {
      const error = Errors.documentNotLoaded();
      handleError(error);
      throw error;
    }
    const block = state.document.blocks.get(blockId);
    if (!block) {
      const error = Errors.blockNotFound(blockId);
      handleError(error);
      throw error;
    }
    const oldContent = block.content;
    logger2.debug("Editing block", { blockId, oldLength: oldContent.length, newLength: content.length });
    editBlock(state.document, blockId, content);
    pushHistory(`Edit block`, [{ type: "edit_block", blockId, data: { oldContent, newContent: content } }]);
    emitEvent("block:edited", { blockId, content, oldContent });
    setState({ document: { ...state.document } });
  }
  function deleteBlockById(blockId, cascade = true) {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    const block = state.document.blocks.get(blockId);
    if (!block) {
      throw Errors.blockNotFound(blockId);
    }
    logger2.debug("Deleting block", { blockId, cascade });
    const blockData = serializeDocument(state.document);
    deleteBlock(state.document, blockId, { cascade });
    pushHistory(`Delete block`, [{ type: "delete_block", blockId, data: { cascade, snapshot: blockData } }]);
    if (state.selection.focusedBlockId === blockId) {
      setState({
        selection: createInitialSelection(),
        document: { ...state.document }
      });
    } else {
      setState({ document: { ...state.document } });
    }
    emitEvent("block:deleted", { blockId });
  }
  function moveBlockTo(blockId, targetId, position) {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    const block = state.document.blocks.get(blockId);
    if (!block) {
      throw Errors.blockNotFound(blockId);
    }
    const target = state.document.blocks.get(targetId);
    if (!target) {
      throw Errors.blockNotFound(targetId);
    }
    logger2.debug("Moving block", { blockId, targetId, position });
    let oldParentId;
    for (const [id, b] of state.document.blocks) {
      if (b.children.includes(blockId)) {
        oldParentId = id;
        break;
      }
    }
    let newParentId;
    let index;
    if (position === "inside") {
      newParentId = targetId;
    } else {
      for (const [id, b] of state.document.blocks) {
        if (b.children.includes(targetId)) {
          newParentId = id;
          const targetIndex = b.children.indexOf(targetId);
          index = position === "before" ? targetIndex : targetIndex + 1;
          break;
        }
      }
      if (!newParentId) {
        throw Errors.invalidDropTarget(blockId, targetId, "Target has no parent");
      }
    }
    moveBlock(state.document, blockId, newParentId, index);
    pushHistory(`Move block`, [
      {
        type: "move_block",
        blockId,
        data: { oldParentId, newParentId, position }
      }
    ]);
    emitEvent("block:moved", { blockId, oldParentId, newParentId, position });
    setState({ document: { ...state.document } });
  }
  function changeBlockType(blockId, type) {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    const block = state.document.blocks.get(blockId);
    if (!block) {
      throw Errors.blockNotFound(blockId);
    }
    const oldType = block.type;
    logger2.debug("Changing block type", { blockId, oldType, newType: type });
    block.type = type;
    pushHistory(`Change block type`, [
      { type: "change_type", blockId, data: { oldType, newType: type } }
    ]);
    emitEvent("block:type_changed", { blockId, type, oldType });
    setState({ document: { ...state.document } });
  }
  function select(blockId) {
    if (!state.document) return;
    const block = state.document.blocks.get(blockId);
    if (!block) {
      logger2.warn("Attempted to select non-existent block", { blockId });
      return;
    }
    logger2.debug("Selecting block", { blockId });
    setState({
      selection: {
        type: "block",
        blocks: { blockIds: [blockId], anchor: blockId, focus: blockId },
        focusedBlockId: blockId
      }
    });
    emitEvent("selection:changed", { blockIds: [blockId], focusedBlockId: blockId });
  }
  function selectMultiple(blockIds) {
    if (!state.document) return;
    const validIds = blockIds.filter((id) => state.document.blocks.has(id));
    if (validIds.length === 0) {
      clearSelection();
      return;
    }
    logger2.debug("Selecting multiple blocks", { count: validIds.length });
    setState({
      selection: {
        type: "block",
        blocks: {
          blockIds: validIds,
          anchor: validIds[0],
          focus: validIds[validIds.length - 1]
        },
        focusedBlockId: validIds[0]
      }
    });
    emitEvent("selection:changed", { blockIds: validIds, focusedBlockId: validIds[0] });
  }
  function clearSelection() {
    logger2.debug("Clearing selection");
    setState({ selection: createInitialSelection() });
    emitEvent("selection:cleared", { blockIds: [] });
  }
  function selectText(blockId, start, end) {
    if (!state.document) return;
    const block = state.document.blocks.get(blockId);
    if (!block) return;
    logger2.debug("Selecting text", { blockId, start, end });
    setState({
      selection: {
        type: "text",
        text: { blockId, start, end },
        focusedBlockId: blockId
      }
    });
    emitEvent("selection:changed", {
      blockIds: [blockId],
      focusedBlockId: blockId,
      textSelection: { blockId, start, end }
    });
  }
  function startEditing(blockId) {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    const block = state.document.blocks.get(blockId);
    if (!block) {
      throw Errors.blockNotFound(blockId);
    }
    logger2.debug("Starting edit", { blockId });
    setState({
      editingBlockId: blockId,
      editState: "editing",
      pendingContent: block.content,
      mode: "edit"
    });
    emitEvent("edit:started", { blockId, content: block.content });
  }
  function stopEditing(save = true) {
    if (!state.editingBlockId || !state.document) {
      return;
    }
    const blockId = state.editingBlockId;
    const content = state.pendingContent;
    logger2.debug("Stopping edit", { blockId, save });
    if (save && content !== null) {
      const block = state.document.blocks.get(blockId);
      if (block && block.content !== content) {
        editBlockContent(blockId, content);
        emitEvent("edit:saved", { blockId, content });
      }
    } else {
      emitEvent("edit:cancelled", { blockId });
    }
    setState({
      editingBlockId: null,
      editState: "idle",
      pendingContent: null,
      mode: "view"
    });
  }
  function updatePendingContent(content) {
    if (state.editingBlockId) {
      setState({ pendingContent: content });
    }
  }
  function startDrag(blockId) {
    if (!state.document) return;
    const block = state.document.blocks.get(blockId);
    if (!block) return;
    logger2.debug("Starting drag", { blockId });
    setState({
      drag: {
        isDragging: true,
        sourceId: blockId,
        targetId: void 0,
        position: void 0
      },
      mode: "drag"
    });
    emitEvent("drag:started", { sourceId: blockId });
  }
  function updateDragTarget(targetId, position) {
    if (!state.drag.isDragging) return;
    setState({
      drag: {
        ...state.drag,
        targetId,
        position
      }
    });
    emitEvent("drag:moved", { sourceId: state.drag.sourceId, targetId, position });
  }
  function endDrag(drop = true) {
    if (!state.drag.isDragging) return;
    const { sourceId, targetId, position } = state.drag;
    logger2.debug("Ending drag", { sourceId, targetId, position, drop });
    if (drop && sourceId && targetId && position) {
      try {
        moveBlockTo(sourceId, targetId, position);
        emitEvent("drag:ended", { sourceId, targetId, position });
      } catch (error) {
        handleError(error instanceof Error ? error : Errors.internalError(String(error)));
        emitEvent("drag:cancelled", { sourceId });
      }
    } else {
      emitEvent("drag:cancelled", { sourceId });
    }
    setState({
      drag: createInitialDragState(),
      mode: "view"
    });
  }
  function undo() {
    if (!state.history.canUndo || state.history.currentIndex < 0) {
      throw Errors.noUndoAvailable();
    }
    const entry = state.history.entries[state.history.currentIndex];
    if (!entry) return;
    logger2.debug("Undoing", { entryId: entry.id, description: entry.description });
    const previousIndex = state.history.currentIndex - 1;
    const previousEntry = state.history.entries[previousIndex];
    if (previousEntry) {
      const doc = snapshotManager.restore(previousEntry.snapshotId);
      setState({
        document: doc,
        history: {
          ...state.history,
          currentIndex: previousIndex,
          canUndo: previousIndex >= 0,
          canRedo: true
        },
        isDirty: true
      });
    } else {
      const doc = snapshotManager.restore("initial");
      setState({
        document: doc,
        history: {
          ...state.history,
          currentIndex: -1,
          canUndo: false,
          canRedo: true
        },
        isDirty: true
      });
    }
    emitEvent("history:undo", {
      entryId: entry.id,
      description: entry.description,
      snapshotId: entry.snapshotId
    });
  }
  function redo() {
    if (!state.history.canRedo) {
      throw Errors.noRedoAvailable();
    }
    const nextIndex = state.history.currentIndex + 1;
    const entry = state.history.entries[nextIndex];
    if (!entry) return;
    logger2.debug("Redoing", { entryId: entry.id, description: entry.description });
    const doc = snapshotManager.restore(entry.snapshotId);
    setState({
      document: doc,
      history: {
        ...state.history,
        currentIndex: nextIndex,
        canUndo: true,
        canRedo: nextIndex < state.history.entries.length - 1
      },
      isDirty: true
    });
    emitEvent("history:redo", {
      entryId: entry.id,
      description: entry.description,
      snapshotId: entry.snapshotId
    });
  }
  function createSnapshot(description) {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    const snapshotId = `snapshot_${Date.now()}`;
    snapshotManager.create(snapshotId, state.document, description);
    logger2.debug("Created snapshot", { snapshotId, description });
    emitEvent("history:snapshot_created", {
      entryId: snapshotId,
      description,
      snapshotId
    });
  }
  function setGraphLayout(layout) {
    logger2.debug("Setting graph layout", { layout });
    setState({
      graph: {
        ...state.graph,
        layout
      }
    });
    emitEvent("graph:layout_changed", { layout });
  }
  function setGraphViewport(x, y, zoom) {
    setState({
      graph: {
        ...state.graph,
        viewport: { x, y, zoom }
      }
    });
    emitEvent("graph:viewport_changed", { viewport: { x, y, zoom } });
  }
  function toggleNodeExpansion(nodeId) {
    const node = state.graph.nodes.get(nodeId);
    if (!node) return;
    const nodes = new Map(state.graph.nodes);
    nodes.set(nodeId, { ...node, isExpanded: !node.isExpanded });
    setState({
      graph: {
        ...state.graph,
        nodes
      }
    });
    emitEvent(node.isExpanded ? "graph:node_collapsed" : "graph:node_expanded", { nodeId });
  }
  function startCompare(leftSnapshotId, rightSnapshotId) {
    logger2.debug("Starting diff comparison", { leftSnapshotId, rightSnapshotId });
    setState({
      diff: {
        ...state.diff,
        isComparing: true,
        leftSnapshotId,
        rightSnapshotId
      }
    });
    emitEvent("diff:started", { leftSnapshotId, rightSnapshotId });
  }
  function stopCompare() {
    logger2.debug("Stopping diff comparison");
    setState({
      diff: createInitialDiffState()
    });
    emitEvent("diff:ended", {});
  }
  function applyChange(blockId) {
    logger2.debug("Applying diff change", { blockId });
    emitEvent("diff:change_applied", { blockId, changeType: "modified" });
  }
  function rejectChange(blockId) {
    logger2.debug("Rejecting diff change", { blockId });
    emitEvent("diff:change_rejected", { blockId, changeType: "modified" });
  }
  function addEdgeToBlock(sourceId, targetId, edgeType) {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    logger2.debug("Adding edge", { sourceId, targetId, edgeType });
    addEdge(state.document, sourceId, edgeType, targetId);
    pushHistory(`Add edge`, [{ type: "add_edge", data: { sourceId, targetId, edgeType } }]);
    emitEvent("edge:added", { sourceId, targetId, edgeType });
    setState({ document: { ...state.document } });
  }
  function removeEdgeFromBlock(sourceId, targetId, edgeType) {
    if (!state.document) {
      throw Errors.documentNotLoaded();
    }
    logger2.debug("Removing edge", { sourceId, targetId, edgeType });
    removeEdge(state.document, sourceId, edgeType, targetId);
    pushHistory(`Remove edge`, [{ type: "remove_edge", data: { sourceId, targetId, edgeType } }]);
    emitEvent("edge:removed", { sourceId, targetId, edgeType });
    setState({ document: { ...state.document } });
  }
  function updateConfig(configUpdate) {
    logger2.debug("Updating config", configUpdate);
    setState({
      config: { ...state.config, ...configUpdate }
    });
  }
  function clearError() {
    setState({ lastError: null });
    emitEvent("error:cleared", { code: "", message: "", category: "", severity: "" });
  }
  return {
    // State (getters)
    get document() {
      return state.document;
    },
    get documentId() {
      return state.documentId;
    },
    get isLoading() {
      return state.isLoading;
    },
    get isDirty() {
      return state.isDirty;
    },
    get lastSaved() {
      return state.lastSaved;
    },
    get view() {
      return state.view;
    },
    get mode() {
      return state.mode;
    },
    get selection() {
      return state.selection;
    },
    get editingBlockId() {
      return state.editingBlockId;
    },
    get editState() {
      return state.editState;
    },
    get pendingContent() {
      return state.pendingContent;
    },
    get drag() {
      return state.drag;
    },
    get graph() {
      return state.graph;
    },
    get diff() {
      return state.diff;
    },
    get history() {
      return state.history;
    },
    get config() {
      return state.config;
    },
    get lastError() {
      return state.lastError;
    },
    // Actions
    loadDocument,
    createDocument: createDocument$1,
    saveDocument,
    setView,
    setMode,
    addBlock: addBlock$1,
    editBlock: editBlockContent,
    deleteBlock: deleteBlockById,
    moveBlock: moveBlockTo,
    changeBlockType,
    select,
    selectMultiple,
    clearSelection,
    selectText,
    startEditing,
    stopEditing,
    updatePendingContent,
    startDrag,
    updateDragTarget,
    endDrag,
    undo,
    redo,
    createSnapshot,
    setGraphLayout,
    setGraphViewport,
    toggleNodeExpansion,
    startCompare,
    stopCompare,
    applyChange,
    rejectChange,
    addEdge: addEdgeToBlock,
    removeEdge: removeEdgeFromBlock,
    updateConfig,
    clearError,
    // Store utilities
    subscribe: (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    getState: () => state,
    events
  };
}
const logger$1 = new Logger({ context: "SelectionManager" });
function createEmptySelection() {
  return {
    type: "none",
    blocks: void 0,
    text: void 0,
    focusedBlockId: void 0
  };
}
function createBlockSelection(blockId) {
  return {
    type: "block",
    blocks: {
      blockIds: [blockId],
      anchor: blockId,
      focus: blockId
    },
    focusedBlockId: blockId
  };
}
function createMultiBlockSelection(blockIds, anchor, focus) {
  if (blockIds.length === 0) {
    return createEmptySelection();
  }
  return {
    type: "block",
    blocks: {
      blockIds,
      anchor: anchor ?? blockIds[0],
      focus: focus ?? blockIds[blockIds.length - 1]
    },
    focusedBlockId: focus ?? blockIds[blockIds.length - 1]
  };
}
function createTextSelection(blockId, start, end) {
  return {
    type: "text",
    text: {
      blockId,
      start: Math.min(start, end),
      end: Math.max(start, end)
    },
    focusedBlockId: blockId
  };
}
function isBlockSelected(selection, blockId) {
  var _a, _b;
  if (selection.type === "none") return false;
  if (selection.type === "text") return ((_a = selection.text) == null ? void 0 : _a.blockId) === blockId;
  if (selection.type === "block") return ((_b = selection.blocks) == null ? void 0 : _b.blockIds.includes(blockId)) ?? false;
  return false;
}
function isBlockFocused(selection, blockId) {
  return selection.focusedBlockId === blockId;
}
function getSelectedBlockIds(selection) {
  var _a;
  if (selection.type === "none") return [];
  if (selection.type === "text") return selection.text ? [selection.text.blockId] : [];
  if (selection.type === "block") return ((_a = selection.blocks) == null ? void 0 : _a.blockIds) ?? [];
  return [];
}
function getPrimarySelectedBlock(selection) {
  var _a, _b;
  if (selection.type === "none") return void 0;
  if (selection.type === "text") return (_a = selection.text) == null ? void 0 : _a.blockId;
  if (selection.type === "block") return (_b = selection.blocks) == null ? void 0 : _b.anchor;
  return void 0;
}
function getTextSelection(selection) {
  return selection.type === "text" ? selection.text : void 0;
}
function isSelectionEmpty(selection) {
  return selection.type === "none";
}
function isTextSelection(selection) {
  return selection.type === "text";
}
function isBlockSelectionType(selection) {
  return selection.type === "block";
}
function getBlockOrder$1(doc) {
  const order = [];
  function traverse(blockId) {
    order.push(blockId);
    const block = doc.blocks.get(blockId);
    if (block) {
      for (const childId of block.children) {
        traverse(childId);
      }
    }
  }
  traverse(doc.root);
  return order;
}
function getNextBlock(doc, currentId) {
  const order = getBlockOrder$1(doc);
  const currentIndex = order.indexOf(currentId);
  if (currentIndex === -1 || currentIndex === order.length - 1) {
    return void 0;
  }
  return order[currentIndex + 1];
}
function getPreviousBlock(doc, currentId) {
  const order = getBlockOrder$1(doc);
  const currentIndex = order.indexOf(currentId);
  if (currentIndex <= 0) {
    return void 0;
  }
  return order[currentIndex - 1];
}
function getParentBlock(doc, blockId) {
  for (const [id, block] of doc.blocks) {
    if (block.children.includes(blockId)) {
      return id;
    }
  }
  return void 0;
}
function getFirstChildBlock(doc, blockId) {
  const block = doc.blocks.get(blockId);
  return block == null ? void 0 : block.children[0];
}
function getSiblingBlocks(doc, blockId) {
  const parentId = getParentBlock(doc, blockId);
  if (!parentId) return [];
  const parent = doc.blocks.get(parentId);
  return (parent == null ? void 0 : parent.children) ?? [];
}
function getNextSibling(doc, blockId) {
  const siblings = getSiblingBlocks(doc, blockId);
  const index = siblings.indexOf(blockId);
  if (index === -1 || index === siblings.length - 1) {
    return void 0;
  }
  return siblings[index + 1];
}
function getPreviousSibling(doc, blockId) {
  const siblings = getSiblingBlocks(doc, blockId);
  const index = siblings.indexOf(blockId);
  if (index <= 0) {
    return void 0;
  }
  return siblings[index - 1];
}
function expandSelection(doc, selection, targetId) {
  var _a, _b;
  if (selection.type === "none") {
    return createBlockSelection(targetId);
  }
  if (selection.type === "text") {
    const currentBlockId = (_a = selection.text) == null ? void 0 : _a.blockId;
    if (!currentBlockId) return createBlockSelection(targetId);
    return expandBlockRange(doc, currentBlockId, targetId);
  }
  if (selection.type === "block") {
    const anchor = (_b = selection.blocks) == null ? void 0 : _b.anchor;
    if (!anchor) return createBlockSelection(targetId);
    return expandBlockRange(doc, anchor, targetId);
  }
  return selection;
}
function expandBlockRange(doc, anchorId, focusId) {
  const order = getBlockOrder$1(doc);
  const anchorIndex = order.indexOf(anchorId);
  const focusIndex = order.indexOf(focusId);
  if (anchorIndex === -1 || focusIndex === -1) {
    return createBlockSelection(focusId);
  }
  const start = Math.min(anchorIndex, focusIndex);
  const end = Math.max(anchorIndex, focusIndex);
  const selectedIds = order.slice(start, end + 1);
  return createMultiBlockSelection(selectedIds, anchorId, focusId);
}
class SelectionManager {
  constructor(doc, config = {}) {
    __publicField(this, "doc");
    __publicField(this, "selection");
    __publicField(this, "config");
    this.doc = doc;
    this.selection = createEmptySelection();
    this.config = config;
  }
  /**
   * Update the document reference.
   */
  setDocument(doc) {
    this.doc = doc;
    if (this.selection.focusedBlockId && !doc.blocks.has(this.selection.focusedBlockId)) {
      this.clear();
    }
  }
  /**
   * Get current selection.
   */
  getSelection() {
    return this.selection;
  }
  /**
   * Update selection and notify.
   */
  setSelection(selection) {
    var _a, _b;
    this.selection = selection;
    (_b = (_a = this.config).onSelectionChange) == null ? void 0 : _b.call(_a, selection);
  }
  /**
   * Select a single block.
   */
  select(blockId) {
    if (!this.doc.blocks.has(blockId)) {
      logger$1.warn("Attempted to select non-existent block", { blockId });
      return;
    }
    this.setSelection(createBlockSelection(blockId));
  }
  /**
   * Select multiple blocks.
   */
  selectMultiple(blockIds) {
    const validIds = blockIds.filter((id) => this.doc.blocks.has(id));
    if (validIds.length === 0) {
      this.clear();
      return;
    }
    this.setSelection(createMultiBlockSelection(validIds));
  }
  /**
   * Clear selection.
   */
  clear() {
    this.setSelection(createEmptySelection());
  }
  /**
   * Select text within a block.
   */
  selectText(blockId, start, end) {
    if (!this.doc.blocks.has(blockId)) {
      logger$1.warn("Attempted to select text in non-existent block", { blockId });
      return;
    }
    this.setSelection(createTextSelection(blockId, start, end));
  }
  /**
   * Expand selection to include another block.
   */
  expandTo(blockId) {
    if (!this.doc.blocks.has(blockId)) return;
    this.setSelection(expandSelection(this.doc, this.selection, blockId));
  }
  /**
   * Move selection to next block.
   */
  moveNext() {
    const currentId = this.selection.focusedBlockId;
    if (!currentId) {
      const order = getBlockOrder$1(this.doc);
      if (order.length > 1) {
        this.select(order[1]);
        return true;
      }
      return false;
    }
    const nextId = getNextBlock(this.doc, currentId);
    if (nextId) {
      this.select(nextId);
      return true;
    }
    return false;
  }
  /**
   * Move selection to previous block.
   */
  movePrevious() {
    const currentId = this.selection.focusedBlockId;
    if (!currentId) return false;
    const prevId = getPreviousBlock(this.doc, currentId);
    if (prevId && prevId !== this.doc.root) {
      this.select(prevId);
      return true;
    }
    return false;
  }
  /**
   * Move selection to parent block.
   */
  moveToParent() {
    const currentId = this.selection.focusedBlockId;
    if (!currentId) return false;
    const parentId = getParentBlock(this.doc, currentId);
    if (parentId && parentId !== this.doc.root) {
      this.select(parentId);
      return true;
    }
    return false;
  }
  /**
   * Move selection to first child.
   */
  moveToFirstChild() {
    const currentId = this.selection.focusedBlockId;
    if (!currentId) return false;
    const childId = getFirstChildBlock(this.doc, currentId);
    if (childId) {
      this.select(childId);
      return true;
    }
    return false;
  }
  /**
   * Expand selection to next block.
   */
  expandNext() {
    const currentId = this.selection.focusedBlockId;
    if (!currentId) return this.moveNext();
    const nextId = getNextBlock(this.doc, currentId);
    if (nextId) {
      this.expandTo(nextId);
      return true;
    }
    return false;
  }
  /**
   * Expand selection to previous block.
   */
  expandPrevious() {
    const currentId = this.selection.focusedBlockId;
    if (!currentId) return this.movePrevious();
    const prevId = getPreviousBlock(this.doc, currentId);
    if (prevId && prevId !== this.doc.root) {
      this.expandTo(prevId);
      return true;
    }
    return false;
  }
  /**
   * Select all blocks.
   */
  selectAll() {
    const order = getBlockOrder$1(this.doc);
    const blockIds = order.slice(1);
    if (blockIds.length > 0) {
      this.setSelection(createMultiBlockSelection(blockIds));
    }
  }
  /**
   * Check if block is selected.
   */
  isSelected(blockId) {
    return isBlockSelected(this.selection, blockId);
  }
  /**
   * Check if block is focused.
   */
  isFocused(blockId) {
    return isBlockFocused(this.selection, blockId);
  }
}
const logger = new Logger({ context: "DiffEngine" });
function computeTextDiff(oldText, newText) {
  if (oldText === newText) {
    return { operations: [{ type: "equal", text: oldText }] };
  }
  const operations = [];
  if (oldText.length < 1e3 && newText.length < 1e3) {
    const result = computeCharacterDiff(oldText, newText);
    return { operations: result };
  }
  const oldLines = oldText.split("\n");
  const newLines = newText.split("\n");
  const lcs = computeLCS(oldLines, newLines);
  let oldIdx = 0;
  let newIdx = 0;
  let lcsIdx = 0;
  while (oldIdx < oldLines.length || newIdx < newLines.length) {
    if (lcsIdx < lcs.length && oldLines[oldIdx] === lcs[lcsIdx] && newLines[newIdx] === lcs[lcsIdx]) {
      operations.push({ type: "equal", text: oldLines[oldIdx] + "\n" });
      oldIdx++;
      newIdx++;
      lcsIdx++;
    } else if (oldIdx < oldLines.length && (lcsIdx >= lcs.length || oldLines[oldIdx] !== lcs[lcsIdx])) {
      operations.push({ type: "delete", text: oldLines[oldIdx] + "\n" });
      oldIdx++;
    } else if (newIdx < newLines.length) {
      operations.push({ type: "insert", text: newLines[newIdx] + "\n" });
      newIdx++;
    }
  }
  return { operations: mergeOperations(operations) };
}
function computeCharacterDiff(oldText, newText) {
  const operations = [];
  let prefixLen = 0;
  while (prefixLen < oldText.length && prefixLen < newText.length && oldText[prefixLen] === newText[prefixLen]) {
    prefixLen++;
  }
  let suffixLen = 0;
  while (suffixLen < oldText.length - prefixLen && suffixLen < newText.length - prefixLen && oldText[oldText.length - 1 - suffixLen] === newText[newText.length - 1 - suffixLen]) {
    suffixLen++;
  }
  if (prefixLen > 0) {
    operations.push({ type: "equal", text: oldText.slice(0, prefixLen) });
  }
  const oldMiddle = oldText.slice(prefixLen, oldText.length - suffixLen);
  const newMiddle = newText.slice(prefixLen, newText.length - suffixLen);
  if (oldMiddle.length > 0) {
    operations.push({ type: "delete", text: oldMiddle });
  }
  if (newMiddle.length > 0) {
    operations.push({ type: "insert", text: newMiddle });
  }
  if (suffixLen > 0) {
    operations.push({ type: "equal", text: oldText.slice(oldText.length - suffixLen) });
  }
  return operations;
}
function computeLCS(a, b) {
  const m = a.length;
  const n = b.length;
  if (m > 1e3 || n > 1e3) {
    return computeLCSOptimized(a, b);
  }
  const dp = Array(m + 1).fill(null).map(() => Array(n + 1).fill(0));
  for (let i2 = 1; i2 <= m; i2++) {
    for (let j2 = 1; j2 <= n; j2++) {
      if (a[i2 - 1] === b[j2 - 1]) {
        dp[i2][j2] = dp[i2 - 1][j2 - 1] + 1;
      } else {
        dp[i2][j2] = Math.max(dp[i2 - 1][j2], dp[i2][j2 - 1]);
      }
    }
  }
  const lcs = [];
  let i = m;
  let j = n;
  while (i > 0 && j > 0) {
    if (a[i - 1] === b[j - 1]) {
      lcs.unshift(a[i - 1]);
      i--;
      j--;
    } else if (dp[i - 1][j] > dp[i][j - 1]) {
      i--;
    } else {
      j--;
    }
  }
  return lcs;
}
function computeLCSOptimized(a, b) {
  const commonElements = [];
  const bSet = new Set(b);
  for (const item of a) {
    if (bSet.has(item)) {
      commonElements.push(item);
    }
  }
  return commonElements;
}
function mergeOperations(operations) {
  if (operations.length === 0) return [];
  const merged = [];
  let current = operations[0];
  for (let i = 1; i < operations.length; i++) {
    const op = operations[i];
    if (op.type === current.type) {
      current = { type: current.type, text: current.text + op.text };
    } else {
      merged.push(current);
      current = op;
    }
  }
  merged.push(current);
  return merged;
}
function getBlockInfo(doc, blockId) {
  const block = doc.blocks.get(blockId);
  if (!block) return void 0;
  let parentId;
  let index = 0;
  for (const [id, b] of doc.blocks) {
    const childIndex = b.children.indexOf(blockId);
    if (childIndex !== -1) {
      parentId = id;
      index = childIndex;
      break;
    }
  }
  return {
    id: blockId,
    parentId,
    index,
    content: block.content,
    type: block.type
  };
}
function computeStructuralChanges(oldDoc, newDoc, commonBlockIds) {
  const changes = [];
  for (const blockId of commonBlockIds) {
    const oldInfo = getBlockInfo(oldDoc, blockId);
    const newInfo = getBlockInfo(newDoc, blockId);
    if (!oldInfo || !newInfo) continue;
    if (oldInfo.parentId !== newInfo.parentId) {
      changes.push({
        type: "moved",
        blockId,
        oldParentId: oldInfo.parentId,
        newParentId: newInfo.parentId,
        oldIndex: oldInfo.index,
        newIndex: newInfo.index
      });
    } else if (oldInfo.index !== newInfo.index) {
      changes.push({
        type: "reordered",
        blockId,
        oldParentId: oldInfo.parentId,
        newParentId: newInfo.parentId,
        oldIndex: oldInfo.index,
        newIndex: newInfo.index
      });
    }
  }
  for (const blockId of newDoc.blocks.keys()) {
    if (!oldDoc.blocks.has(blockId)) {
      const info = getBlockInfo(newDoc, blockId);
      if (info) {
        changes.push({
          type: "added",
          blockId,
          newParentId: info.parentId,
          newIndex: info.index
        });
      }
    }
  }
  for (const blockId of oldDoc.blocks.keys()) {
    if (!newDoc.blocks.has(blockId)) {
      const info = getBlockInfo(oldDoc, blockId);
      if (info) {
        changes.push({
          type: "removed",
          blockId,
          oldParentId: info.parentId,
          oldIndex: info.index
        });
      }
    }
  }
  return changes;
}
function computeMetadataChanges(oldBlock, newBlock) {
  const changes = [];
  if (oldBlock.type !== newBlock.type) {
    changes.push({ field: "type", oldValue: oldBlock.type, newValue: newBlock.type });
  }
  if (oldBlock.role !== newBlock.role) {
    changes.push({ field: "role", oldValue: oldBlock.role, newValue: newBlock.role });
  }
  if (oldBlock.label !== newBlock.label) {
    changes.push({ field: "label", oldValue: oldBlock.label, newValue: newBlock.label });
  }
  const oldTags = new Set(oldBlock.tags);
  const newTags = new Set(newBlock.tags);
  const addedTags = [...newTags].filter((t) => !oldTags.has(t));
  const removedTags = [...oldTags].filter((t) => !newTags.has(t));
  if (addedTags.length > 0 || removedTags.length > 0) {
    changes.push({
      field: "tags",
      oldValue: [...oldTags],
      newValue: [...newTags]
    });
  }
  if (oldBlock.edges.length !== newBlock.edges.length) {
    changes.push({
      field: "edges",
      oldValue: oldBlock.edges.length,
      newValue: newBlock.edges.length
    });
  }
  return changes;
}
function computeBlockDiff(oldBlock, newBlock) {
  if (!oldBlock && newBlock) {
    return {
      blockId: newBlock.id,
      changeType: "added",
      newBlock
    };
  }
  if (oldBlock && !newBlock) {
    return {
      blockId: oldBlock.id,
      changeType: "removed",
      oldBlock
    };
  }
  if (oldBlock && newBlock) {
    const contentChanged = oldBlock.content !== newBlock.content;
    const metadataChanges = computeMetadataChanges(oldBlock, newBlock);
    if (!contentChanged && metadataChanges.length === 0) {
      return {
        blockId: oldBlock.id,
        changeType: "unchanged",
        oldBlock,
        newBlock
      };
    }
    return {
      blockId: oldBlock.id,
      changeType: "modified",
      oldBlock,
      newBlock,
      contentDiff: contentChanged ? computeTextDiff(oldBlock.content, newBlock.content) : void 0,
      metadataChanges: metadataChanges.length > 0 ? metadataChanges : void 0
    };
  }
  throw new Error("Invalid block diff state");
}
function computeDocumentDiff(oldDoc, newDoc, fromSnapshotId, toSnapshotId) {
  logger.debug("Computing document diff", {
    fromSnapshot: fromSnapshotId,
    toSnapshot: toSnapshotId,
    oldBlockCount: oldDoc.blocks.size,
    newBlockCount: newDoc.blocks.size
  });
  const blockDiffs = /* @__PURE__ */ new Map();
  const allBlockIds = /* @__PURE__ */ new Set([...oldDoc.blocks.keys(), ...newDoc.blocks.keys()]);
  const commonBlockIds = /* @__PURE__ */ new Set();
  const summary = {
    added: 0,
    removed: 0,
    modified: 0,
    moved: 0,
    unchanged: 0
  };
  for (const blockId of allBlockIds) {
    const oldBlock = oldDoc.blocks.get(blockId);
    const newBlock = newDoc.blocks.get(blockId);
    if (oldBlock && newBlock) {
      commonBlockIds.add(blockId);
    }
    const diff = computeBlockDiff(oldBlock, newBlock);
    blockDiffs.set(blockId, diff);
    switch (diff.changeType) {
      case "added":
        summary.added++;
        break;
      case "removed":
        summary.removed++;
        break;
      case "modified":
        summary.modified++;
        break;
      case "unchanged":
        summary.unchanged++;
        break;
    }
  }
  const structuralChanges = computeStructuralChanges(oldDoc, newDoc, commonBlockIds);
  summary.moved = structuralChanges.filter((c) => c.type === "moved").length;
  logger.debug("Diff computed", summary);
  return {
    fromSnapshotId,
    toSnapshotId,
    blockDiffs,
    structuralChanges,
    summary
  };
}
function getChangedBlocks(diff) {
  return Array.from(diff.blockDiffs.values()).filter((d) => d.changeType !== "unchanged");
}
function getBlocksByChangeType(diff, changeType) {
  return Array.from(diff.blockDiffs.values()).filter((d) => d.changeType === changeType);
}
function hasBlockChanged(diff, blockId) {
  const blockDiff = diff.blockDiffs.get(blockId);
  return blockDiff !== void 0 && blockDiff.changeType !== "unchanged";
}
function getBlockTextDiff(diff, blockId) {
  const blockDiff = diff.blockDiffs.get(blockId);
  return blockDiff == null ? void 0 : blockDiff.contentDiff;
}
function formatTextDiff(textDiff) {
  return textDiff.operations.map((op) => {
    switch (op.type) {
      case "equal":
        return op.text;
      case "insert":
        return `[+${op.text}+]`;
      case "delete":
        return `[-${op.text}-]`;
    }
  }).join("");
}
function hasDiffChanges(diff) {
  return diff.summary.added > 0 || diff.summary.removed > 0 || diff.summary.modified > 0 || diff.summary.moved > 0;
}
function useEditorStore(config) {
  const storeRef = useRef(null);
  if (storeRef.current === null) {
    storeRef.current = createEditorStore(config);
  }
  return storeRef.current;
}
function useEditorState(store, selector) {
  const subscribe = useCallback(
    (callback) => store.subscribe(callback),
    [store]
  );
  const getSnapshot = useCallback(() => selector(store.getState()), [store, selector]);
  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}
function useDocument(store) {
  return useEditorState(store, (state) => state.document);
}
function useSelection(store) {
  const selection = useEditorState(store, (state) => state.selection);
  const editingBlockId = useEditorState(store, (state) => state.editingBlockId);
  return {
    selection,
    editingBlockId,
    isBlockSelected: (blockId) => {
      var _a, _b;
      if (selection.type === "block") {
        return ((_a = selection.blocks) == null ? void 0 : _a.blockIds.includes(blockId)) ?? false;
      }
      if (selection.type === "text") {
        return ((_b = selection.text) == null ? void 0 : _b.blockId) === blockId;
      }
      return false;
    },
    isBlockFocused: (blockId) => selection.focusedBlockId === blockId,
    isBlockEditing: (blockId) => editingBlockId === blockId
  };
}
function useHistory(store) {
  const history = useEditorState(store, (state) => state.history);
  return {
    canUndo: history.canUndo,
    canRedo: history.canRedo,
    entries: history.entries,
    currentIndex: history.currentIndex,
    undo: store.undo,
    redo: store.redo
  };
}
function useDrag(store) {
  const drag = useEditorState(store, (state) => state.drag);
  return {
    isDragging: drag.isDragging,
    sourceId: drag.sourceId,
    targetId: drag.targetId,
    position: drag.position,
    startDrag: store.startDrag,
    updateTarget: store.updateDragTarget,
    endDrag: store.endDrag
  };
}
function useView(store) {
  const view = useEditorState(store, (state) => state.view);
  const mode = useEditorState(store, (state) => state.mode);
  return {
    view,
    mode,
    setView: store.setView,
    setMode: store.setMode
  };
}
function useBlockActions(store) {
  return {
    addBlock: store.addBlock,
    editBlock: store.editBlock,
    deleteBlock: store.deleteBlock,
    moveBlock: store.moveBlock,
    changeBlockType: store.changeBlockType
  };
}
function useEditActions(store) {
  const editingBlockId = useEditorState(store, (state) => state.editingBlockId);
  const pendingContent = useEditorState(store, (state) => state.pendingContent);
  const editState = useEditorState(store, (state) => state.editState);
  return {
    editingBlockId,
    pendingContent,
    editState,
    startEditing: store.startEditing,
    stopEditing: store.stopEditing,
    updateContent: store.updatePendingContent
  };
}
function useEditorEvent(store, eventType, handler) {
  useEffect(() => {
    return store.events.on(eventType, handler);
  }, [store, eventType, handler]);
}
function useKeyboardShortcuts(store, enabled = true) {
  useEffect(() => {
    if (!enabled) return;
    const handleKeyDown = (event) => {
      var _a;
      const isMod = event.metaKey || event.ctrlKey;
      const isShift = event.shiftKey;
      if (isMod && !isShift && event.key === "z") {
        event.preventDefault();
        if (store.history.canUndo) {
          store.undo();
        }
        return;
      }
      if (isMod && isShift && event.key === "z" || isMod && event.key === "y") {
        event.preventDefault();
        if (store.history.canRedo) {
          store.redo();
        }
        return;
      }
      if (isMod && event.key === "s") {
        event.preventDefault();
        store.saveDocument().catch(console.error);
        return;
      }
      if (isMod && event.key === "a") {
        event.preventDefault();
        if (store.document) {
          const blockIds = Array.from(store.document.blocks.keys()).filter(
            (id) => id !== store.document.root
          );
          store.selectMultiple(blockIds);
        }
        return;
      }
      if (event.key === "Escape") {
        if (store.editingBlockId) {
          store.stopEditing(false);
        } else if (store.drag.isDragging) {
          store.endDrag(false);
        } else {
          store.clearSelection();
        }
        return;
      }
      if (event.key === "Enter" && !isMod) {
        const focusedId = store.selection.focusedBlockId;
        if (focusedId && !store.editingBlockId) {
          event.preventDefault();
          store.startEditing(focusedId);
        }
        return;
      }
      if ((event.key === "Delete" || event.key === "Backspace") && !store.editingBlockId) {
        const selectedIds = store.selection.type === "block" ? (_a = store.selection.blocks) == null ? void 0 : _a.blockIds : void 0;
        if (selectedIds && selectedIds.length > 0) {
          event.preventDefault();
          selectedIds.forEach((id) => {
            try {
              store.deleteBlock(id);
            } catch {
            }
          });
        }
        return;
      }
      if (!store.editingBlockId) {
        if (event.key === "ArrowUp") {
          event.preventDefault();
          navigateUp(store, isShift);
          return;
        }
        if (event.key === "ArrowDown") {
          event.preventDefault();
          navigateDown(store, isShift);
          return;
        }
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [store, enabled]);
}
function navigateUp(store, expand) {
  var _a, _b;
  if (!store.document) return;
  const currentId = store.selection.focusedBlockId;
  if (!currentId) {
    const firstChild = (_a = store.document.blocks.get(store.document.root)) == null ? void 0 : _a.children[0];
    if (firstChild) {
      store.select(firstChild);
    }
    return;
  }
  const order = getBlockOrder(store.document);
  const currentIndex = order.indexOf(currentId);
  if (currentIndex > 1) {
    const prevId = order[currentIndex - 1];
    if (expand) {
      const selectedIds = ((_b = store.selection.blocks) == null ? void 0 : _b.blockIds) ?? [currentId];
      if (!selectedIds.includes(prevId)) {
        store.selectMultiple([...selectedIds, prevId]);
      }
    } else {
      store.select(prevId);
    }
  }
}
function navigateDown(store, expand) {
  var _a, _b;
  if (!store.document) return;
  const currentId = store.selection.focusedBlockId;
  if (!currentId) {
    const firstChild = (_a = store.document.blocks.get(store.document.root)) == null ? void 0 : _a.children[0];
    if (firstChild) {
      store.select(firstChild);
    }
    return;
  }
  const order = getBlockOrder(store.document);
  const currentIndex = order.indexOf(currentId);
  if (currentIndex < order.length - 1) {
    const nextId = order[currentIndex + 1];
    if (expand) {
      const selectedIds = ((_b = store.selection.blocks) == null ? void 0 : _b.blockIds) ?? [currentId];
      if (!selectedIds.includes(nextId)) {
        store.selectMultiple([...selectedIds, nextId]);
      }
    } else {
      store.select(nextId);
    }
  }
}
function getBlockOrder(doc) {
  const order = [];
  function traverse(blockId) {
    order.push(blockId);
    const block = doc.blocks.get(blockId);
    if (block) {
      for (const childId of block.children) {
        traverse(childId);
      }
    }
  }
  traverse(doc.root);
  return order;
}
const styles$5 = {
  overlay: {
    position: "absolute",
    top: "100%",
    right: 0,
    zIndex: 1e3,
    marginTop: "4px",
    minWidth: "280px",
    maxWidth: "400px",
    backgroundColor: "#fff",
    border: "1px solid #e5e5e5",
    borderRadius: "8px",
    boxShadow: "0 4px 12px rgba(0, 0, 0, 0.15)",
    overflow: "hidden"
  },
  header: {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "8px 12px",
    backgroundColor: "#fafafa",
    borderBottom: "1px solid #e5e5e5"
  },
  headerTitle: {
    fontSize: "12px",
    fontWeight: 600,
    color: "#333"
  },
  closeButton: {
    padding: "2px 6px",
    border: "none",
    backgroundColor: "transparent",
    cursor: "pointer",
    fontSize: "14px",
    color: "#999",
    lineHeight: 1
  },
  content: {
    padding: "8px 12px",
    maxHeight: "300px",
    overflow: "auto"
  },
  section: {
    marginBottom: "12px"
  },
  sectionLast: {
    marginBottom: 0
  },
  sectionTitle: {
    fontSize: "10px",
    fontWeight: 600,
    color: "#999",
    textTransform: "uppercase",
    letterSpacing: "0.5px",
    marginBottom: "4px"
  },
  row: {
    display: "flex",
    alignItems: "flex-start",
    marginBottom: "4px"
  },
  label: {
    flex: "0 0 80px",
    fontSize: "11px",
    color: "#666"
  },
  value: {
    flex: 1,
    fontSize: "11px",
    color: "#333",
    wordBreak: "break-all"
  },
  valueCode: {
    fontFamily: "Monaco, Consolas, monospace",
    fontSize: "10px",
    backgroundColor: "#f5f5f5",
    padding: "1px 4px",
    borderRadius: "2px"
  },
  tags: {
    display: "flex",
    flexWrap: "wrap",
    gap: "4px"
  },
  tag: {
    display: "inline-block",
    padding: "2px 6px",
    fontSize: "10px",
    backgroundColor: "#e3f2fd",
    color: "#1976d2",
    borderRadius: "10px"
  },
  edge: {
    display: "flex",
    alignItems: "center",
    gap: "4px",
    marginBottom: "4px",
    fontSize: "10px"
  },
  edgeType: {
    padding: "1px 4px",
    backgroundColor: "#f5f5f5",
    borderRadius: "2px",
    color: "#666"
  },
  edgeTarget: {
    fontFamily: "Monaco, Consolas, monospace",
    fontSize: "9px",
    color: "#999"
  },
  noData: {
    fontSize: "11px",
    color: "#999",
    fontStyle: "italic"
  },
  customValue: {
    fontSize: "10px",
    fontFamily: "Monaco, Consolas, monospace",
    backgroundColor: "#f9f9f9",
    padding: "4px 6px",
    borderRadius: "2px",
    overflow: "auto",
    maxHeight: "80px"
  }
};
function MetadataTooltip({ block, onClose }) {
  const metadata = block.metadata;
  const formatDate = (date) => {
    if (!date) return "N/A";
    if (typeof date === "string") {
      return new Date(date).toLocaleString();
    }
    return date.toLocaleString();
  };
  const truncateId2 = (id) => {
    if (id.length <= 20) return id;
    return `${id.slice(0, 10)}...${id.slice(-6)}`;
  };
  return /* @__PURE__ */ jsxs(
    "div",
    {
      style: styles$5.overlay,
      onClick: (e) => e.stopPropagation(),
      "data-testid": "metadata-tooltip",
      children: [
        /* @__PURE__ */ jsxs("div", { style: styles$5.header, children: [
          /* @__PURE__ */ jsx("span", { style: styles$5.headerTitle, children: "Block Metadata" }),
          /* @__PURE__ */ jsx("button", { style: styles$5.closeButton, onClick: onClose, title: "Close", children: "×" })
        ] }),
        /* @__PURE__ */ jsxs("div", { style: styles$5.content, children: [
          /* @__PURE__ */ jsxs("div", { style: styles$5.section, children: [
            /* @__PURE__ */ jsx("div", { style: styles$5.sectionTitle, children: "Identity" }),
            /* @__PURE__ */ jsxs("div", { style: styles$5.row, children: [
              /* @__PURE__ */ jsx("span", { style: styles$5.label, children: "ID" }),
              /* @__PURE__ */ jsx("span", { style: styles$5.value, children: /* @__PURE__ */ jsx("code", { style: styles$5.valueCode, title: block.id, children: truncateId2(block.id) }) })
            ] }),
            /* @__PURE__ */ jsxs("div", { style: styles$5.row, children: [
              /* @__PURE__ */ jsx("span", { style: styles$5.label, children: "Type" }),
              /* @__PURE__ */ jsx("span", { style: styles$5.value, children: block.type })
            ] }),
            (block.role || (metadata == null ? void 0 : metadata.semanticRole)) && /* @__PURE__ */ jsxs("div", { style: styles$5.row, children: [
              /* @__PURE__ */ jsx("span", { style: styles$5.label, children: "Role" }),
              /* @__PURE__ */ jsx("span", { style: styles$5.value, children: block.role || (metadata == null ? void 0 : metadata.semanticRole) })
            ] }),
            block.label && /* @__PURE__ */ jsxs("div", { style: styles$5.row, children: [
              /* @__PURE__ */ jsx("span", { style: styles$5.label, children: "Label" }),
              /* @__PURE__ */ jsx("span", { style: styles$5.value, children: block.label })
            ] })
          ] }),
          /* @__PURE__ */ jsxs("div", { style: styles$5.section, children: [
            /* @__PURE__ */ jsx("div", { style: styles$5.sectionTitle, children: "Tags" }),
            block.tags.length > 0 ? /* @__PURE__ */ jsx("div", { style: styles$5.tags, children: block.tags.map((tag, index) => /* @__PURE__ */ jsx("span", { style: styles$5.tag, children: tag }, index)) }) : /* @__PURE__ */ jsx("span", { style: styles$5.noData, children: "No tags" })
          ] }),
          /* @__PURE__ */ jsxs("div", { style: styles$5.section, children: [
            /* @__PURE__ */ jsx("div", { style: styles$5.sectionTitle, children: "Timestamps" }),
            /* @__PURE__ */ jsxs("div", { style: styles$5.row, children: [
              /* @__PURE__ */ jsx("span", { style: styles$5.label, children: "Created" }),
              /* @__PURE__ */ jsx("span", { style: styles$5.value, children: formatDate(metadata == null ? void 0 : metadata.createdAt) })
            ] }),
            /* @__PURE__ */ jsxs("div", { style: styles$5.row, children: [
              /* @__PURE__ */ jsx("span", { style: styles$5.label, children: "Modified" }),
              /* @__PURE__ */ jsx("span", { style: styles$5.value, children: formatDate(metadata == null ? void 0 : metadata.modifiedAt) })
            ] })
          ] }),
          /* @__PURE__ */ jsxs("div", { style: styles$5.section, children: [
            /* @__PURE__ */ jsx("div", { style: styles$5.sectionTitle, children: "Structure" }),
            /* @__PURE__ */ jsxs("div", { style: styles$5.row, children: [
              /* @__PURE__ */ jsx("span", { style: styles$5.label, children: "Children" }),
              /* @__PURE__ */ jsx("span", { style: styles$5.value, children: block.children.length })
            ] }),
            /* @__PURE__ */ jsxs("div", { style: styles$5.row, children: [
              /* @__PURE__ */ jsx("span", { style: styles$5.label, children: "Content" }),
              /* @__PURE__ */ jsxs("span", { style: styles$5.value, children: [
                block.content.length,
                " chars"
              ] })
            ] })
          ] }),
          /* @__PURE__ */ jsxs("div", { style: styles$5.section, children: [
            /* @__PURE__ */ jsxs("div", { style: styles$5.sectionTitle, children: [
              "Edges (",
              block.edges.length,
              ")"
            ] }),
            block.edges.length > 0 ? /* @__PURE__ */ jsxs("div", { children: [
              block.edges.slice(0, 5).map((edge, index) => /* @__PURE__ */ jsxs("div", { style: styles$5.edge, children: [
                /* @__PURE__ */ jsx("span", { style: styles$5.edgeType, children: edge.edgeType }),
                /* @__PURE__ */ jsx("span", { children: "→" }),
                /* @__PURE__ */ jsx("span", { style: styles$5.edgeTarget, title: edge.target, children: truncateId2(edge.target) })
              ] }, index)),
              block.edges.length > 5 && /* @__PURE__ */ jsxs("span", { style: styles$5.noData, children: [
                "+",
                block.edges.length - 5,
                " more edges"
              ] })
            ] }) : /* @__PURE__ */ jsx("span", { style: styles$5.noData, children: "No edges" })
          ] }),
          (metadata == null ? void 0 : metadata.custom) && Object.keys(metadata.custom).length > 0 && /* @__PURE__ */ jsxs("div", { style: { ...styles$5.section, ...styles$5.sectionLast }, children: [
            /* @__PURE__ */ jsx("div", { style: styles$5.sectionTitle, children: "Custom Metadata" }),
            /* @__PURE__ */ jsx("pre", { style: styles$5.customValue, children: JSON.stringify(metadata.custom, null, 2) })
          ] })
        ] })
      ]
    }
  );
}
const styles$4 = {
  container: {
    position: "relative"
  },
  textarea: {
    width: "100%",
    minHeight: "24px",
    padding: "0",
    margin: "0",
    border: "none",
    outline: "none",
    resize: "none",
    backgroundColor: "transparent",
    fontFamily: "inherit",
    fontSize: "inherit",
    fontWeight: "inherit",
    lineHeight: "inherit",
    color: "inherit",
    overflow: "hidden"
  },
  codeTextarea: {
    fontFamily: 'Monaco, Consolas, "Courier New", monospace',
    fontSize: "13px",
    backgroundColor: "#f5f5f5",
    padding: "12px",
    borderRadius: "4px",
    whiteSpace: "pre"
  },
  toolbar: {
    display: "flex",
    alignItems: "center",
    gap: "4px",
    padding: "4px 0",
    marginBottom: "4px",
    borderBottom: "1px solid #e5e5e5"
  },
  toolbarButton: {
    padding: "2px 6px",
    border: "1px solid #ddd",
    borderRadius: "2px",
    backgroundColor: "#fff",
    cursor: "pointer",
    fontSize: "11px",
    color: "#666",
    transition: "background-color 0.1s"
  },
  typeSelector: {
    padding: "2px 4px",
    border: "1px solid #ddd",
    borderRadius: "2px",
    backgroundColor: "#fff",
    fontSize: "11px",
    cursor: "pointer"
  },
  hint: {
    fontSize: "10px",
    color: "#999",
    marginTop: "4px"
  }
};
function BlockEditor({ block, store }) {
  const textareaRef = useRef(null);
  const [showToolbar, setShowToolbar] = useState(false);
  const pendingContent = useEditorState(store, (s) => s.pendingContent);
  const content = pendingContent ?? block.content;
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.focus();
      textareaRef.current.setSelectionRange(
        textareaRef.current.value.length,
        textareaRef.current.value.length
      );
    }
  }, []);
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, [content]);
  const handleChange = useCallback(
    (e) => {
      store.updatePendingContent(e.target.value);
    },
    [store]
  );
  const handleKeyDown = useCallback(
    (e) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
        e.preventDefault();
        store.stopEditing(true);
        return;
      }
      if (e.key === "Escape") {
        e.preventDefault();
        store.stopEditing(false);
        return;
      }
      if (e.key === "Enter" && !e.shiftKey && !e.metaKey && !e.ctrlKey) {
        const textarea = textareaRef.current;
        if (textarea && textarea.selectionStart === textarea.value.length) {
          e.preventDefault();
          store.stopEditing(true);
          const parent = findParent(store, block.id);
          if (parent) {
            const newId = store.addBlock(parent, "");
            store.startEditing(newId);
          }
          return;
        }
      }
      if (e.key === "Tab" && block.type === "code") {
        e.preventDefault();
        const textarea = textareaRef.current;
        if (textarea) {
          const start = textarea.selectionStart;
          const end = textarea.selectionEnd;
          const value = textarea.value;
          const newValue = value.substring(0, start) + "  " + value.substring(end);
          store.updatePendingContent(newValue);
          setTimeout(() => {
            textarea.selectionStart = textarea.selectionEnd = start + 2;
          }, 0);
        }
        return;
      }
      e.stopPropagation();
    },
    [store, block.id, block.type]
  );
  const handleBlur = useCallback(() => {
    setTimeout(() => {
      var _a;
      if ((_a = document.activeElement) == null ? void 0 : _a.closest("[data-editor-toolbar]")) {
        return;
      }
      store.stopEditing(true);
    }, 100);
  }, [store]);
  const handleTypeChange = useCallback(
    (e) => {
      store.changeBlockType(block.id, e.target.value);
    },
    [store, block.id]
  );
  const getTextareaStyle = () => {
    var _a;
    const baseStyle = { ...styles$4.textarea };
    if (block.type === "code") {
      Object.assign(baseStyle, styles$4.codeTextarea);
    }
    const role = block.role ?? ((_a = block.metadata) == null ? void 0 : _a.semanticRole);
    switch (role) {
      case "heading1":
        baseStyle.fontSize = "28px";
        baseStyle.fontWeight = "700";
        break;
      case "heading2":
        baseStyle.fontSize = "24px";
        baseStyle.fontWeight = "600";
        break;
      case "heading3":
        baseStyle.fontSize = "20px";
        baseStyle.fontWeight = "600";
        break;
      case "heading4":
        baseStyle.fontSize = "18px";
        baseStyle.fontWeight = "600";
        break;
      case "heading5":
        baseStyle.fontSize = "16px";
        baseStyle.fontWeight = "600";
        break;
      case "heading6":
        baseStyle.fontSize = "14px";
        baseStyle.fontWeight = "600";
        break;
    }
    return baseStyle;
  };
  return /* @__PURE__ */ jsxs(
    "div",
    {
      style: styles$4.container,
      onMouseEnter: () => setShowToolbar(true),
      onMouseLeave: () => setShowToolbar(false),
      children: [
        showToolbar && /* @__PURE__ */ jsxs("div", { style: styles$4.toolbar, "data-editor-toolbar": true, children: [
          /* @__PURE__ */ jsxs(
            "select",
            {
              value: block.type,
              onChange: handleTypeChange,
              style: styles$4.typeSelector,
              onClick: (e) => e.stopPropagation(),
              children: [
                /* @__PURE__ */ jsx("option", { value: "text", children: "Text" }),
                /* @__PURE__ */ jsx("option", { value: "code", children: "Code" }),
                /* @__PURE__ */ jsx("option", { value: "table", children: "Table" }),
                /* @__PURE__ */ jsx("option", { value: "math", children: "Math" }),
                /* @__PURE__ */ jsx("option", { value: "json", children: "JSON" }),
                /* @__PURE__ */ jsx("option", { value: "media", children: "Media" })
              ]
            }
          ),
          /* @__PURE__ */ jsx(
            "button",
            {
              style: styles$4.toolbarButton,
              onClick: (e) => {
                e.stopPropagation();
                store.deleteBlock(block.id);
              },
              title: "Delete block",
              children: "Delete"
            }
          )
        ] }),
        /* @__PURE__ */ jsx(
          "textarea",
          {
            ref: textareaRef,
            value: content,
            onChange: handleChange,
            onKeyDown: handleKeyDown,
            onBlur: handleBlur,
            style: getTextareaStyle(),
            placeholder: "Type something...",
            rows: 1,
            spellCheck: block.type !== "code",
            "data-testid": `block-editor-${block.id}`
          }
        ),
        /* @__PURE__ */ jsxs("div", { style: styles$4.hint, children: [
          "Press ",
          /* @__PURE__ */ jsx("kbd", { children: "Esc" }),
          " to cancel, ",
          /* @__PURE__ */ jsx("kbd", { children: "Cmd+Enter" }),
          " to save"
        ] })
      ]
    }
  );
}
function findParent(store, blockId) {
  const document2 = store.document;
  if (!document2) return void 0;
  for (const [id, block] of document2.blocks) {
    if (block.children.includes(blockId)) {
      return id;
    }
  }
  return void 0;
}
const styles$3 = {
  blockWrapper: {
    position: "relative",
    marginBottom: "4px"
  },
  block: {
    position: "relative",
    padding: "4px 8px",
    borderRadius: "4px",
    cursor: "pointer",
    transition: "background-color 0.1s, box-shadow 0.1s",
    minHeight: "24px"
  },
  blockHover: {
    backgroundColor: "#f5f5f5"
  },
  blockSelected: {
    backgroundColor: "#e3f2fd",
    boxShadow: "0 0 0 2px #2196f3"
  },
  blockEditing: {
    backgroundColor: "#fff",
    boxShadow: "0 0 0 2px #4caf50"
  },
  blockDragTarget: {
    backgroundColor: "#e8f5e9"
  },
  dragHandle: {
    position: "absolute",
    left: "-24px",
    top: "50%",
    transform: "translateY(-50%)",
    width: "16px",
    height: "16px",
    cursor: "grab",
    opacity: 0,
    transition: "opacity 0.1s",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    color: "#999",
    fontSize: "10px"
  },
  dragHandleVisible: {
    opacity: 1
  },
  dropIndicator: {
    position: "absolute",
    left: 0,
    right: 0,
    height: "2px",
    backgroundColor: "#2196f3",
    pointerEvents: "none"
  },
  dropIndicatorBefore: {
    top: "-2px"
  },
  dropIndicatorAfter: {
    bottom: "-2px"
  },
  dropIndicatorInside: {
    top: 0,
    bottom: 0,
    left: 0,
    right: 0,
    height: "auto",
    border: "2px dashed #2196f3",
    backgroundColor: "rgba(33, 150, 243, 0.1)"
  },
  children: {
    paddingLeft: "24px",
    borderLeft: "1px solid #e5e5e5",
    marginLeft: "8px"
  },
  addButton: {
    padding: "2px 8px",
    border: "none",
    backgroundColor: "transparent",
    color: "#999",
    cursor: "pointer",
    fontSize: "12px",
    opacity: 0,
    transition: "opacity 0.1s"
  },
  addButtonVisible: {
    opacity: 1
  },
  // Content type styles
  heading1: {
    fontSize: "28px",
    fontWeight: 700,
    marginTop: "16px",
    marginBottom: "8px"
  },
  heading2: {
    fontSize: "24px",
    fontWeight: 600,
    marginTop: "14px",
    marginBottom: "6px"
  },
  heading3: {
    fontSize: "20px",
    fontWeight: 600,
    marginTop: "12px",
    marginBottom: "4px"
  },
  heading4: {
    fontSize: "18px",
    fontWeight: 600,
    marginTop: "10px",
    marginBottom: "4px"
  },
  heading5: {
    fontSize: "16px",
    fontWeight: 600,
    marginTop: "8px",
    marginBottom: "4px"
  },
  heading6: {
    fontSize: "14px",
    fontWeight: 600,
    marginTop: "8px",
    marginBottom: "4px"
  },
  paragraph: {
    fontSize: "14px",
    lineHeight: "1.6"
  },
  code: {
    fontFamily: 'Monaco, Consolas, "Courier New", monospace',
    fontSize: "13px",
    backgroundColor: "#f5f5f5",
    padding: "12px",
    borderRadius: "4px",
    overflow: "auto",
    whiteSpace: "pre-wrap"
  },
  quote: {
    borderLeft: "3px solid #ddd",
    paddingLeft: "12px",
    color: "#666",
    fontStyle: "italic"
  }
};
function BlockRenderer({
  block,
  document: document2,
  store,
  depth,
  path
}) {
  var _a;
  const [isHovered, setIsHovered] = useState(false);
  const [showTooltip, setShowTooltip] = useState(false);
  const blockRef = useRef(null);
  const selection = useEditorState(store, (s) => s.selection);
  const editingBlockId = useEditorState(store, (s) => s.editingBlockId);
  const drag = useEditorState(store, (s) => s.drag);
  const isSelected = selection.type === "block" ? ((_a = selection.blocks) == null ? void 0 : _a.blockIds.includes(block.id)) ?? false : selection.focusedBlockId === block.id;
  const isEditing = editingBlockId === block.id;
  const isDragSource = drag.sourceId === block.id;
  const isDragTarget = drag.targetId === block.id;
  const handleClick = useCallback(
    (e) => {
      var _a2;
      e.stopPropagation();
      if (!isEditing) {
        if (e.shiftKey && selection.focusedBlockId) {
          const currentIds = ((_a2 = selection.blocks) == null ? void 0 : _a2.blockIds) ?? [];
          if (!currentIds.includes(block.id)) {
            store.selectMultiple([...currentIds, block.id]);
          }
        } else {
          store.select(block.id);
        }
      }
    },
    [store, block.id, isEditing, selection]
  );
  const handleDoubleClick = useCallback(
    (e) => {
      e.stopPropagation();
      if (!isEditing) {
        store.startEditing(block.id);
      }
    },
    [store, block.id, isEditing]
  );
  const handleMouseEnter = useCallback(() => {
    setIsHovered(true);
  }, []);
  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
    setShowTooltip(false);
  }, []);
  const handleDragStart = useCallback(
    (e) => {
      e.dataTransfer.effectAllowed = "move";
      e.dataTransfer.setData("text/plain", block.id);
      store.startDrag(block.id);
    },
    [store, block.id]
  );
  const handleDragOver = useCallback(
    (e) => {
      var _a2;
      if (!drag.isDragging || drag.sourceId === block.id) return;
      e.preventDefault();
      e.dataTransfer.dropEffect = "move";
      const rect = (_a2 = blockRef.current) == null ? void 0 : _a2.getBoundingClientRect();
      if (!rect) return;
      const relativeY = e.clientY - rect.top;
      const height = rect.height;
      let position;
      if (relativeY < height * 0.25) {
        position = "before";
      } else if (relativeY > height * 0.75) {
        position = "after";
      } else {
        position = "inside";
      }
      store.updateDragTarget(block.id, position);
    },
    [store, block.id, drag.isDragging, drag.sourceId]
  );
  const handleDragLeave = useCallback(() => {
  }, []);
  const handleDrop = useCallback(
    (e) => {
      e.preventDefault();
      if (drag.isDragging) {
        store.endDrag(true);
      }
    },
    [store, drag.isDragging]
  );
  const handleDragEnd = useCallback(() => {
    store.endDrag(false);
  }, [store]);
  const handleAddChild = useCallback(() => {
    const newId = store.addBlock(block.id, "");
    store.startEditing(newId);
  }, [store, block.id]);
  const handleTooltipToggle = useCallback(() => {
    setShowTooltip((prev) => !prev);
  }, []);
  const getBlockStyle = () => {
    const baseStyle = { ...styles$3.block };
    if (isHovered && !isEditing) {
      Object.assign(baseStyle, styles$3.blockHover);
    }
    if (isSelected && !isEditing) {
      Object.assign(baseStyle, styles$3.blockSelected);
    }
    if (isEditing) {
      Object.assign(baseStyle, styles$3.blockEditing);
    }
    if (isDragTarget && !isDragSource) {
      Object.assign(baseStyle, styles$3.blockDragTarget);
    }
    return baseStyle;
  };
  const renderContent = () => {
    if (isEditing) {
      return /* @__PURE__ */ jsx(BlockEditor, { block, store });
    }
    return /* @__PURE__ */ jsx(BlockContent, { block });
  };
  const renderDropIndicator = () => {
    if (!isDragTarget || isDragSource) return null;
    switch (drag.position) {
      case "before":
        return /* @__PURE__ */ jsx(
          "div",
          {
            style: { ...styles$3.dropIndicator, ...styles$3.dropIndicatorBefore }
          }
        );
      case "after":
        return /* @__PURE__ */ jsx(
          "div",
          {
            style: { ...styles$3.dropIndicator, ...styles$3.dropIndicatorAfter }
          }
        );
      case "inside":
        return /* @__PURE__ */ jsx("div", { style: { ...styles$3.dropIndicator, ...styles$3.dropIndicatorInside } });
      default:
        return null;
    }
  };
  return /* @__PURE__ */ jsxs(
    "div",
    {
      style: styles$3.blockWrapper,
      "data-block-id": block.id,
      "data-testid": `block-${block.id}`,
      children: [
        /* @__PURE__ */ jsxs(
          "div",
          {
            ref: blockRef,
            style: getBlockStyle(),
            onClick: handleClick,
            onDoubleClick: handleDoubleClick,
            onMouseEnter: handleMouseEnter,
            onMouseLeave: handleMouseLeave,
            draggable: !isEditing && store.config.enableDragDrop,
            onDragStart: handleDragStart,
            onDragOver: handleDragOver,
            onDragLeave: handleDragLeave,
            onDrop: handleDrop,
            onDragEnd: handleDragEnd,
            children: [
              /* @__PURE__ */ jsx(
                "div",
                {
                  style: {
                    ...styles$3.dragHandle,
                    ...isHovered ? styles$3.dragHandleVisible : {}
                  },
                  onMouseDown: (e) => e.stopPropagation(),
                  children: "⋮⋮"
                }
              ),
              renderContent(),
              renderDropIndicator(),
              isHovered && !isEditing && /* @__PURE__ */ jsx(
                "button",
                {
                  style: {
                    position: "absolute",
                    right: "4px",
                    top: "4px",
                    padding: "2px 6px",
                    fontSize: "10px",
                    border: "1px solid #ddd",
                    borderRadius: "2px",
                    backgroundColor: "#fff",
                    cursor: "pointer"
                  },
                  onClick: (e) => {
                    e.stopPropagation();
                    handleTooltipToggle();
                  },
                  title: "Show metadata",
                  children: "i"
                }
              ),
              showTooltip && /* @__PURE__ */ jsx(MetadataTooltip, { block, onClose: () => setShowTooltip(false) })
            ]
          }
        ),
        block.children.length > 0 && /* @__PURE__ */ jsx("div", { style: styles$3.children, children: block.children.map((childId) => {
          const child = document2.blocks.get(childId);
          if (!child) return null;
          return /* @__PURE__ */ jsx(
            BlockRenderer,
            {
              block: child,
              document: document2,
              store,
              depth: depth + 1,
              path: [...path, block.id]
            },
            childId
          );
        }) }),
        isHovered && !isEditing && /* @__PURE__ */ jsx(
          "button",
          {
            style: {
              ...styles$3.addButton,
              ...isHovered ? styles$3.addButtonVisible : {}
            },
            onClick: (e) => {
              e.stopPropagation();
              handleAddChild();
            },
            children: "+ Add block"
          }
        )
      ]
    }
  );
}
function BlockContent({ block }) {
  var _a;
  const role = block.role ?? ((_a = block.metadata) == null ? void 0 : _a.semanticRole);
  const type = block.type;
  const getContentStyle = () => {
    switch (role) {
      case "heading1":
        return styles$3.heading1;
      case "heading2":
        return styles$3.heading2;
      case "heading3":
        return styles$3.heading3;
      case "heading4":
        return styles$3.heading4;
      case "heading5":
        return styles$3.heading5;
      case "heading6":
        return styles$3.heading6;
      case "code":
        return styles$3.code;
      case "quote":
        return styles$3.quote;
      default:
        return styles$3.paragraph;
    }
  };
  switch (type) {
    case "code":
      return /* @__PURE__ */ jsx("pre", { style: styles$3.code, children: /* @__PURE__ */ jsx("code", { children: block.content || " " }) });
    case "table":
      return /* @__PURE__ */ jsx("div", { style: { overflow: "auto" }, children: /* @__PURE__ */ jsx("pre", { style: { ...styles$3.code, whiteSpace: "pre" }, children: block.content || "Empty table" }) });
    case "math":
      return /* @__PURE__ */ jsx(
        "div",
        {
          style: {
            fontFamily: "serif",
            fontStyle: "italic",
            textAlign: "center",
            padding: "8px"
          },
          children: block.content || "Empty equation"
        }
      );
    case "media":
      return /* @__PURE__ */ jsxs(
        "div",
        {
          style: {
            padding: "8px",
            backgroundColor: "#f5f5f5",
            borderRadius: "4px",
            textAlign: "center"
          },
          children: [
            "[Media: ",
            block.content || "No source",
            "]"
          ]
        }
      );
    case "json":
      return /* @__PURE__ */ jsx("pre", { style: styles$3.code, children: /* @__PURE__ */ jsx("code", { children: formatJson(block.content) }) });
    default:
      return /* @__PURE__ */ jsx("div", { style: getContentStyle(), children: block.content || /* @__PURE__ */ jsx("span", { style: { color: "#999" }, children: "Empty block" }) });
  }
}
function formatJson(content) {
  try {
    return JSON.stringify(JSON.parse(content), null, 2);
  } catch {
    return content;
  }
}
const styles$2 = {
  container: {
    position: "relative",
    width: "100%",
    height: "100%",
    overflow: "hidden",
    backgroundColor: "#fafafa"
  },
  canvas: {
    position: "absolute",
    top: 0,
    left: 0,
    width: "100%",
    height: "100%"
  },
  controls: {
    position: "absolute",
    top: "16px",
    right: "16px",
    display: "flex",
    flexDirection: "column",
    gap: "8px",
    zIndex: 10
  },
  controlGroup: {
    display: "flex",
    gap: "4px",
    backgroundColor: "#fff",
    padding: "4px",
    borderRadius: "4px",
    boxShadow: "0 2px 4px rgba(0, 0, 0, 0.1)"
  },
  controlButton: {
    padding: "6px 12px",
    border: "1px solid #ddd",
    borderRadius: "4px",
    backgroundColor: "#fff",
    cursor: "pointer",
    fontSize: "12px"
  },
  controlButtonActive: {
    backgroundColor: "#e3f2fd",
    borderColor: "#2196f3"
  },
  legend: {
    position: "absolute",
    bottom: "16px",
    left: "16px",
    backgroundColor: "#fff",
    padding: "12px",
    borderRadius: "4px",
    boxShadow: "0 2px 4px rgba(0, 0, 0, 0.1)",
    fontSize: "11px"
  },
  legendTitle: {
    fontWeight: 600,
    marginBottom: "8px"
  },
  legendItem: {
    display: "flex",
    alignItems: "center",
    gap: "8px",
    marginBottom: "4px"
  },
  legendColor: {
    width: "16px",
    height: "3px",
    borderRadius: "1px"
  },
  nodeTooltip: {
    position: "absolute",
    backgroundColor: "#333",
    color: "#fff",
    padding: "6px 10px",
    borderRadius: "4px",
    fontSize: "11px",
    maxWidth: "200px",
    zIndex: 100,
    pointerEvents: "none"
  }
};
const EDGE_COLORS = {
  // Structural (default)
  parent_child: "#9e9e9e",
  // Derivation
  derived_from: "#2196f3",
  supersedes: "#1976d2",
  transformed_from: "#1565c0",
  // Reference
  references: "#4caf50",
  cited_by: "#388e3c",
  links_to: "#2e7d32",
  // Semantic
  supports: "#9c27b0",
  contradicts: "#f44336",
  elaborates: "#7b1fa2",
  summarizes: "#6a1b9a",
  // Version
  version_of: "#ff9800",
  alternative_of: "#f57c00",
  translation_of: "#ef6c00"
};
function GraphView({ store }) {
  const containerRef = useRef(null);
  const canvasRef = useRef(null);
  const document2 = useEditorState(store, (s) => s.document);
  const graphState = useEditorState(store, (s) => s.graph);
  const selection = useEditorState(store, (s) => s.selection);
  const [viewport, setViewport] = useState({ x: 0, y: 0, zoom: 1 });
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const [hoveredNodeId, setHoveredNodeId] = useState(null);
  const [tooltipPos, setTooltipPos] = useState({ x: 0, y: 0 });
  const { nodes, edges } = useMemo(() => {
    if (!document2) {
      return { nodes: /* @__PURE__ */ new Map(), edges: [] };
    }
    return computeLayout(document2, graphState.layout, selection.focusedBlockId);
  }, [document2, graphState.layout, selection.focusedBlockId]);
  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    const rect = container.getBoundingClientRect();
    canvas.width = rect.width * window.devicePixelRatio;
    canvas.height = rect.height * window.devicePixelRatio;
    canvas.style.width = `${rect.width}px`;
    canvas.style.height = `${rect.height}px`;
    ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    ctx.clearRect(0, 0, rect.width, rect.height);
    ctx.save();
    ctx.translate(viewport.x + rect.width / 2, viewport.y + rect.height / 2);
    ctx.scale(viewport.zoom, viewport.zoom);
    for (const edge of edges) {
      drawEdge(ctx, edge, nodes, hoveredNodeId);
    }
    for (const node of nodes.values()) {
      drawNode(ctx, node, selection.focusedBlockId === node.id, hoveredNodeId === node.id);
    }
    ctx.restore();
  }, [nodes, edges, viewport, selection.focusedBlockId, hoveredNodeId]);
  const handleMouseDown = useCallback((e) => {
    setIsDragging(true);
    setDragStart({ x: e.clientX - viewport.x, y: e.clientY - viewport.y });
  }, [viewport]);
  const handleMouseMove = useCallback(
    (e) => {
      if (isDragging) {
        setViewport({
          ...viewport,
          x: e.clientX - dragStart.x,
          y: e.clientY - dragStart.y
        });
      } else {
        const container = containerRef.current;
        if (!container) return;
        const rect = container.getBoundingClientRect();
        const mouseX = (e.clientX - rect.left - rect.width / 2 - viewport.x) / viewport.zoom;
        const mouseY = (e.clientY - rect.top - rect.height / 2 - viewport.y) / viewport.zoom;
        let found = null;
        for (const node of nodes.values()) {
          if (mouseX >= node.x - node.width / 2 && mouseX <= node.x + node.width / 2 && mouseY >= node.y - node.height / 2 && mouseY <= node.y + node.height / 2) {
            found = node.id;
            setTooltipPos({ x: e.clientX, y: e.clientY });
            break;
          }
        }
        setHoveredNodeId(found);
      }
    },
    [isDragging, dragStart, viewport, nodes]
  );
  const handleMouseUp = useCallback(() => {
    setIsDragging(false);
  }, []);
  const handleClick = useCallback(() => {
    if (hoveredNodeId) {
      store.select(hoveredNodeId);
    }
  }, [store, hoveredNodeId]);
  const handleWheel = useCallback(
    (e) => {
      e.preventDefault();
      const delta = e.deltaY > 0 ? 0.9 : 1.1;
      setViewport((v) => ({
        ...v,
        zoom: Math.max(0.1, Math.min(3, v.zoom * delta))
      }));
    },
    []
  );
  const handleLayoutChange = useCallback(
    (layout) => {
      store.setGraphLayout(layout);
    },
    [store]
  );
  const handleZoomIn = useCallback(() => {
    setViewport((v) => ({ ...v, zoom: Math.min(3, v.zoom * 1.2) }));
  }, []);
  const handleZoomOut = useCallback(() => {
    setViewport((v) => ({ ...v, zoom: Math.max(0.1, v.zoom / 1.2) }));
  }, []);
  const handleResetView = useCallback(() => {
    setViewport({ x: 0, y: 0, zoom: 1 });
  }, []);
  const hoveredNode = hoveredNodeId ? nodes.get(hoveredNodeId) : null;
  return /* @__PURE__ */ jsxs(
    "div",
    {
      ref: containerRef,
      style: styles$2.container,
      onMouseDown: handleMouseDown,
      onMouseMove: handleMouseMove,
      onMouseUp: handleMouseUp,
      onMouseLeave: handleMouseUp,
      onClick: handleClick,
      onWheel: handleWheel,
      "data-testid": "graph-view",
      children: [
        /* @__PURE__ */ jsx("canvas", { ref: canvasRef, style: styles$2.canvas }),
        /* @__PURE__ */ jsxs("div", { style: styles$2.controls, children: [
          /* @__PURE__ */ jsxs("div", { style: styles$2.controlGroup, children: [
            /* @__PURE__ */ jsx(
              LayoutButton,
              {
                layout: "hierarchical",
                currentLayout: graphState.layout,
                onClick: handleLayoutChange
              }
            ),
            /* @__PURE__ */ jsx(
              LayoutButton,
              {
                layout: "force",
                currentLayout: graphState.layout,
                onClick: handleLayoutChange
              }
            ),
            /* @__PURE__ */ jsx(
              LayoutButton,
              {
                layout: "radial",
                currentLayout: graphState.layout,
                onClick: handleLayoutChange
              }
            )
          ] }),
          /* @__PURE__ */ jsxs("div", { style: styles$2.controlGroup, children: [
            /* @__PURE__ */ jsx("button", { onClick: handleZoomIn, style: styles$2.controlButton, children: "+" }),
            /* @__PURE__ */ jsx("button", { onClick: handleZoomOut, style: styles$2.controlButton, children: "-" }),
            /* @__PURE__ */ jsx("button", { onClick: handleResetView, style: styles$2.controlButton, children: "Reset" })
          ] })
        ] }),
        /* @__PURE__ */ jsxs("div", { style: styles$2.legend, children: [
          /* @__PURE__ */ jsx("div", { style: styles$2.legendTitle, children: "Edge Types" }),
          /* @__PURE__ */ jsxs("div", { style: styles$2.legendItem, children: [
            /* @__PURE__ */ jsx("div", { style: { ...styles$2.legendColor, backgroundColor: EDGE_COLORS.parent_child } }),
            /* @__PURE__ */ jsx("span", { children: "Parent-Child" })
          ] }),
          /* @__PURE__ */ jsxs("div", { style: styles$2.legendItem, children: [
            /* @__PURE__ */ jsx("div", { style: { ...styles$2.legendColor, backgroundColor: EDGE_COLORS.references } }),
            /* @__PURE__ */ jsx("span", { children: "References" })
          ] }),
          /* @__PURE__ */ jsxs("div", { style: styles$2.legendItem, children: [
            /* @__PURE__ */ jsx("div", { style: { ...styles$2.legendColor, backgroundColor: EDGE_COLORS.supports } }),
            /* @__PURE__ */ jsx("span", { children: "Semantic" })
          ] })
        ] }),
        hoveredNode && /* @__PURE__ */ jsxs(
          "div",
          {
            style: {
              ...styles$2.nodeTooltip,
              left: tooltipPos.x + 10,
              top: tooltipPos.y + 10
            },
            children: [
              /* @__PURE__ */ jsx("div", { children: /* @__PURE__ */ jsx("strong", { children: hoveredNode.block.type }) }),
              /* @__PURE__ */ jsxs("div", { children: [
                hoveredNode.block.content.slice(0, 50),
                hoveredNode.block.content.length > 50 ? "..." : ""
              ] }),
              /* @__PURE__ */ jsxs("div", { style: { color: "#aaa", fontSize: "9px" }, children: [
                hoveredNode.block.children.length,
                " children, ",
                hoveredNode.block.edges.length,
                " edges"
              ] })
            ]
          }
        )
      ]
    }
  );
}
function LayoutButton({ layout, currentLayout, onClick }) {
  const labels = {
    hierarchical: "Tree",
    force: "Force",
    dagre: "DAG",
    radial: "Radial"
  };
  return /* @__PURE__ */ jsx(
    "button",
    {
      onClick: () => onClick(layout),
      style: {
        ...styles$2.controlButton,
        ...layout === currentLayout ? styles$2.controlButtonActive : {}
      },
      children: labels[layout]
    }
  );
}
function computeLayout(doc, layout, selectedId) {
  const nodes = /* @__PURE__ */ new Map();
  const edges = [];
  const NODE_WIDTH = 120;
  const NODE_HEIGHT = 40;
  const H_SPACING = 40;
  const V_SPACING = 60;
  switch (layout) {
    case "hierarchical":
      computeHierarchicalLayout(doc, nodes, NODE_WIDTH, NODE_HEIGHT, H_SPACING, V_SPACING);
      break;
    case "force":
      computeForceLayout(doc, nodes, NODE_WIDTH, NODE_HEIGHT);
      break;
    case "radial":
      computeRadialLayout(doc, nodes, NODE_WIDTH, NODE_HEIGHT);
      break;
    default:
      computeHierarchicalLayout(doc, nodes, NODE_WIDTH, NODE_HEIGHT, H_SPACING, V_SPACING);
  }
  if (selectedId && nodes.has(selectedId)) {
    const node = nodes.get(selectedId);
    nodes.set(selectedId, { ...node, isSelected: true });
  }
  for (const [blockId, block] of doc.blocks) {
    for (const childId of block.children) {
      const sourceNode = nodes.get(blockId);
      const targetNode = nodes.get(childId);
      if (sourceNode && targetNode) {
        edges.push({
          id: `${blockId}->${childId}`,
          sourceId: blockId,
          targetId: childId,
          edgeType: "parent_child",
          points: [
            { x: sourceNode.x, y: sourceNode.y + sourceNode.height / 2 },
            { x: targetNode.x, y: targetNode.y - targetNode.height / 2 }
          ],
          isHighlighted: false
        });
      }
    }
    for (const edge of block.edges) {
      const sourceNode = nodes.get(blockId);
      const targetNode = nodes.get(edge.target);
      if (sourceNode && targetNode) {
        edges.push({
          id: `${blockId}-${edge.edgeType}->${edge.target}`,
          sourceId: blockId,
          targetId: edge.target,
          edgeType: edge.edgeType,
          points: [
            { x: sourceNode.x + sourceNode.width / 2, y: sourceNode.y },
            { x: targetNode.x - targetNode.width / 2, y: targetNode.y }
          ],
          isHighlighted: false
        });
      }
    }
  }
  return { nodes, edges };
}
function computeHierarchicalLayout(doc, nodes, nodeWidth, nodeHeight, hSpacing, vSpacing) {
  const depths = /* @__PURE__ */ new Map();
  const childrenCounts = /* @__PURE__ */ new Map();
  function computeDepth(blockId, depth) {
    depths.set(blockId, depth);
    childrenCounts.set(depth, (childrenCounts.get(depth) ?? 0) + 1);
    const block = doc.blocks.get(blockId);
    if (block) {
      for (const childId of block.children) {
        computeDepth(childId, depth + 1);
      }
    }
  }
  computeDepth(doc.root, 0);
  const xPositions = /* @__PURE__ */ new Map();
  for (const [blockId, depth] of depths) {
    const block = doc.blocks.get(blockId);
    if (!block) continue;
    const xPos = xPositions.get(depth) ?? 0;
    const totalAtDepth = childrenCounts.get(depth) ?? 1;
    const x = xPos - (totalAtDepth - 1) * (nodeWidth + hSpacing) / 2;
    nodes.set(blockId, {
      id: blockId,
      x,
      y: depth * (nodeHeight + vSpacing),
      width: nodeWidth,
      height: nodeHeight,
      block,
      depth,
      isExpanded: true,
      isSelected: false,
      isHighlighted: false
    });
    xPositions.set(depth, xPos + nodeWidth + hSpacing);
  }
}
function computeForceLayout(doc, nodes, nodeWidth, nodeHeight) {
  const positions = /* @__PURE__ */ new Map();
  let i = 0;
  for (const blockId of doc.blocks.keys()) {
    const angle = i / doc.blocks.size * 2 * Math.PI;
    const radius = 150;
    positions.set(blockId, {
      x: Math.cos(angle) * radius,
      y: Math.sin(angle) * radius,
      vx: 0,
      vy: 0
    });
    i++;
  }
  for (let iter = 0; iter < 50; iter++) {
    for (const [id1, pos1] of positions) {
      for (const [id2, pos2] of positions) {
        if (id1 >= id2) continue;
        const dx = pos2.x - pos1.x;
        const dy = pos2.y - pos1.y;
        const dist = Math.max(1, Math.sqrt(dx * dx + dy * dy));
        const force = 1e3 / (dist * dist);
        const fx = dx / dist * force;
        const fy = dy / dist * force;
        pos1.vx -= fx;
        pos1.vy -= fy;
        pos2.vx += fx;
        pos2.vy += fy;
      }
    }
    for (const [blockId, block] of doc.blocks) {
      const pos1 = positions.get(blockId);
      if (!pos1) continue;
      for (const childId of block.children) {
        const pos2 = positions.get(childId);
        if (!pos2) continue;
        const dx = pos2.x - pos1.x;
        const dy = pos2.y - pos1.y;
        const dist = Math.max(1, Math.sqrt(dx * dx + dy * dy));
        const force = dist * 0.01;
        const fx = dx / dist * force;
        const fy = dy / dist * force;
        pos1.vx += fx;
        pos1.vy += fy;
        pos2.vx -= fx;
        pos2.vy -= fy;
      }
    }
    for (const pos of positions.values()) {
      pos.x += pos.vx * 0.1;
      pos.y += pos.vy * 0.1;
      pos.vx *= 0.9;
      pos.vy *= 0.9;
    }
  }
  for (const [blockId, pos] of positions) {
    const block = doc.blocks.get(blockId);
    if (!block) continue;
    nodes.set(blockId, {
      id: blockId,
      x: pos.x,
      y: pos.y,
      width: nodeWidth,
      height: nodeHeight,
      block,
      depth: 0,
      isExpanded: true,
      isSelected: false,
      isHighlighted: false
    });
  }
}
function computeRadialLayout(doc, nodes, nodeWidth, nodeHeight) {
  const depths = /* @__PURE__ */ new Map();
  function computeDepth(blockId, depth) {
    depths.set(blockId, depth);
    const block = doc.blocks.get(blockId);
    if (block) {
      for (const childId of block.children) {
        computeDepth(childId, depth + 1);
      }
    }
  }
  computeDepth(doc.root, 0);
  const byDepth = /* @__PURE__ */ new Map();
  for (const [blockId, depth] of depths) {
    if (!byDepth.has(depth)) {
      byDepth.set(depth, []);
    }
    byDepth.get(depth).push(blockId);
  }
  const radiusStep = 100;
  for (const [depth, blockIds] of byDepth) {
    const radius = depth * radiusStep;
    const angleStep = 2 * Math.PI / blockIds.length;
    blockIds.forEach((blockId, index) => {
      const block = doc.blocks.get(blockId);
      if (!block) return;
      const angle = index * angleStep - Math.PI / 2;
      nodes.set(blockId, {
        id: blockId,
        x: Math.cos(angle) * radius,
        y: Math.sin(angle) * radius,
        width: nodeWidth,
        height: nodeHeight,
        block,
        depth,
        isExpanded: true,
        isSelected: false,
        isHighlighted: false
      });
    });
  }
}
function drawNode(ctx, node, isSelected, isHovered) {
  const { x, y, width, height, block } = node;
  ctx.fillStyle = isSelected ? "#e3f2fd" : isHovered ? "#f5f5f5" : "#fff";
  ctx.strokeStyle = isSelected ? "#2196f3" : "#ddd";
  ctx.lineWidth = isSelected ? 2 : 1;
  const radius = 6;
  ctx.beginPath();
  ctx.moveTo(x - width / 2 + radius, y - height / 2);
  ctx.lineTo(x + width / 2 - radius, y - height / 2);
  ctx.quadraticCurveTo(x + width / 2, y - height / 2, x + width / 2, y - height / 2 + radius);
  ctx.lineTo(x + width / 2, y + height / 2 - radius);
  ctx.quadraticCurveTo(x + width / 2, y + height / 2, x + width / 2 - radius, y + height / 2);
  ctx.lineTo(x - width / 2 + radius, y + height / 2);
  ctx.quadraticCurveTo(x - width / 2, y + height / 2, x - width / 2, y + height / 2 - radius);
  ctx.lineTo(x - width / 2, y - height / 2 + radius);
  ctx.quadraticCurveTo(x - width / 2, y - height / 2, x - width / 2 + radius, y - height / 2);
  ctx.closePath();
  ctx.fill();
  ctx.stroke();
  const typeColor = getTypeColor(block.type);
  ctx.fillStyle = typeColor;
  ctx.fillRect(x - width / 2, y - height / 2, 4, height);
  ctx.fillStyle = "#333";
  ctx.font = "11px -apple-system, BlinkMacSystemFont, sans-serif";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  const label = block.content.slice(0, 15) || block.type;
  ctx.fillText(label + (block.content.length > 15 ? "..." : ""), x, y);
}
function drawEdge(ctx, edge, nodes, hoveredNodeId) {
  const source = nodes.get(edge.sourceId);
  const target = nodes.get(edge.targetId);
  if (!source || !target) return;
  const isHighlighted = edge.sourceId === hoveredNodeId || edge.targetId === hoveredNodeId;
  const color = getEdgeColor(edge.edgeType);
  ctx.strokeStyle = isHighlighted ? color : `${color}80`;
  ctx.lineWidth = isHighlighted ? 2 : 1;
  ctx.beginPath();
  ctx.moveTo(edge.points[0].x, edge.points[0].y);
  if (edge.points.length === 2) {
    const midY = (edge.points[0].y + edge.points[1].y) / 2;
    ctx.bezierCurveTo(
      edge.points[0].x,
      midY,
      edge.points[1].x,
      midY,
      edge.points[1].x,
      edge.points[1].y
    );
  } else {
    for (let i = 1; i < edge.points.length; i++) {
      ctx.lineTo(edge.points[i].x, edge.points[i].y);
    }
  }
  ctx.stroke();
  const lastPoint = edge.points[edge.points.length - 1];
  const prevPoint = edge.points[edge.points.length - 2] ?? edge.points[0];
  const angle = Math.atan2(lastPoint.y - prevPoint.y, lastPoint.x - prevPoint.x);
  const arrowSize = 6;
  ctx.fillStyle = isHighlighted ? color : `${color}80`;
  ctx.beginPath();
  ctx.moveTo(lastPoint.x, lastPoint.y);
  ctx.lineTo(
    lastPoint.x - arrowSize * Math.cos(angle - Math.PI / 6),
    lastPoint.y - arrowSize * Math.sin(angle - Math.PI / 6)
  );
  ctx.lineTo(
    lastPoint.x - arrowSize * Math.cos(angle + Math.PI / 6),
    lastPoint.y - arrowSize * Math.sin(angle + Math.PI / 6)
  );
  ctx.closePath();
  ctx.fill();
}
function getTypeColor(type) {
  const colors = {
    text: "#4caf50",
    code: "#ff9800",
    table: "#2196f3",
    math: "#9c27b0",
    json: "#795548",
    media: "#e91e63",
    binary: "#607d8b",
    composite: "#00bcd4"
  };
  return colors[type] ?? "#9e9e9e";
}
function getEdgeColor(type) {
  return EDGE_COLORS[type] ?? "#9e9e9e";
}
const styles$1 = {
  container: {
    display: "flex",
    flexDirection: "column",
    height: "100%",
    backgroundColor: "#fff"
  },
  header: {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "12px 16px",
    borderBottom: "1px solid #e5e5e5",
    backgroundColor: "#fafafa"
  },
  headerTitle: {
    fontSize: "14px",
    fontWeight: 600
  },
  headerActions: {
    display: "flex",
    gap: "8px"
  },
  button: {
    padding: "6px 12px",
    border: "1px solid #ddd",
    borderRadius: "4px",
    backgroundColor: "#fff",
    cursor: "pointer",
    fontSize: "12px"
  },
  buttonPrimary: {
    backgroundColor: "#4caf50",
    borderColor: "#4caf50",
    color: "#fff"
  },
  snapshotSelector: {
    display: "flex",
    alignItems: "center",
    gap: "12px",
    padding: "12px 16px",
    borderBottom: "1px solid #e5e5e5"
  },
  snapshotSelect: {
    padding: "6px 8px",
    border: "1px solid #ddd",
    borderRadius: "4px",
    fontSize: "12px",
    minWidth: "150px"
  },
  content: {
    flex: 1,
    overflow: "auto",
    padding: "16px"
  },
  summary: {
    display: "flex",
    gap: "16px",
    padding: "12px 16px",
    backgroundColor: "#f5f5f5",
    borderRadius: "4px",
    marginBottom: "16px",
    fontSize: "12px"
  },
  summaryItem: {
    display: "flex",
    alignItems: "center",
    gap: "4px"
  },
  summaryBadge: {
    display: "inline-block",
    padding: "2px 8px",
    borderRadius: "10px",
    fontSize: "11px",
    fontWeight: 500
  },
  badgeAdded: {
    backgroundColor: "#e8f5e9",
    color: "#2e7d32"
  },
  badgeRemoved: {
    backgroundColor: "#ffebee",
    color: "#c62828"
  },
  badgeModified: {
    backgroundColor: "#fff3e0",
    color: "#ef6c00"
  },
  badgeMoved: {
    backgroundColor: "#e3f2fd",
    color: "#1565c0"
  },
  diffList: {
    display: "flex",
    flexDirection: "column",
    gap: "8px"
  },
  diffItem: {
    border: "1px solid #e5e5e5",
    borderRadius: "4px",
    overflow: "hidden"
  },
  diffItemHeader: {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "8px 12px",
    backgroundColor: "#fafafa",
    borderBottom: "1px solid #e5e5e5",
    cursor: "pointer"
  },
  diffItemHeaderLeft: {
    display: "flex",
    alignItems: "center",
    gap: "8px"
  },
  diffItemId: {
    fontFamily: "Monaco, Consolas, monospace",
    fontSize: "10px",
    color: "#666"
  },
  diffItemType: {
    fontSize: "11px",
    color: "#333"
  },
  diffItemActions: {
    display: "flex",
    gap: "4px"
  },
  diffItemContent: {
    padding: "12px"
  },
  diffLine: {
    fontFamily: "Monaco, Consolas, monospace",
    fontSize: "12px",
    lineHeight: "1.5",
    padding: "2px 4px",
    borderRadius: "2px"
  },
  diffLineAdded: {
    backgroundColor: "#e8f5e9",
    color: "#1b5e20"
  },
  diffLineRemoved: {
    backgroundColor: "#ffebee",
    color: "#b71c1c",
    textDecoration: "line-through"
  },
  diffLineEqual: {
    color: "#666"
  },
  splitView: {
    display: "grid",
    gridTemplateColumns: "1fr 1fr",
    gap: "1px",
    backgroundColor: "#e5e5e5"
  },
  splitPane: {
    backgroundColor: "#fff",
    padding: "12px"
  },
  splitPaneTitle: {
    fontSize: "11px",
    fontWeight: 600,
    color: "#666",
    marginBottom: "8px",
    textTransform: "uppercase"
  },
  emptyState: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: "200px",
    color: "#666",
    fontSize: "14px"
  },
  filterBar: {
    display: "flex",
    alignItems: "center",
    gap: "12px",
    padding: "8px 16px",
    borderBottom: "1px solid #e5e5e5"
  },
  filterCheckbox: {
    display: "flex",
    alignItems: "center",
    gap: "4px",
    fontSize: "12px"
  }
};
function DiffViewer({ store }) {
  const diffState = useEditorState(store, (s) => s.diff);
  const document2 = useEditorState(store, (s) => s.document);
  const [showUnchanged, setShowUnchanged] = useState(false);
  const [expandedItems, setExpandedItems] = useState(/* @__PURE__ */ new Set());
  const [filter, setFilter] = useState("all");
  const snapshots = useMemo(() => {
    return [
      { id: "current", description: "Current state" },
      { id: "initial", description: "Initial state" }
    ];
  }, []);
  const diff = useMemo(() => {
    if (!diffState.isComparing || !document2) {
      return null;
    }
    return {
      fromSnapshotId: diffState.leftSnapshotId ?? "initial",
      toSnapshotId: diffState.rightSnapshotId ?? "current",
      blockDiffs: /* @__PURE__ */ new Map(),
      structuralChanges: [],
      summary: { added: 0, removed: 0, modified: 0, moved: 0, unchanged: document2.blocks.size }
    };
  }, [diffState.isComparing, diffState.leftSnapshotId, diffState.rightSnapshotId, document2]);
  const filteredDiffs = useMemo(() => {
    if (!diff) return [];
    const diffs = Array.from(diff.blockDiffs.values());
    return diffs.filter((d) => {
      if (!showUnchanged && d.changeType === "unchanged") return false;
      if (filter !== "all" && d.changeType !== filter) return false;
      return true;
    });
  }, [diff, showUnchanged, filter]);
  const handleStartCompare = useCallback(() => {
    store.startCompare("initial", "current");
  }, [store]);
  const handleStopCompare = useCallback(() => {
    store.stopCompare();
  }, [store]);
  const handleToggleExpand = useCallback((blockId) => {
    setExpandedItems((prev) => {
      const next = new Set(prev);
      if (next.has(blockId)) {
        next.delete(blockId);
      } else {
        next.add(blockId);
      }
      return next;
    });
  }, []);
  const handleApplyChange = useCallback(
    (blockId) => {
      store.applyChange(blockId);
    },
    [store]
  );
  const handleRejectChange = useCallback(
    (blockId) => {
      store.rejectChange(blockId);
    },
    [store]
  );
  const handleApplyAll = useCallback(() => {
    filteredDiffs.forEach((d) => {
      if (d.changeType !== "unchanged") {
        store.applyChange(d.blockId);
      }
    });
  }, [store, filteredDiffs]);
  if (!diffState.isComparing) {
    return /* @__PURE__ */ jsxs("div", { style: styles$1.container, children: [
      /* @__PURE__ */ jsx("div", { style: styles$1.header, children: /* @__PURE__ */ jsx("span", { style: styles$1.headerTitle, children: "Diff Viewer" }) }),
      /* @__PURE__ */ jsxs("div", { style: styles$1.emptyState, children: [
        /* @__PURE__ */ jsx("p", { children: "Select two snapshots to compare" }),
        /* @__PURE__ */ jsx(
          "button",
          {
            style: { ...styles$1.button, marginTop: "12px" },
            onClick: handleStartCompare,
            children: "Start Comparison"
          }
        )
      ] })
    ] });
  }
  return /* @__PURE__ */ jsxs("div", { style: styles$1.container, "data-testid": "diff-viewer", children: [
    /* @__PURE__ */ jsxs("div", { style: styles$1.header, children: [
      /* @__PURE__ */ jsx("span", { style: styles$1.headerTitle, children: "Comparing Changes" }),
      /* @__PURE__ */ jsxs("div", { style: styles$1.headerActions, children: [
        /* @__PURE__ */ jsx(
          "button",
          {
            style: { ...styles$1.button, ...styles$1.buttonPrimary },
            onClick: handleApplyAll,
            children: "Apply All"
          }
        ),
        /* @__PURE__ */ jsx("button", { style: styles$1.button, onClick: handleStopCompare, children: "Close" })
      ] })
    ] }),
    /* @__PURE__ */ jsxs("div", { style: styles$1.snapshotSelector, children: [
      /* @__PURE__ */ jsx("span", { children: "From:" }),
      /* @__PURE__ */ jsx("select", { style: styles$1.snapshotSelect, value: diffState.leftSnapshotId ?? "", children: snapshots.map((s) => /* @__PURE__ */ jsx("option", { value: s.id, children: s.description }, s.id)) }),
      /* @__PURE__ */ jsx("span", { children: "To:" }),
      /* @__PURE__ */ jsx("select", { style: styles$1.snapshotSelect, value: diffState.rightSnapshotId ?? "", children: snapshots.map((s) => /* @__PURE__ */ jsx("option", { value: s.id, children: s.description }, s.id)) })
    ] }),
    /* @__PURE__ */ jsxs("div", { style: styles$1.filterBar, children: [
      /* @__PURE__ */ jsxs("label", { style: styles$1.filterCheckbox, children: [
        /* @__PURE__ */ jsx(
          "input",
          {
            type: "checkbox",
            checked: showUnchanged,
            onChange: (e) => setShowUnchanged(e.target.checked)
          }
        ),
        "Show unchanged"
      ] }),
      /* @__PURE__ */ jsxs(
        "select",
        {
          value: filter,
          onChange: (e) => setFilter(e.target.value),
          style: { ...styles$1.snapshotSelect, minWidth: "100px" },
          children: [
            /* @__PURE__ */ jsx("option", { value: "all", children: "All changes" }),
            /* @__PURE__ */ jsx("option", { value: "added", children: "Added" }),
            /* @__PURE__ */ jsx("option", { value: "removed", children: "Removed" }),
            /* @__PURE__ */ jsx("option", { value: "modified", children: "Modified" }),
            /* @__PURE__ */ jsx("option", { value: "moved", children: "Moved" })
          ]
        }
      )
    ] }),
    diff && /* @__PURE__ */ jsxs("div", { style: styles$1.content, children: [
      /* @__PURE__ */ jsxs("div", { style: styles$1.summary, children: [
        /* @__PURE__ */ jsxs("div", { style: styles$1.summaryItem, children: [
          /* @__PURE__ */ jsxs("span", { style: { ...styles$1.summaryBadge, ...styles$1.badgeAdded }, children: [
            "+",
            diff.summary.added
          ] }),
          /* @__PURE__ */ jsx("span", { children: "Added" })
        ] }),
        /* @__PURE__ */ jsxs("div", { style: styles$1.summaryItem, children: [
          /* @__PURE__ */ jsxs("span", { style: { ...styles$1.summaryBadge, ...styles$1.badgeRemoved }, children: [
            "-",
            diff.summary.removed
          ] }),
          /* @__PURE__ */ jsx("span", { children: "Removed" })
        ] }),
        /* @__PURE__ */ jsxs("div", { style: styles$1.summaryItem, children: [
          /* @__PURE__ */ jsxs("span", { style: { ...styles$1.summaryBadge, ...styles$1.badgeModified }, children: [
            "~",
            diff.summary.modified
          ] }),
          /* @__PURE__ */ jsx("span", { children: "Modified" })
        ] }),
        /* @__PURE__ */ jsxs("div", { style: styles$1.summaryItem, children: [
          /* @__PURE__ */ jsxs("span", { style: { ...styles$1.summaryBadge, ...styles$1.badgeMoved }, children: [
            "↔",
            diff.summary.moved
          ] }),
          /* @__PURE__ */ jsx("span", { children: "Moved" })
        ] })
      ] }),
      /* @__PURE__ */ jsx("div", { style: styles$1.diffList, children: filteredDiffs.length === 0 ? /* @__PURE__ */ jsx("div", { style: styles$1.emptyState, children: /* @__PURE__ */ jsx("p", { children: "No changes to display" }) }) : filteredDiffs.map((blockDiff) => /* @__PURE__ */ jsx(
        DiffItem,
        {
          blockDiff,
          isExpanded: expandedItems.has(blockDiff.blockId),
          onToggle: () => handleToggleExpand(blockDiff.blockId),
          onApply: () => handleApplyChange(blockDiff.blockId),
          onReject: () => handleRejectChange(blockDiff.blockId)
        },
        blockDiff.blockId
      )) })
    ] })
  ] });
}
function DiffItem({
  blockDiff,
  isExpanded,
  onToggle,
  onApply,
  onReject
}) {
  var _a;
  const { blockId, changeType, oldBlock, newBlock, contentDiff } = blockDiff;
  const getBadgeStyle = () => {
    switch (changeType) {
      case "added":
        return styles$1.badgeAdded;
      case "removed":
        return styles$1.badgeRemoved;
      case "modified":
        return styles$1.badgeModified;
      case "moved":
        return styles$1.badgeMoved;
      default:
        return {};
    }
  };
  const getChangeLabel = () => {
    switch (changeType) {
      case "added":
        return "Added";
      case "removed":
        return "Removed";
      case "modified":
        return "Modified";
      case "moved":
        return "Moved";
      case "unchanged":
        return "Unchanged";
      default:
        return changeType;
    }
  };
  return /* @__PURE__ */ jsxs("div", { style: styles$1.diffItem, children: [
    /* @__PURE__ */ jsxs("div", { style: styles$1.diffItemHeader, onClick: onToggle, children: [
      /* @__PURE__ */ jsxs("div", { style: styles$1.diffItemHeaderLeft, children: [
        /* @__PURE__ */ jsx("span", { style: { ...styles$1.summaryBadge, ...getBadgeStyle() }, children: getChangeLabel() }),
        /* @__PURE__ */ jsx("span", { style: styles$1.diffItemId, children: truncateId(blockId) }),
        /* @__PURE__ */ jsx("span", { style: styles$1.diffItemType, children: ((_a = newBlock ?? oldBlock) == null ? void 0 : _a.type) ?? "unknown" })
      ] }),
      /* @__PURE__ */ jsxs("div", { style: styles$1.diffItemActions, children: [
        changeType !== "unchanged" && /* @__PURE__ */ jsxs(Fragment, { children: [
          /* @__PURE__ */ jsx(
            "button",
            {
              style: { ...styles$1.button, padding: "2px 8px", fontSize: "11px" },
              onClick: (e) => {
                e.stopPropagation();
                onApply();
              },
              children: "Apply"
            }
          ),
          /* @__PURE__ */ jsx(
            "button",
            {
              style: { ...styles$1.button, padding: "2px 8px", fontSize: "11px" },
              onClick: (e) => {
                e.stopPropagation();
                onReject();
              },
              children: "Reject"
            }
          )
        ] }),
        /* @__PURE__ */ jsx("span", { style: { fontSize: "12px", color: "#666" }, children: isExpanded ? "▼" : "▶" })
      ] })
    ] }),
    isExpanded && /* @__PURE__ */ jsx("div", { style: styles$1.diffItemContent, children: changeType === "modified" && contentDiff ? /* @__PURE__ */ jsx(TextDiffView, { textDiff: contentDiff }) : changeType === "added" && newBlock ? /* @__PURE__ */ jsxs("div", { style: { ...styles$1.diffLine, ...styles$1.diffLineAdded }, children: [
      "+ ",
      newBlock.content
    ] }) : changeType === "removed" && oldBlock ? /* @__PURE__ */ jsxs("div", { style: { ...styles$1.diffLine, ...styles$1.diffLineRemoved }, children: [
      "- ",
      oldBlock.content
    ] }) : /* @__PURE__ */ jsx(SplitBlockView, { oldBlock, newBlock }) })
  ] });
}
function TextDiffView({ textDiff }) {
  return /* @__PURE__ */ jsx("div", { children: textDiff.operations.map((op, index) => {
    let style = styles$1.diffLine;
    switch (op.type) {
      case "insert":
        style = { ...styles$1.diffLine, ...styles$1.diffLineAdded };
        break;
      case "delete":
        style = { ...styles$1.diffLine, ...styles$1.diffLineRemoved };
        break;
      default:
        style = { ...styles$1.diffLine, ...styles$1.diffLineEqual };
    }
    return /* @__PURE__ */ jsxs("span", { style, children: [
      op.type === "insert" && "+",
      op.type === "delete" && "-",
      op.text
    ] }, index);
  }) });
}
function SplitBlockView({ oldBlock, newBlock }) {
  return /* @__PURE__ */ jsxs("div", { style: styles$1.splitView, children: [
    /* @__PURE__ */ jsxs("div", { style: styles$1.splitPane, children: [
      /* @__PURE__ */ jsx("div", { style: styles$1.splitPaneTitle, children: "Before" }),
      oldBlock ? /* @__PURE__ */ jsx("pre", { style: { margin: 0, fontSize: "12px", whiteSpace: "pre-wrap" }, children: oldBlock.content }) : /* @__PURE__ */ jsx("span", { style: { color: "#999", fontStyle: "italic" }, children: "Not present" })
    ] }),
    /* @__PURE__ */ jsxs("div", { style: styles$1.splitPane, children: [
      /* @__PURE__ */ jsx("div", { style: styles$1.splitPaneTitle, children: "After" }),
      newBlock ? /* @__PURE__ */ jsx("pre", { style: { margin: 0, fontSize: "12px", whiteSpace: "pre-wrap" }, children: newBlock.content }) : /* @__PURE__ */ jsx("span", { style: { color: "#999", fontStyle: "italic" }, children: "Not present" })
    ] })
  ] });
}
function truncateId(id) {
  if (id.length <= 16) return id;
  return `${id.slice(0, 8)}...${id.slice(-4)}`;
}
const styles = {
  container: {
    display: "flex",
    flexDirection: "column",
    height: "100%",
    width: "100%",
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    fontSize: "14px",
    lineHeight: "1.5",
    color: "#1a1a1a",
    backgroundColor: "#ffffff"
  },
  toolbar: {
    display: "flex",
    alignItems: "center",
    gap: "8px",
    padding: "8px 16px",
    borderBottom: "1px solid #e5e5e5",
    backgroundColor: "#fafafa"
  },
  toolbarGroup: {
    display: "flex",
    alignItems: "center",
    gap: "4px"
  },
  toolbarButton: {
    padding: "4px 8px",
    border: "1px solid #d1d1d1",
    borderRadius: "4px",
    backgroundColor: "#ffffff",
    cursor: "pointer",
    fontSize: "12px",
    transition: "background-color 0.1s"
  },
  toolbarButtonActive: {
    backgroundColor: "#e3f2fd",
    borderColor: "#2196f3"
  },
  toolbarButtonDisabled: {
    opacity: 0.5,
    cursor: "not-allowed"
  },
  toolbarDivider: {
    width: "1px",
    height: "20px",
    backgroundColor: "#e5e5e5",
    margin: "0 8px"
  },
  content: {
    flex: 1,
    overflow: "auto",
    padding: "24px"
  },
  documentView: {
    maxWidth: "800px",
    margin: "0 auto"
  },
  splitView: {
    display: "grid",
    gridTemplateColumns: "1fr 1fr",
    gap: "16px",
    height: "100%"
  },
  splitPane: {
    overflow: "auto",
    border: "1px solid #e5e5e5",
    borderRadius: "4px"
  },
  emptyState: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: "200px",
    color: "#666"
  },
  statusBar: {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "4px 16px",
    borderTop: "1px solid #e5e5e5",
    backgroundColor: "#fafafa",
    fontSize: "12px",
    color: "#666"
  }
};
function Editor({
  document: initialDocument,
  config,
  store: externalStore,
  onChange,
  onSave,
  className,
  style
}) {
  const internalStore = useEditorStore(config);
  const store = externalStore ?? internalStore;
  const document2 = useEditorState(store, (s) => s.document);
  const view = useEditorState(store, (s) => s.view);
  const isDirty = useEditorState(store, (s) => s.isDirty);
  const history = useEditorState(store, (s) => s.history);
  const lastError = useEditorState(store, (s) => s.lastError);
  useKeyboardShortcuts(store, store.config.enableKeyboardShortcuts);
  useEffect(() => {
    if (initialDocument && !document2) {
      store.loadDocument(initialDocument);
    }
  }, [initialDocument, document2, store]);
  useEffect(() => {
    if (document2 && onChange) {
      onChange(document2);
    }
  }, [document2, onChange]);
  const handleViewChange = useCallback(
    (newView) => {
      store.setView(newView);
    },
    [store]
  );
  const handleUndo = useCallback(() => {
    if (history.canUndo) {
      store.undo();
    }
  }, [store, history.canUndo]);
  const handleRedo = useCallback(() => {
    if (history.canRedo) {
      store.redo();
    }
  }, [store, history.canRedo]);
  const handleSave = useCallback(async () => {
    if (document2 && onSave) {
      await onSave(document2);
    }
    await store.saveDocument();
  }, [store, document2, onSave]);
  const renderContent = () => {
    if (!document2) {
      return /* @__PURE__ */ jsxs("div", { style: styles.emptyState, children: [
        /* @__PURE__ */ jsx("p", { children: "No document loaded" }),
        /* @__PURE__ */ jsx(
          "button",
          {
            onClick: () => store.createDocument("New Document"),
            style: { ...styles.toolbarButton, marginTop: "8px" },
            children: "Create New Document"
          }
        )
      ] });
    }
    switch (view) {
      case "document":
        return /* @__PURE__ */ jsx("div", { style: styles.documentView, children: /* @__PURE__ */ jsx(DocumentView, { store }) });
      case "graph":
        return /* @__PURE__ */ jsx(GraphView, { store });
      case "diff":
        return /* @__PURE__ */ jsx(DiffViewer, { store });
      case "split":
        return /* @__PURE__ */ jsxs("div", { style: styles.splitView, children: [
          /* @__PURE__ */ jsx("div", { style: styles.splitPane, children: /* @__PURE__ */ jsx("div", { style: { padding: "16px" }, children: /* @__PURE__ */ jsx(DocumentView, { store }) }) }),
          /* @__PURE__ */ jsx("div", { style: styles.splitPane, children: /* @__PURE__ */ jsx(GraphView, { store }) })
        ] });
      default:
        return /* @__PURE__ */ jsx(DocumentView, { store });
    }
  };
  return /* @__PURE__ */ jsxs(
    "div",
    {
      className,
      style: { ...styles.container, ...style },
      "data-testid": "ucm-editor",
      children: [
        /* @__PURE__ */ jsxs("div", { style: styles.toolbar, children: [
          /* @__PURE__ */ jsxs("div", { style: styles.toolbarGroup, children: [
            /* @__PURE__ */ jsx(
              ViewButton,
              {
                label: "Document",
                view: "document",
                currentView: view,
                onClick: handleViewChange
              }
            ),
            /* @__PURE__ */ jsx(
              ViewButton,
              {
                label: "Graph",
                view: "graph",
                currentView: view,
                onClick: handleViewChange
              }
            ),
            /* @__PURE__ */ jsx(
              ViewButton,
              {
                label: "Diff",
                view: "diff",
                currentView: view,
                onClick: handleViewChange
              }
            ),
            /* @__PURE__ */ jsx(
              ViewButton,
              {
                label: "Split",
                view: "split",
                currentView: view,
                onClick: handleViewChange
              }
            )
          ] }),
          /* @__PURE__ */ jsx("div", { style: styles.toolbarDivider }),
          /* @__PURE__ */ jsxs("div", { style: styles.toolbarGroup, children: [
            /* @__PURE__ */ jsx(
              "button",
              {
                onClick: handleUndo,
                disabled: !history.canUndo,
                style: {
                  ...styles.toolbarButton,
                  ...history.canUndo ? {} : styles.toolbarButtonDisabled
                },
                title: "Undo (Cmd+Z)",
                children: "Undo"
              }
            ),
            /* @__PURE__ */ jsx(
              "button",
              {
                onClick: handleRedo,
                disabled: !history.canRedo,
                style: {
                  ...styles.toolbarButton,
                  ...history.canRedo ? {} : styles.toolbarButtonDisabled
                },
                title: "Redo (Cmd+Shift+Z)",
                children: "Redo"
              }
            )
          ] }),
          /* @__PURE__ */ jsx("div", { style: styles.toolbarDivider }),
          /* @__PURE__ */ jsx(
            "button",
            {
              onClick: handleSave,
              disabled: !isDirty,
              style: {
                ...styles.toolbarButton,
                ...isDirty ? {} : styles.toolbarButtonDisabled
              },
              title: "Save (Cmd+S)",
              children: "Save"
            }
          )
        ] }),
        /* @__PURE__ */ jsx("div", { style: styles.content, children: renderContent() }),
        /* @__PURE__ */ jsxs("div", { style: styles.statusBar, children: [
          /* @__PURE__ */ jsxs("div", { children: [
            document2 ? `${document2.blocks.size} blocks` : "No document",
            isDirty && " (unsaved changes)"
          ] }),
          /* @__PURE__ */ jsx("div", { children: lastError && /* @__PURE__ */ jsxs("span", { style: { color: "#d32f2f" }, children: [
            "Error: ",
            lastError.message
          ] }) })
        ] })
      ]
    }
  );
}
function ViewButton({ label, view, currentView, onClick }) {
  const isActive = view === currentView;
  return /* @__PURE__ */ jsx(
    "button",
    {
      onClick: () => onClick(view),
      style: {
        ...styles.toolbarButton,
        ...isActive ? styles.toolbarButtonActive : {}
      },
      children: label
    }
  );
}
function DocumentView({ store }) {
  const document2 = useEditorState(store, (s) => s.document);
  if (!document2) {
    return /* @__PURE__ */ jsx("div", { children: "No document" });
  }
  const rootBlock = document2.blocks.get(document2.root);
  if (!rootBlock) {
    return /* @__PURE__ */ jsx("div", { children: "Invalid document structure" });
  }
  return /* @__PURE__ */ jsxs("div", { "data-testid": "document-view", children: [
    rootBlock.children.map((childId) => {
      const child = document2.blocks.get(childId);
      if (!child) return null;
      return /* @__PURE__ */ jsx(
        BlockRenderer,
        {
          block: child,
          document: document2,
          store,
          depth: 0,
          path: [document2.root]
        },
        childId
      );
    }),
    rootBlock.children.length === 0 && /* @__PURE__ */ jsxs("div", { style: styles.emptyState, children: [
      /* @__PURE__ */ jsx("p", { children: "Empty document" }),
      /* @__PURE__ */ jsx(
        "button",
        {
          onClick: () => store.addBlock(document2.root, "Start typing..."),
          style: { ...styles.toolbarButton, marginTop: "8px" },
          children: "Add Block"
        }
      )
    ] })
  ] });
}
export {
  BlockEditor,
  BlockRenderer,
  DEFAULT_EDITOR_CONFIG,
  DiffViewer,
  Editor,
  EditorError,
  Errors,
  GraphView,
  Logger,
  MetadataTooltip,
  SelectionManager,
  SimpleEventEmitter,
  andThen,
  computeDocumentDiff,
  configureLogger,
  consoleHandler,
  createBlockSelection,
  createBufferHandler,
  createEditorStore,
  createEmptySelection,
  createMultiBlockSelection,
  createTextSelection,
  err,
  expandSelection,
  formatTextDiff,
  getBlockOrder$1 as getBlockOrder,
  getBlockTextDiff,
  getBlocksByChangeType,
  getChangedBlocks,
  getFirstChildBlock,
  getNextBlock,
  getNextSibling,
  getParentBlock,
  getPreviousBlock,
  getPreviousSibling,
  getPrimarySelectedBlock,
  getSelectedBlockIds,
  getSiblingBlocks,
  getTextSelection,
  hasBlockChanged,
  hasDiffChanges,
  isBlockFocused,
  isBlockSelected,
  isBlockSelectionType,
  isSelectionEmpty,
  isTextSelection,
  logger$2 as logger,
  map,
  ok,
  unwrap,
  unwrapOr,
  useBlockActions,
  useDocument,
  useDrag,
  useEditActions,
  useEditorEvent,
  useEditorState,
  useEditorStore,
  useHistory,
  useKeyboardShortcuts,
  useSelection,
  useView
};
//# sourceMappingURL=index.js.map
