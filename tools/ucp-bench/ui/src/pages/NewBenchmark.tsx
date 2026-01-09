import { useState } from 'react'
import { useQuery, useMutation } from '@tanstack/react-query'
import { useNavigate } from 'react-router-dom'
import { fetchCategories, fetchProviders, createSuite, startRun, Category, Provider } from '../api/client'
import { Play, Check } from 'lucide-react'

export default function NewBenchmark() {
  const navigate = useNavigate()
  const [name, setName] = useState('New Benchmark')
  const [selectedCategories, setSelectedCategories] = useState<Set<string>>(new Set())
  const [selectedProviders, setSelectedProviders] = useState<Set<string>>(new Set())
  const [executeCommands, setExecuteCommands] = useState(false)
  const [concurrency, setConcurrency] = useState(3)

  const { data: categories } = useQuery({ queryKey: ['categories'], queryFn: fetchCategories })
  const { data: providers } = useQuery({ queryKey: ['providers'], queryFn: fetchProviders })

  const createAndRun = useMutation({
    mutationFn: async () => {
      const pairs = Array.from(selectedProviders).map(id => {
        const [provider_id, model_id] = id.split('/')
        return { provider_id, model_id, enabled: true }
      })

      const suite = await createSuite({
        name,
        categories: Array.from(selectedCategories),
        config: {
          matrix: { pairs },
          concurrency,
          execute_commands: executeCommands,
          capture_debug_info: true,
          capture_document_snapshots: true,
        },
      })

      const { run_id } = await startRun(suite.id)
      return run_id
    },
    onSuccess: (runId) => {
      navigate(`/run/${runId}`)
    },
  })

  const toggleCategory = (id: string) => {
    const next = new Set(selectedCategories)
    if (next.has(id)) next.delete(id)
    else next.add(id)
    setSelectedCategories(next)
  }

  const toggleProvider = (providerId: string, modelId: string) => {
    const key = `${providerId}/${modelId}`
    const next = new Set(selectedProviders)
    if (next.has(key)) next.delete(key)
    else next.add(key)
    setSelectedProviders(next)
  }

  const selectAllCategories = () => {
    if (categories) {
      setSelectedCategories(new Set(categories.map(c => c.id)))
    }
  }

  const selectAllProviders = () => {
    if (providers) {
      const all = providers.flatMap(p => p.models.map(m => `${p.id}/${m.id}`))
      setSelectedProviders(new Set(all))
    }
  }

  return (
    <div className="space-y-8 max-w-4xl">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">New Benchmark</h1>
        <p className="text-gray-500">Configure and run a new benchmark suite</p>
      </div>

      {/* Name */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">Benchmark Name</label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        />
      </div>

      {/* Categories */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold">Test Categories</h2>
          <button onClick={selectAllCategories} className="text-sm text-blue-600 hover:underline">
            Select All
          </button>
        </div>
        <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
          {categories?.map(cat => (
            <button
              key={cat.id}
              onClick={() => toggleCategory(cat.id)}
              className={`p-3 rounded-lg border text-left transition-colors ${
                selectedCategories.has(cat.id)
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-200 hover:border-gray-300'
              }`}
            >
              <div className="flex items-center justify-between">
                <span className="font-medium">{cat.name}</span>
                {selectedCategories.has(cat.id) && <Check size={16} className="text-blue-500" />}
              </div>
              <span className="text-sm text-gray-500">{cat.test_count} tests</span>
            </button>
          ))}
        </div>
      </div>

      {/* Providers */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold">Providers & Models</h2>
          <button onClick={selectAllProviders} className="text-sm text-blue-600 hover:underline">
            Select All
          </button>
        </div>
        <div className="space-y-4">
          {providers?.map(provider => (
            <div key={provider.id} className="border border-gray-200 rounded-lg p-4">
              <h3 className="font-medium mb-2">{provider.name}</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                {provider.models.map(model => {
                  const key = `${provider.id}/${model.id}`
                  const isSelected = selectedProviders.has(key)
                  return (
                    <button
                      key={model.id}
                      onClick={() => toggleProvider(provider.id, model.id)}
                      className={`p-2 rounded border text-left text-sm transition-colors ${
                        isSelected
                          ? 'border-blue-500 bg-blue-50'
                          : 'border-gray-200 hover:border-gray-300'
                      }`}
                    >
                      <div className="flex items-center justify-between">
                        <span>{model.name}</span>
                        {isSelected && <Check size={14} className="text-blue-500" />}
                      </div>
                    </button>
                  )
                })}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Options */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <h2 className="text-lg font-semibold mb-4">Options</h2>
        <div className="space-y-4">
          <label className="flex items-center gap-3">
            <input
              type="checkbox"
              checked={executeCommands}
              onChange={(e) => setExecuteCommands(e.target.checked)}
              className="w-4 h-4 rounded border-gray-300"
            />
            <div>
              <span className="font-medium">Execute Commands</span>
              <p className="text-sm text-gray-500">Actually execute UCL commands and validate document changes</p>
            </div>
          </label>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Concurrency</label>
            <input
              type="number"
              min={1}
              max={10}
              value={concurrency}
              onChange={(e) => setConcurrency(parseInt(e.target.value) || 1)}
              className="w-24 px-3 py-2 border border-gray-300 rounded-lg"
            />
          </div>
        </div>
      </div>

      {/* Run Button */}
      <div className="flex justify-end">
        <button
          onClick={() => createAndRun.mutate()}
          disabled={selectedCategories.size === 0 || selectedProviders.size === 0 || createAndRun.isPending}
          className="flex items-center gap-2 px-6 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Play size={20} />
          {createAndRun.isPending ? 'Starting...' : 'Run Benchmark'}
        </button>
      </div>
    </div>
  )
}
