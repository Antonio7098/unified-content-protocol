/**
 * Structured logging for UCM Editor.
 *
 * Provides consistent, level-based logging with context support.
 */

// =============================================================================
// LOG LEVELS
// =============================================================================

export type LogLevel = 'debug' | 'info' | 'warn' | 'error'

const LOG_LEVEL_PRIORITY: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
}

// =============================================================================
// LOG ENTRY
// =============================================================================

export interface LogEntry {
  level: LogLevel
  message: string
  timestamp: Date
  context?: string
  data?: Record<string, unknown>
  error?: Error
}

// =============================================================================
// LOG HANDLER
// =============================================================================

export type LogHandler = (entry: LogEntry) => void

// =============================================================================
// CONSOLE FORMATTER
// =============================================================================

const LEVEL_COLORS: Record<LogLevel, string> = {
  debug: '\x1b[36m', // Cyan
  info: '\x1b[32m', // Green
  warn: '\x1b[33m', // Yellow
  error: '\x1b[31m', // Red
}

const RESET = '\x1b[0m'

function formatLogEntry(entry: LogEntry, useColors: boolean): string {
  const time = entry.timestamp.toISOString().slice(11, 23) // HH:mm:ss.SSS
  const level = entry.level.toUpperCase().padEnd(5)
  const context = entry.context ? `[${entry.context}]` : ''

  if (useColors) {
    const color = LEVEL_COLORS[entry.level]
    return `${color}${time} ${level}${RESET} ${context} ${entry.message}`
  }

  return `${time} ${level} ${context} ${entry.message}`
}

// =============================================================================
// DEFAULT HANDLERS
// =============================================================================

/**
 * Console log handler that outputs to console with appropriate methods.
 */
export const consoleHandler: LogHandler = (entry: LogEntry) => {
  const formatted = formatLogEntry(entry, typeof window === 'undefined')
  const args: unknown[] = [formatted]

  if (entry.data && Object.keys(entry.data).length > 0) {
    args.push(entry.data)
  }

  if (entry.error) {
    args.push(entry.error)
  }

  switch (entry.level) {
    case 'debug':
      console.debug(...args)
      break
    case 'info':
      console.info(...args)
      break
    case 'warn':
      console.warn(...args)
      break
    case 'error':
      console.error(...args)
      break
  }
}

/**
 * In-memory buffer handler for testing and inspection.
 */
export function createBufferHandler(maxEntries = 1000): {
  handler: LogHandler
  getEntries: () => LogEntry[]
  clear: () => void
} {
  const entries: LogEntry[] = []

  return {
    handler: (entry: LogEntry) => {
      entries.push(entry)
      if (entries.length > maxEntries) {
        entries.shift()
      }
    },
    getEntries: () => [...entries],
    clear: () => {
      entries.length = 0
    },
  }
}

// =============================================================================
// LOGGER CLASS
// =============================================================================

export interface LoggerConfig {
  level: LogLevel
  handlers: LogHandler[]
  context?: string
}

const DEFAULT_CONFIG: LoggerConfig = {
  level: 'info',
  handlers: [consoleHandler],
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
export class Logger {
  private config: LoggerConfig
  private context?: string

  constructor(config: Partial<LoggerConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config }
    this.context = config.context
  }

  /**
   * Create a child logger with additional context.
   */
  child(context: string): Logger {
    const childContext = this.context ? `${this.context}:${context}` : context
    return new Logger({
      ...this.config,
      context: childContext,
    })
  }

  /**
   * Set the log level.
   */
  setLevel(level: LogLevel): void {
    this.config.level = level
  }

  /**
   * Get the current log level.
   */
  getLevel(): LogLevel {
    return this.config.level
  }

  /**
   * Add a log handler.
   */
  addHandler(handler: LogHandler): void {
    this.config.handlers.push(handler)
  }

  /**
   * Remove a log handler.
   */
  removeHandler(handler: LogHandler): void {
    const index = this.config.handlers.indexOf(handler)
    if (index !== -1) {
      this.config.handlers.splice(index, 1)
    }
  }

  /**
   * Clear all handlers.
   */
  clearHandlers(): void {
    this.config.handlers = []
  }

  /**
   * Check if a level would be logged.
   */
  isLevelEnabled(level: LogLevel): boolean {
    return LOG_LEVEL_PRIORITY[level] >= LOG_LEVEL_PRIORITY[this.config.level]
  }

  /**
   * Log a debug message.
   */
  debug(message: string, data?: Record<string, unknown>): void {
    this.log('debug', message, data)
  }

  /**
   * Log an info message.
   */
  info(message: string, data?: Record<string, unknown>): void {
    this.log('info', message, data)
  }

  /**
   * Log a warning message.
   */
  warn(message: string, data?: Record<string, unknown>): void {
    this.log('warn', message, data)
  }

  /**
   * Log an error message.
   */
  error(message: string, error?: Error | Record<string, unknown>, data?: Record<string, unknown>): void {
    if (error instanceof Error) {
      this.log('error', message, data, error)
    } else {
      this.log('error', message, error)
    }
  }

  /**
   * Log a message with timing.
   */
  time<T>(label: string, fn: () => T): T {
    const start = performance.now()
    try {
      const result = fn()
      const duration = performance.now() - start
      this.debug(`${label} completed`, { durationMs: duration.toFixed(2) })
      return result
    } catch (error) {
      const duration = performance.now() - start
      this.error(`${label} failed`, error instanceof Error ? error : undefined, {
        durationMs: duration.toFixed(2),
      })
      throw error
    }
  }

  /**
   * Log a message with async timing.
   */
  async timeAsync<T>(label: string, fn: () => Promise<T>): Promise<T> {
    const start = performance.now()
    try {
      const result = await fn()
      const duration = performance.now() - start
      this.debug(`${label} completed`, { durationMs: duration.toFixed(2) })
      return result
    } catch (error) {
      const duration = performance.now() - start
      this.error(`${label} failed`, error instanceof Error ? error : undefined, {
        durationMs: duration.toFixed(2),
      })
      throw error
    }
  }

  /**
   * Create a group of related log messages.
   */
  group(label: string): { end: () => void } {
    this.debug(`${label} started`)
    const start = performance.now()
    return {
      end: () => {
        const duration = performance.now() - start
        this.debug(`${label} ended`, { durationMs: duration.toFixed(2) })
      },
    }
  }

  private log(level: LogLevel, message: string, data?: Record<string, unknown>, error?: Error): void {
    if (!this.isLevelEnabled(level)) {
      return
    }

    const entry: LogEntry = {
      level,
      message,
      timestamp: new Date(),
      context: this.context,
      data,
      error,
    }

    for (const handler of this.config.handlers) {
      try {
        handler(entry)
      } catch (e) {
        // Prevent handler errors from breaking the application
        console.error('Log handler error:', e)
      }
    }
  }
}

// =============================================================================
// GLOBAL LOGGER
// =============================================================================

/**
 * Global logger instance for the editor.
 */
export const logger = new Logger({ context: 'UCMEditor' })

/**
 * Configure the global logger.
 */
export function configureLogger(config: Partial<LoggerConfig>): void {
  if (config.level) {
    logger.setLevel(config.level)
  }
  if (config.handlers) {
    logger.clearHandlers()
    config.handlers.forEach((h) => logger.addHandler(h))
  }
}
