export type UserDetails = {
  id: string
  created_at: string
  name: string
  email: string
  invite?: string
}

export type SessionResponse = {
  id: string
  user_id: string
  created_at: string
  accessed_at: string
  ip_address: string
  user_agent: string
}
