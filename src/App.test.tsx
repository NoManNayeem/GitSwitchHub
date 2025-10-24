import { describe, it, expect } from 'vitest'
import { render, screen, act } from '@testing-library/react'
import App from './App'

describe('App', () => {
  it('renders GitSwitchHub title', async () => {
    await act(async () => {
      render(<App />)
    })
    expect(screen.getByText(/GitSwitchHub/i)).toBeInTheDocument()
  })

  it('renders without crashing', async () => {
    await act(async () => {
      render(<App />)
    })
    expect(document.body).toBeInTheDocument()
  })
})
