export interface Todo {
  id: string
  title: string
  completed: boolean
  created_at: string
  updated_at: string
}

const API_BASE = '/api'

export async function fetchTodos(): Promise<Todo[]> {
  const response = await fetch(`${API_BASE}/todos`)
  if (!response.ok) {
    throw new Error('Failed to fetch todos')
  }
  return response.json()
}

export async function createTodo(title: string): Promise<Todo> {
  const response = await fetch(`${API_BASE}/todos`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ title }),
  })
  if (!response.ok) {
    throw new Error('Failed to create todo')
  }
  return response.json()
}

export async function updateTodo(
  id: string,
  updates: { title?: string; completed?: boolean },
): Promise<Todo> {
  const response = await fetch(`${API_BASE}/todos/${id}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(updates),
  })
  if (!response.ok) {
    throw new Error('Failed to update todo')
  }
  return response.json()
}

export async function deleteTodo(id: string): Promise<void> {
  const response = await fetch(`${API_BASE}/todos/${id}`, {
    method: 'DELETE',
  })
  if (!response.ok) {
    throw new Error('Failed to delete todo')
  }
}
