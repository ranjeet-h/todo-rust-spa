import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Plus } from 'lucide-react'
import { useState } from 'react'

interface AddTodoProps {
  onAdd: (title: string) => void
  isLoading?: boolean
}

export function AddTodo({ onAdd, isLoading }: AddTodoProps) {
  const [title, setTitle] = useState('')

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (title.trim()) {
      onAdd(title.trim())
      setTitle('')
    }
  }

  return (
    <form onSubmit={handleSubmit} className="flex gap-3">
      <Input
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder="What needs to be done?"
        className="flex-1 h-12 text-base px-4 bg-card border-border/50 focus:border-primary"
        disabled={isLoading}
      />
      <Button
        type="submit"
        size="lg"
        disabled={!title.trim() || isLoading}
        className="h-12 px-6 gap-2"
      >
        <Plus className="h-5 w-5" />
        Add
      </Button>
    </form>
  )
}
