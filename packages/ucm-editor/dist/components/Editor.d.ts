/**
 * Editor - Main UCM Editor component.
 *
 * Provides the top-level editor container with view switching
 * and keyboard shortcut handling.
 */
import React from 'react';
import type { Document } from 'ucp-content';
import type { EditorStoreInstance } from '../core/EditorStore.js';
import type { EditorConfig } from '../types/editor.js';
export interface EditorProps {
    /** Initial document to load */
    document?: Document;
    /** Editor configuration */
    config?: Partial<EditorConfig>;
    /** Custom store instance (for external state management) */
    store?: EditorStoreInstance;
    /** Callback when document changes */
    onChange?: (document: Document) => void;
    /** Callback when document is saved */
    onSave?: (document: Document) => Promise<void>;
    /** Additional class name */
    className?: string;
    /** Additional styles */
    style?: React.CSSProperties;
}
/**
 * Main UCM Editor component.
 *
 * @example
 * ```tsx
 * import { Editor } from 'ucm-editor'
 * import { parseMarkdown } from 'ucp-content'
 *
 * function App() {
 *   const doc = parseMarkdown('# Hello\n\nWorld')
 *
 *   return (
 *     <Editor
 *       document={doc}
 *       onChange={(doc) => console.log('Changed:', doc)}
 *     />
 *   )
 * }
 * ```
 */
export declare function Editor({ document: initialDocument, config, store: externalStore, onChange, onSave, className, style, }: EditorProps): React.ReactElement;
export default Editor;
