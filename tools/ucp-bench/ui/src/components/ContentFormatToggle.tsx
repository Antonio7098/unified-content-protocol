import type { ComponentPropsWithoutRef, ReactNode } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import type { Components } from 'react-markdown'

export type ContentFormat = 'md-raw' | 'md-formatted' | 'ucm' | 'ucl'

interface ContentFormatToggleProps {
  value: ContentFormat
  onChange: (value: ContentFormat) => void
  className?: string
  showMdRaw?: boolean
  showMdFormatted?: boolean
  showUcm?: boolean
  showUcl?: boolean
}

export function ContentFormatToggle({
  value,
  onChange,
  className = '',
  showMdRaw = true,
  showMdFormatted = true,
  showUcm = true,
  showUcl = true,
}: ContentFormatToggleProps) {
  const formats: { id: ContentFormat; label: string }[] = []

  if (showMdRaw) formats.push({ id: 'md-raw', label: 'MD Raw' })
  if (showMdFormatted) formats.push({ id: 'md-formatted', label: 'MD' })
  if (showUcm) formats.push({ id: 'ucm', label: 'UCM' })
  if (showUcl) formats.push({ id: 'ucl', label: 'UCL' })

  return (
    <div className={`flex items-center gap-1 bg-white rounded-lg border border-gray-200 p-1 ${className}`}>
      {formats.map((format) => (
        <button
          key={format.id}
          onClick={() => onChange(format.id)}
          className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
            value === format.id
              ? 'bg-blue-50 text-blue-700'
              : 'text-gray-600 hover:text-gray-900'
          }`}
        >
          {format.label}
        </button>
      ))}
    </div>
  )
}

interface ContentViewerProps {
  format: ContentFormat
  mdRaw?: string
  ucm?: unknown
  ucl?: string
  className?: string
}

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null && !Array.isArray(value)

const extractLeadingMetadata = (markdown: string | undefined) => {
  if (!markdown) return { metadata: null, body: '' }

  const leadingWhitespace = markdown.match(/^\s*/)
  const offset = leadingWhitespace ? leadingWhitespace[0].length : 0
  const trimmedStart = markdown.slice(offset)
  if (!trimmedStart.startsWith('{') && !trimmedStart.startsWith('[')) {
    return { metadata: null, body: markdown }
  }

  let depth = 0
  let inString = false
  let escape = false
  let endIndex = -1

  for (let i = 0; i < trimmedStart.length; i++) {
    const char = trimmedStart[i]

    if (escape) {
      escape = false
      continue
    }

    if (char === '\\') {
      escape = true
      continue
    }

    if (char === '"') {
      inString = !inString
      continue
    }

    if (inString) continue

    if (char === '{' || char === '[') {
      depth += 1
    } else if (char === '}' || char === ']') {
      depth -= 1
      if (depth === 0) {
        endIndex = i + 1
        break
      }
    }
  }

  if (endIndex === -1) {
    return { metadata: null, body: markdown }
  }

  const candidate = trimmedStart.slice(0, endIndex)
  try {
    const parsed = JSON.parse(candidate)
    if (!isRecord(parsed)) {
      return { metadata: null, body: markdown }
    }

    const rest = trimmedStart.slice(endIndex).replace(/^\s*/, '')
    return { metadata: parsed, body: rest }
  } catch {
    return { metadata: null, body: markdown }
  }
}

const toStringArray = (value: unknown) => {
  if (Array.isArray(value)) {
    const mapped = value.filter((item): item is string => typeof item === 'string' && item.trim().length > 0).map((item) => item.trim())
    return mapped.length ? mapped : null
  }
  if (typeof value === 'string') {
    const parts = value
      .split(',')
      .map((part) => part.trim())
      .filter(Boolean)
    return parts.length ? parts : null
  }
  return null
}

const toStringValue = (value: unknown) => (typeof value === 'string' && value.trim().length > 0 ? value.trim() : null)

const formatTimestamp = (value: unknown) => {
  const text = toStringValue(value)
  if (!text) return null
  const date = new Date(text)
  if (Number.isNaN(date.getTime())) return text
  return date.toLocaleString(undefined, { dateStyle: 'medium', timeStyle: 'short' })
}

const renderValueNode = (value: unknown) => {
  if (value === null || value === undefined) {
    return <span className="text-gray-500">—</span>
  }
  if (typeof value === 'string' || typeof value === 'number' || typeof value === 'boolean') {
    return <span>{String(value)}</span>
  }
  if (Array.isArray(value)) {
    const items = value
      .map((item) => (typeof item === 'string' ? item : null))
      .filter((item): item is string => Boolean(item && item.trim().length > 0))
    if (items.length === 0) {
      return <span className="text-gray-500">—</span>
    }
    return (
      <div className="flex flex-wrap gap-1.5">
        {items.map((item) => (
          <span key={item} className="px-2 py-0.5 rounded-full bg-gray-100 text-gray-700 text-xs font-medium">
            {item}
          </span>
        ))}
      </div>
    )
  }
  return (
    <div className="bg-gray-900 text-gray-100 rounded-lg p-3 text-xs font-mono whitespace-pre-wrap overflow-auto">
      {JSON.stringify(value, null, 2)}
    </div>
  )
}

const MetadataPanel = ({ metadata }: { metadata: Record<string, unknown> }) => {
  const title = toStringValue(metadata.title)
  const version = toStringValue(metadata.version)
  const curator = toStringValue(metadata.curator)
  const updated = formatTimestamp(metadata.updated)
  const tags = toStringArray(metadata.tags)
  const capabilities = toStringArray(metadata.capabilities)

  const reservedKeys = new Set(['title', 'version', 'curator', 'updated', 'tags', 'capabilities'])
  const extras = Object.entries(metadata).filter(([key]) => !reservedKeys.has(key))

  return (
    <div className="bg-gray-50 border border-gray-200 rounded-xl p-4 space-y-4">
      {title && (
        <div>
          <p className="text-xs uppercase tracking-wide text-gray-500 mb-0.5">Document</p>
          <div className="flex flex-wrap items-center gap-2">
            <h3 className="text-lg font-semibold text-gray-900">{title}</h3>
            {version && <span className="px-2 py-0.5 text-xs font-semibold rounded-full bg-blue-100 text-blue-700">v{version}</span>}
          </div>
        </div>
      )}

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm">
        {curator && (
          <div>
            <p className="text-xs uppercase tracking-wide text-gray-500">Curator</p>
            <p className="text-gray-900">{curator}</p>
          </div>
        )}
        {updated && (
          <div>
            <p className="text-xs uppercase tracking-wide text-gray-500">Last Updated</p>
            <p className="text-gray-900">{updated}</p>
          </div>
        )}
      </div>

      {capabilities && (
        <div>
          <p className="text-xs uppercase tracking-wide text-gray-500 mb-1.5">Capabilities</p>
          <div className="flex flex-wrap gap-1.5">
            {capabilities.map((capability) => (
              <span key={capability} className="px-2 py-0.5 text-xs font-medium rounded-full bg-emerald-50 text-emerald-700">
                {capability}
              </span>
            ))}
          </div>
        </div>
      )}

      {tags && (
        <div>
          <p className="text-xs uppercase tracking-wide text-gray-500 mb-1.5">Tags</p>
          <div className="flex flex-wrap gap-1.5">
            {tags.map((tag) => (
              <span key={tag} className="px-2 py-0.5 text-xs font-medium rounded-full bg-indigo-50 text-indigo-700">
                {tag}
              </span>
            ))}
          </div>
        </div>
      )}

      {extras.length > 0 && (
        <div className="space-y-2">
          <p className="text-xs uppercase tracking-wide text-gray-500">Additional Metadata</p>
          <dl className="space-y-2 text-sm">
            {extras.map(([key, value]) => (
              <div key={key} className="bg-white rounded-lg border border-gray-200 p-2.5">
                <dt className="text-xs uppercase tracking-wide text-gray-500">{key}</dt>
                <dd className="mt-1 text-gray-900">{renderValueNode(value)}</dd>
              </div>
            ))}
          </dl>
        </div>
      )}
    </div>
  )
}

const isLikelyJson = (value: string) => {
  const trimmed = value.trim()
  if ((!trimmed.startsWith('{') || !trimmed.endsWith('}')) && (!trimmed.startsWith('[') || !trimmed.endsWith(']'))) {
    return false
  }
  try {
    JSON.parse(trimmed)
    return true
  } catch {
    return false
  }
}

const prettifyJson = (value: string) => {
  try {
    return JSON.stringify(JSON.parse(value), null, 2)
  } catch {
    return value
  }
}

const extractText = (children: ReactNode | ReactNode[]) => {
  if (Array.isArray(children)) {
    if (children.length !== 1) return null
    return typeof children[0] === 'string' ? children[0] : null
  }
  return typeof children === 'string' ? children : null
}

const MarkdownComponents: Components = {
  table({ children }) {
    return (
      <div className="overflow-auto rounded-lg border border-gray-200">
        <table className="w-full text-sm [&_th]:text-left [&_th]:font-semibold [&_th]:bg-gray-50 [&_td]:align-top [&_td]:py-2 [&_td]:pr-4">
          {children}
        </table>
      </div>
    )
  },
  a({ children, ...props }) {
    return (
      <a className="text-blue-600 underline underline-offset-2" target="_blank" rel="noreferrer" {...props}>
        {children}
      </a>
    )
  },
  code(props) {
    const { inline, className, children, ...rest } = props as ComponentPropsWithoutRef<'code'> & {
      inline?: boolean
      children?: ReactNode
    }
    if (inline) {
      return (
        <code className={`px-1.5 py-0.5 rounded bg-slate-100 text-slate-900 ${className || ''}`} {...rest}>
          {children}
        </code>
      )
    }
    return (
      <pre className="bg-gray-900 text-gray-100 p-3 rounded-lg text-sm overflow-auto">
        <code className={className} {...rest}>
          {children}
        </code>
      </pre>
    )
  },
  p({ children, ...props }) {
    const text = extractText(children)
    if (text && isLikelyJson(text)) {
      return (
        <div className="bg-gray-900 text-gray-100 rounded-lg p-4 text-xs font-mono overflow-auto" {...props}>
          <pre className="whitespace-pre-wrap">{prettifyJson(text)}</pre>
        </div>
      )
    }
    return <p {...props}>{children}</p>
  },
}

export function ContentViewer({
  format,
  mdRaw,
  ucm,
  ucl,
  className = '',
}: ContentViewerProps) {
  if (format === 'md-raw') {
    return (
      <div className={`prose prose-sm max-w-none overflow-auto ${className}`}>
        <div className="bg-white p-6 rounded-lg">
          <div className="whitespace-pre-wrap">{mdRaw || 'No markdown available'}</div>
        </div>
      </div>
    )
  }

  if (format === 'md-formatted') {
    const markdownSource = mdRaw || ''
    const { metadata, body } = extractLeadingMetadata(markdownSource)
    const markdownToRender = metadata ? body : markdownSource
    const hasBody = markdownToRender.trim().length > 0

    return (
      <div className={`overflow-auto ${className}`}>
        <div className="flex flex-col gap-4">
          {metadata && <MetadataPanel metadata={metadata} />}
          <div className="prose prose-sm max-w-none">
            <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-100">
              <ReactMarkdown remarkPlugins={[remarkGfm]} components={MarkdownComponents}>
                {hasBody ? markdownToRender : 'No markdown available'}
              </ReactMarkdown>
            </div>
          </div>
        </div>
      </div>
    )
  }

  if (format === 'ucm') {
    return (
      <div className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-auto">
        <pre className="text-sm font-mono whitespace-pre-wrap">
          {ucm ? JSON.stringify(ucm, null, 2) : 'No UCM available'}
        </pre>
      </div>
    )
  }

  if (format === 'ucl') {
    return (
      <div className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-auto">
        <pre className="text-sm whitespace-pre-wrap font-mono">
          {ucl || 'No UCL available'}
        </pre>
      </div>
    )
  }

  return null
}
