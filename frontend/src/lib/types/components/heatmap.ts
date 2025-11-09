export type HeatmapDataPoint = {
  /**
   * date YYYY-MM-DD format
   */
  date: string
  value: number
}

export type HeatmapProps = {
  data?: HeatmapDataPoint[]
  year?: number
  loading?: boolean
}
