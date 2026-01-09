import { useQuery } from '@tanstack/react-query'
import { useState } from 'react'
import { fetchDocument, fetchDocuments, fetchDocumentById } from '../api/client'
import { ContentFormat, ContentViewer } from '../components/ContentFormatToggle'
import { FileText } from 'lucide-react'

export default function DocumentViewer() {
  const { data: documents, isLoading: docsLoading } = useQuery({
    queryKey: ['documents'],
    queryFn: fetchDocuments,
  })
  const [selectedDocId, setSelectedDocId] = useState<string | null>(null)
  const [format, setFormat] = useState<ContentFormat>('md-formatted')

  const { data: docData, isLoading, error } = useQuery({
    queryKey: ['document', selectedDocId],
    queryFn: () => fetchDocumentById(selectedDocId!),
    enabled: !!selectedDocId,
  })

  const { data: defaultDoc, isLoading: defaultLoading } = useQuery({
    queryKey: ['document'],
    queryFn: fetchDocument,
    enabled: !selectedDocId,
  })

  const displayData = docData || defaultDoc
  const isDocLoading = selectedDocId ? isLoading : defaultLoading

  if (docsLoading) {
    return <div className="text-gray-500">Loading documents...</div>
  }

  const tabs: { id: ContentFormat; label: string; description: string }[] = [
    { id: 'md-raw', label: 'MD Raw', description: 'Original markdown source' },
    { id: 'md-formatted', label: 'MD Rendered', description: 'Rendered markdown view' },
    { id: 'ucm', label: 'UCM JSON', description: 'Canonical graph definition used for execution' },
    { id: 'ucl', label: 'LLM Prompt View', description: 'Token-optimized adjacency list sent to the model' },
  ]

  return (
    <div className="space-y-6 max-w-full overflow-hidden">
      <header className="flex items-start justify-between">
        <div>
          <p className="text-sm uppercase tracking-wide text-gray-500">Benchmark Document</p>
          <h1 className="text-2xl font-bold text-gray-900">Document Explorer</h1>
          <p className="text-gray-500 mt-1">
            Select a document to view its structure and content in various formats.
          </p>
        </div>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-[260px_minmax(0,1fr)] gap-6 lg:gap-8">
        <aside className="bg-white border border-gray-200 rounded-xl p-4 h-fit">
          <h2 className="text-sm font-semibold text-gray-900 mb-3">Documents</h2>
          <div className="space-y-1">
            <button
              onClick={() => setSelectedDocId(null)}
              className={`w-full flex items-center gap-2 px-3 py-2 rounded-lg text-left text-sm transition-colors ${
                selectedDocId === null
                  ? 'bg-blue-50 text-blue-700'
                  : 'text-gray-600 hover:bg-gray-50'
              }`}
            >
              <FileText size={16} />
              <span className="truncate">Default Document</span>
            </button>
            {documents?.map((doc) => (
              <button
                key={doc.id}
                onClick={() => setSelectedDocId(doc.id)}
                className={`w-full flex items-center gap-2 px-3 py-2 rounded-lg text-left text-sm transition-colors ${
                  selectedDocId === doc.id
                    ? 'bg-blue-50 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-50'
                }`}
              >
                <FileText size={16} />
                <span className="truncate">{doc.name}</span>
              </button>
            ))}
          </div>
        </aside>

        <section className="bg-white border border-gray-200 rounded-xl min-w-0">
          {isDocLoading ? (
            <div className="p-6 text-gray-500">Loading document...</div>
          ) : error || !displayData ? (
            <div className="p-6 text-red-600">Unable to load document.</div>
          ) : (
            <>
              <div className="border-b border-gray-200 flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                <div className="-mb-px flex overflow-x-auto">
                  {tabs.map((tab) => {
                    const isActive = format === tab.id
                    return (
                      <button
                        key={tab.id}
                        onClick={() => setFormat(tab.id)}
                        className={`px-4 py-3 text-sm font-medium border-b-2 transition whitespace-nowrap ${
                          isActive
                            ? 'text-blue-600 border-blue-500 bg-white'
                            : 'text-gray-500 border-transparent hover:text-gray-700 hover:border-gray-300'
                        }`}
                      >
                        {tab.label}
                      </button>
                    )
                  })}
                </div>
                <div className="px-4 py-2 text-sm text-gray-500">
                  {displayData.snapshot.block_count} blocks
                </div>
              </div>
              <div className="p-6 space-y-3">
                <p className="text-sm text-gray-500">
                  {tabs.find((t) => t.id === format)?.description}
                </p>
                <ContentViewer
                  format={format}
                  mdRaw={displayData.markdown}
                  ucm={displayData.ucm}
                  ucl={displayData.description}
                  className="max-h-[520px] w-full"
                />
              </div>
            </>
          )}
        </section>
      </div>
    </div>
  )
}
