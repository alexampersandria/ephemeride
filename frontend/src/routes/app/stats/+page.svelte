<script lang="ts">
import Button from '$lib/components/Button.svelte'
import Heatmap from '$lib/components/Heatmap.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import type { HeatmapDataPoint } from '$lib/types/components/heatmap'
import { getEntries } from '$lib/utils/api'
import { currentDateObject, yearDateRange } from '$lib/utils/log'
import { ChartLine, ChevronLeft, ChevronRight } from 'lucide-svelte'
import { onMount } from 'svelte'

let userStore = useUserStore()

let yearlyDataYear = $state(currentDateObject().year)
let yearlyData: HeatmapDataPoint[] | undefined = $state(undefined)

const getData = async (year = yearlyDataYear) => {
  if (userStore.sessionId) {
    return getEntries(userStore.sessionId, {
      from_date: yearDateRange(year).firstDate,
      to_date: yearDateRange(year).lastDate,
      limit: 366,
    }).then(data => {
      if (data) {
        let heatmapData: HeatmapDataPoint[] = data.data.map(entry => {
          return {
            date: entry.date,
            value: entry.mood,
          }
        })
        yearlyData = heatmapData
        return heatmapData
      }
    })
  }
}

onMount(() => {
  getData()
})

const navigateYear = async (year: number) => {
  yearlyDataYear = year
  getData(year)
}
</script>

<div class="app-page stats-page">
  <div class="container">
    <div class="app-page-title">
      <ChartLine />
      Stats (WIP)
    </div>

    <div class="section heatmap-current-year">
      <div class="section-title">
        {yearlyDataYear} Heatmap
        <div class="navigation">
          <Button
            type="ghost"
            onclick={() => navigateYear(yearlyDataYear - 1)}
            aria-label="Previous Year">
            <ChevronLeft />
          </Button>

          <Button
            type="ghost"
            onclick={() => navigateYear(yearlyDataYear + 1)}
            aria-label="Next Year">
            <ChevronRight />
          </Button>
        </div>
      </div>

      <Heatmap
        year={yearlyDataYear}
        data={yearlyData}
        loading={yearlyData === undefined} />
    </div>
  </div>
</div>

<style lang="scss">
.stats-page {
  .section {
    display: flex;
    flex-direction: column;
    gap: var(--padding-s);

    .section-title {
      display: flex;
      justify-content: space-between;
      align-items: center;

      .navigation {
        display: flex;
        gap: var(--padding-xs);
      }
    }
  }
}
</style>
