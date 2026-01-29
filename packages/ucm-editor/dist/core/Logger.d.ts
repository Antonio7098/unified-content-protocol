/**
 * Structured logging for UCM Editor.
 *
 * Provides consistent, level-based logging with context support.
 */
export type LogLevel = 'debug' | 'info' | 'warn' | 'error';
export interface LogEntry {
    level: LogLevel;
    message: string;
    timestamp: Date;
    context?: string;
    data?: Record<string, unknown>;
    error?: Error;
}
export type LogHandler = (entry: LogEntry) => void;
/**
 * Console log handler that outputs to console with appropriate methods.
 */
export declare const consoleHandler: LogHandler;
/**
 * In-memory buffer handler for testing and inspection.
 */
export declare function createBufferHandler(maxEntries?: number): {
    handler: LogHandler;
    getEntries: () => LogEntry[];
    clear: () => void;
};
export interface LoggerConfig {
    level: LogLevel;
    handlers: LogHandler[];
    context?: string;
}
/**
 * Structured logger for UCM Editor.
 *
 * @example
 * ```typescript
 * const logger = new Logger({ context: 'EditorStore' })
 *
 * logger.info('Document loaded', { blockCount: 42 })
 * logger.warn('Large document detected', { blockCount: 1000 })
 * logger.error('Failed to save', { error: err })
 *
 * // Child logger with additional context
 * const blockLogger = logger.child('BlockEditor')
 * blockLogger.debug('Editing block', { blockId: 'blk_123' })
 * ```
 */
export declare class Logger {
    private config;
    private context?;
    constructor(config?: Partial<LoggerConfig>);
    /**
     * Create a child logger with additional context.
     */
    child(context: string): Logger;
    /**
     * Set the log level.
     */
    setLevel(level: LogLevel): void;
    /**
     * Get the current log level.
     */
    getLevel(): LogLevel;
    /**
     * Add a log handler.
     */
    addHandler(handler: LogHandler): void;
    /**
     * Remove a log handler.
     */
    removeHandler(handler: LogHandler): void;
    /**
     * Clear all handlers.
     */
    clearHandlers(): void;
    /**
     * Check if a level would be logged.
     */
    isLevelEnabled(level: LogLevel): boolean;
    /**
     * Log a debug message.
     */
    debug(message: string, data?: Record<string, unknown>): void;
    /**
     * Log an info message.
     */
    info(message: string, data?: Record<string, unknown>): void;
    /**
     * Log a warning message.
     */
    warn(message: string, data?: Record<string, unknown>): void;
    /**
     * Log an error message.
     */
    error(message: string, error?: Error | Record<string, unknown>, data?: Record<string, unknown>): void;
    /**
     * Log a message with timing.
     */
    time<T>(label: string, fn: () => T): T;
    /**
     * Log a message with async timing.
     */
    timeAsync<T>(label: string, fn: () => Promise<T>): Promise<T>;
    /**
     * Create a group of related log messages.
     */
    group(label: string): {
        end: () => void;
    };
    private log;
}
/**
 * Global logger instance for the editor.
 */
export declare const logger: Logger;
/**
 * Configure the global logger.
 */
export declare function configureLogger(config: Partial<LoggerConfig>): void;
