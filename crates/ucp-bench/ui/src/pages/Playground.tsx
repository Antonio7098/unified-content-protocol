import { useState, useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import {
  Send,
  FileText,
  CheckCircle2,
  XCircle,
  Clock,
  Code,
  FileJson,
  Eye,
  MessageSquare,
  Loader2
} from 'lucide-react'
import {
  fetchPlaygroundDocuments,
  fetchPlaygroundDocument,
  sendPlaygroundChat,
  fetchProviders,
  PlaygroundChatResponse
} from '../api/client'

type DocumentView = 'markdown' | 'udm'

interface ChatMessage {
  id: string
  timestamp: string
  userMessage: string
  response: PlaygroundChatResponse
}

export default function Playground() {
  const [selectedDocumentId, setSelectedDocumentId] = useState<string>('')
  const [selectedProviderId, setSelectedProviderId] = useState<string>('')
  const [selectedModelId, setSelectedModelId] = useState<string>('')
  const [message, setMessage] = useState('')
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [documentView, setDocumentView] = useState<DocumentView>('markdown')
  const [executeCommands, setExecuteCommands] = useState(false)
  const [selectedMessage, setSelectedMessage] = useState<ChatMessage | null>(null)
  const [showMessageDetail, setShowMessageDetail] = useState(false)

  const { data: documents } = useQuery({
    queryKey: ['playground-documents'],
    queryFn: fetchPlaygroundDocuments
  })

  const { data: providers } = useQuery({
    queryKey: ['providers'],
    queryFn: fetchProviders
  })

  const { data: currentDocument } = useQuery({
    queryKey: ['playground-document', selectedDocumentId],
    queryFn: () => fetchPlaygroundDocument(selectedDocumentId),
    enabled: !!selectedDocumentId
  })

  useEffect(() => {
    if (documents && documents.length > 0 && !selectedDocumentId) {
      setSelectedDocumentId(documents[0].id)
    }
  }, [documents, selectedDocumentId])

  useEffect(() => {
    if (providers && providers.length > 0 && !selectedProviderId) {
      const availableProvider = providers.find((p: any) => p.available) || providers[0]
      setSelectedProviderId(availableProvider.id)
      if (availableProvider.models.length > 0) {
        setSelectedModelId(availableProvider.models[0].id)
      }
    }
  }, [providers, selectedProviderId])

  useEffect(() => {
    if (selectedProviderId && providers) {
      const provider = providers.find((p: any) => p.id === selectedProviderId)
      if (provider && provider.models.length > 0) {
        setSelectedModelId(provider.models[0].id)
      }
    }
  }, [selectedProviderId, providers])

  const selectedProvider = providers?.find((p: any) => p.id === selectedProviderId)

  const handleSendMessage = async () => {
    if (!message.trim() || !selectedDocumentId || !selectedProviderId || !selectedModelId) return

    setIsLoading(true)
    try {
      const response = await sendPlaygroundChat({
        document_id: selectedDocumentId,
        provider_id: selectedProviderId,
        model_id: selectedModelId,
        message: message.trim(),
        execute_commands: executeCommands
      })

      const chatMessage: ChatMessage = {
        id: response.message_id,
        timestamp: response.timestamp,
        userMessage: message.trim(),
        response
      }

      setMessages(prev => [...prev, chatMessage])
      setMessage('')
    } catch (error) {
      console.error('Failed to send message:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSendMessage()
    }
  }

  const renderDocument = () => {
    if (!currentDocument) return null

    if (documentView === 'udm') {
      return (
        <div className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-auto max-h-full">
          <pre className="text-sm font-mono whitespace-pre-wrap">
            {JSON.stringify(currentDocument.ucm, null, 2)}
          </pre>
        </div>
      )
    }

    return (
      <div className="prose prose-sm max-w-none overflow-auto max-h-full">
        <div className="bg-white p-6 rounded-lg">
          <div className="whitespace-pre-wrap">{currentDocument.markdown}</div>
        </div>
      </div>
    )
  }

  return (
    <div className="h-screen flex flex-col">
      <div className="bg-white border-b border-gray-200 p-4">
        <div className="flex items-center justify-between mb-4">
          <h1 className="text-2xl font-bold text-gray-900">Playground</h1>
          <div className="flex items-center gap-4">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={executeCommands}
                onChange={(e) => setExecuteCommands(e.target.checked)}
                className="w-4 h-4 rounded border-gray-300"
              />
              <span className="text-sm font-medium">Execute Commands</span>
            </label>
          </div>
        </div>

        <div className="flex gap-4">
          <div className="flex-1">
            <label className="block text-sm font-medium text-gray-700 mb-1">Document</label>
            <select
              value={selectedDocumentId}
              onChange={(e) => setSelectedDocumentId(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            >
              {documents?.map((doc: any) => (
                <option key={doc.id} value={doc.id}>{doc.name}</option>
              ))}
            </select>
          </div>

          <div className="flex-1">
            <label className="block text-sm font-medium text-gray-700 mb-1">Provider</label>
            <select
              value={selectedProviderId}
              onChange={(e) => setSelectedProviderId(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            >
              {providers?.map((provider: any) => (
                <option key={provider.id} value={provider.id}>{provider.name}</option>
              ))}
            </select>
          </div>

          <div className="flex-1">
            <label className="block text-sm font-medium text-gray-700 mb-1">Model</label>
            <select
              value={selectedModelId}
              onChange={(e) => setSelectedModelId(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            >
              {selectedProvider?.models.map((model: any) => (
                <option key={model.id} value={model.id}>{model.name}</option>
              ))}
            </select>
          </div>
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        <div className="flex-1 border-r border-gray-200 flex flex-col">
          <div className="bg-gray-50 border-b border-gray-200 p-3 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <FileText size={18} className="text-gray-600" />
              <span className="font-medium text-gray-900">Document</span>
            </div>
            <div className="flex items-center gap-2 bg-white rounded-lg border border-gray-200 p-1">
              <button
                onClick={() => setDocumentView('markdown')}
                className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
                  documentView === 'markdown'
                    ? 'bg-blue-50 text-blue-700'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                Markdown
              </button>
              <button
                onClick={() => setDocumentView('udm')}
                className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
                  documentView === 'udm'
                    ? 'bg-blue-50 text-blue-700'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                UDM
              </button>
            </div>
          </div>
          <div className="flex-1 overflow-auto p-4 bg-gray-50">
            {currentDocument ? renderDocument() : (
              <div className="text-center text-gray-500 mt-20">
                <FileText size={48} className="mx-auto mb-4 opacity-50" />
                <p>Select a document to view</p>
              </div>
            )}
          </div>
        </div>

        <div className="w-96 flex flex-col bg-white">
          <div className="bg-gray-50 border-b border-gray-200 p-3 flex items-center gap-2">
            <MessageSquare size={18} className="text-gray-600" />
            <span className="font-medium text-gray-900">Chat</span>
          </div>

          <div className="flex-1 overflow-auto p-4 space-y-4">
            {messages.length === 0 ? (
              <div className="text-center text-gray-500 mt-20">
                <MessageSquare size={48} className="mx-auto mb-4 opacity-50" />
                <p>Start a conversation to edit the document</p>
              </div>
            ) : (
              messages.map((msg) => (
                <div
                  key={msg.id}
                  className="cursor-pointer hover:bg-gray-50 rounded-lg p-3 border border-gray-200 transition-colors"
                  onClick={() => {
                    setSelectedMessage(msg)
                    setShowMessageDetail(true)
                  }}
                >
                  <div className="flex items-start gap-3">
                    <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
                      <span className="text-sm font-medium text-blue-700">U</span>
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="text-sm text-gray-900 mb-1">{msg.userMessage}</div>
                      <div className="flex items-center gap-2 text-xs text-gray-500">
                        {msg.response.parse_success ? (
                          <CheckCircle2 size={14} className="text-green-500" />
                        ) : (
                          <XCircle size={14} className="text-red-500" />
                        )}
                        <span>{msg.response.extracted_ucl || 'No UCL generated'}</span>
                      </div>
                    </div>
                  </div>
                </div>
              ))
            )}
          </div>

          <div className="border-t border-gray-200 p-4">
            <div className="flex gap-2">
              <input
                type="text"
                value={message}
                onChange={(e) => setMessage(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder="Describe the change you want to make..."
                className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                disabled={isLoading}
              />
              <button
                onClick={handleSendMessage}
                disabled={isLoading || !message.trim()}
                className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
              >
                {isLoading ? (
                  <Loader2 size={18} className="animate-spin" />
                ) : (
                  <Send size={18} />
                )}
              </button>
            </div>
          </div>
        </div>
      </div>

      {showMessageDetail && selectedMessage && (
        <MessageDetailModal
          message={selectedMessage}
          onClose={() => {
            setShowMessageDetail(false)
            setSelectedMessage(null)
          }}
        />
      )}
    </div>
  )
}

function MessageDetailModal({ message, onClose }: { message: ChatMessage; onClose: () => void }) {
  const [activeTab, setActiveTab] = useState<'prompt' | 'response' | 'parsed' | 'diffs'>('prompt')

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-xl shadow-2xl max-w-4xl w-full max-h-[90vh] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <h2 className="text-lg font-semibold">Message Details</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600"
          >
            ×
          </button>
        </div>

        <div className="border-b border-gray-200">
          <div className="flex">
            {[
              { id: 'prompt' as const, label: 'Prompt', icon: MessageSquare },
              { id: 'response' as const, label: 'Raw Response', icon: Code },
              { id: 'parsed' as const, label: 'Parsed UCL', icon: FileJson },
              { id: 'diffs' as const, label: 'Diffs', icon: Eye },
            ].map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
                  activeTab === tab.id
                    ? 'border-blue-500 text-blue-700'
                    : 'border-transparent text-gray-600 hover:text-gray-900'
                }`}
              >
                <tab.icon size={16} />
                {tab.label}
              </button>
            ))}
          </div>
        </div>

        <div className="flex-1 overflow-auto p-4">
          {activeTab === 'prompt' && (
            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-gray-700 mb-2">Full Prompt Sent to LLM</h3>
                <div className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-auto">
                  <pre className="text-sm whitespace-pre-wrap font-mono">{message.response.full_prompt}</pre>
                </div>
              </div>
              <div>
                <h3 className="text-sm font-medium text-gray-700 mb-2">User Message</h3>
                <div className="bg-gray-50 p-3 rounded-lg text-sm">{message.userMessage}</div>
              </div>
            </div>
          )}

          {activeTab === 'response' && (
            <div>
              <h3 className="text-sm font-medium text-gray-700 mb-2">Raw LLM Response</h3>
              <div className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-auto">
                <pre className="text-sm whitespace-pre-wrap">{message.response.raw_response}</pre>
              </div>
            </div>
          )}

          {activeTab === 'parsed' && (
            <div className="space-y-4">
              <div className="flex items-center gap-2 mb-4">
                {message.response.parse_success ? (
                  <CheckCircle2 size={20} className="text-green-500" />
                ) : (
                  <XCircle size={20} className="text-red-500" />
                )}
                <span className="font-medium">
                  {message.response.parse_success ? 'Parse Successful' : 'Parse Failed'}
                </span>
              </div>

              <div>
                <h3 className="text-sm font-medium text-gray-700 mb-2">Extracted UCL</h3>
                <div className="bg-gray-50 p-3 rounded-lg text-sm font-mono">
                  {message.response.extracted_ucl || 'No UCL extracted'}
                </div>
              </div>

              {message.response.parsed_commands.length > 0 && (
                <div>
                  <h3 className="text-sm font-medium text-gray-700 mb-2">Parsed Commands</h3>
                  <div className="space-y-2">
                    {message.response.parsed_commands.map((cmd, i) => (
                      <div key={i} className="bg-blue-50 p-3 rounded-lg text-sm font-mono">
                        {cmd}
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}

          {activeTab === 'diffs' && (
            <div className="space-y-4">
              <div className="flex items-center gap-4 text-sm">
                <div className="flex items-center gap-2">
                  <Clock size={16} className="text-gray-500" />
                  <span>{message.response.latency_ms}ms</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-gray-500">Tokens:</span>
                  <span>{message.response.input_tokens} in / {message.response.output_tokens} out</span>
                </div>
              </div>

              {message.response.execute_success !== null && (
                <div className="flex items-center gap-2">
                  {message.response.execute_success ? (
                    <CheckCircle2 size={20} className="text-green-500" />
                  ) : (
                    <XCircle size={20} className="text-red-500" />
                  )}
                  <span className="font-medium">
                    {message.response.execute_success ? 'Execution Successful' : 'Execution Failed'}
                  </span>
                </div>
              )}

              {message.response.diff && (
                <div>
                  <h3 className="text-sm font-medium text-gray-700 mb-2">Commands Applied</h3>
                  <div className="bg-gray-50 p-3 rounded-lg">
                    <pre className="text-sm">{JSON.stringify(message.response.diff, null, 2)}</pre>
                  </div>
                </div>
              )}

              {message.response.error && (
                <div>
                  <h3 className="text-sm font-medium text-gray-700 mb-2">Error</h3>
                  <div className="bg-red-50 text-red-900 p-3 rounded-lg text-sm">
                    {message.response.error}
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
