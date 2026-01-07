import { useState, useEffect, useCallback } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Skeleton } from '@/components/ui/skeleton'
import { TodoItem } from '@/components/TodoItem'
import { AddTodo } from '@/components/AddTodo'
import { CheckCircle2 } from 'lucide-react'
import type { Todo } from '@/lib/api'
import * as api from '@/lib/api'

export function TodoList() {
  const [todos, setTodos] = useState<Todo[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [isAdding, setIsAdding] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const loadTodos = useCallback(async () => {
    try {
      setError(null)
      const data = await api.fetchTodos()
      setTodos(data)
    } catch (err) {
      setError('Failed to load todos. Please try again.')
      console.error(err)
    } finally {
      setIsLoading(false)
    }
  }, [])

  useEffect(() => {
    loadTodos()
  }, [loadTodos])

  const handleAdd = async (title: string) => {
    try {
      setIsAdding(true)
      const newTodo = await api.createTodo(title)
      setTodos((prev) => [newTodo, ...prev])
    } catch (err) {
      setError('Failed to add todo.')
      console.error(err)
    } finally {
      setIsAdding(false)
    }
  }

  const handleToggle = async (id: string, completed: boolean) => {
    try {
      const updated = await api.updateTodo(id, { completed })
      setTodos((prev) => prev.map((todo) => (todo.id === id ? updated : todo)))
    } catch (err) {
      setError('Failed to update todo.')
      console.error(err)
    }
  }

  const handleUpdate = async (id: string, title: string) => {
    try {
      const updated = await api.updateTodo(id, { title })
      setTodos((prev) => prev.map((todo) => (todo.id === id ? updated : todo)))
    } catch (err) {
      setError('Failed to update todo.')
      console.error(err)
    }
  }

  const handleDelete = async (id: string) => {
    try {
      await api.deleteTodo(id)
      setTodos((prev) => prev.filter((todo) => todo.id !== id))
    } catch (err) {
      setError('Failed to delete todo.')
      console.error(err)
    }
  }

  const completedCount = todos.filter((t) => t.completed).length
  const totalCount = todos.length

  return (
    <Card className="w-full max-w-2xl mx-auto shadow-xl border-border/50 bg-card/80 backdrop-blur-sm">
      <CardHeader className="space-y-4 pb-6">
        <div className="flex items-center justify-between">
          <CardTitle className="text-3xl font-bold bg-linear-to-r from-foreground to-foreground/70 bg-clip-text">
            <span className="flex items-center gap-3">
              <CheckCircle2 className="h-8 w-8 text-primary" />
              Todo App
            </span>
          </CardTitle>
          {totalCount > 0 && (
            <span className="text-sm text-muted-foreground">
              {completedCount} of {totalCount} completed
            </span>
          )}
        </div>
        <AddTodo onAdd={handleAdd} isLoading={isAdding} />
      </CardHeader>

      <CardContent className="space-y-3">
        {error && (
          <div className="p-4 rounded-lg bg-destructive/10 text-destructive text-sm">
            {error}
          </div>
        )}

        {isLoading ? (
          <div className="space-y-3">
            {[...Array(3)].map((_, i) => (
              <Skeleton key={i} className="h-16 w-full rounded-xl" />
            ))}
          </div>
        ) : todos.length === 0 ? (
          <div className="text-center py-12 text-muted-foreground">
            <CheckCircle2 className="h-12 w-12 mx-auto mb-4 opacity-50" />
            <p className="text-lg">No todos yet</p>
            <p className="text-sm">Add your first task above!</p>
          </div>
        ) : (
          <div className="space-y-2">
            {todos.map((todo) => (
              <TodoItem
                key={todo.id}
                todo={todo}
                onToggle={handleToggle}
                onDelete={handleDelete}
                onUpdate={handleUpdate}
              />
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
