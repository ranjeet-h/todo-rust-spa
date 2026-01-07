import { createFileRoute } from '@tanstack/react-router'
import { TodoList } from '@/components/TodoList'

export const Route = createFileRoute('/')({
  component: App,
})

function App() {
  return (
    <div className="min-h-screen bg-linear-to-br from-background via-background to-muted/30 py-12 px-4">
      <div className="max-w-4xl mx-auto">
        <TodoList />
      </div>
    </div>
  )
}
