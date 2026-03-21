export interface CommandParam {
  name: string
  label: string
  type: 'text' | 'number' | 'select'
  placeholder?: string
  options?: string[]
  default?: string
  required?: boolean
}

import type { Component } from 'vue'

export interface Command {
  id: string
  label: string
  description?: string
  shortcut?: string
  /** Inline SVG string or a Vue component */
  icon?: string | Component
  params?: CommandParam[]
  action: (params?: Record<string, string>) => void | Promise<void>
}

export interface CommandGroup {
  id: string
  label: string
  commands: Command[]
}
