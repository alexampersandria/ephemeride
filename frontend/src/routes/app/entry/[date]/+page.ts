import { currentDate } from '$lib/utils/log'
import type { PageLoad } from './$types'

export const load: PageLoad = ({ params }) => {
  if (params.date === 'today') {
    params.date = currentDate()
  }

  return {
    date: params.date,
  }
}
