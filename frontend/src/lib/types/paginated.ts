export type Paginated<T> = {
  data: T[]
  pagination: PaginationObject
}

export type PaginationObject = {
  limit: number
  offset: number
  total_count: number
}
