<script lang="ts">
import type { CalendarProps } from '$lib/types/components/calendar'
import {
  calendarDaysInMonth,
  calendarDefaults,
  formatMonth,
} from '$lib/utils/log'
import { ChevronLeft, ChevronRight } from 'lucide-svelte'
import Button from './Button.svelte'

let {
  mode = 'single',
  month = calendarDefaults().month,
  year = calendarDefaults().year,
  days = [],
}: CalendarProps = $props()

let daysInMonth = $derived.by(() => {
  return calendarDaysInMonth(year, month)
})

const navigate = (increment: number) => {
  month += increment
  if (month < 1) {
    month = 12
    year -= 1
  } else if (month > 12) {
    month = 1
    year += 1
  }
}
</script>

<div class="calendar">
  <div class="navigation">
    <div class="left-right">
      <Button type="ghost" onclick={() => navigate(-1)}>
        <ChevronLeft />
      </Button>
      <Button type="ghost" onclick={() => navigate(1)}>
        <ChevronRight />
      </Button>
    </div>

    <div class="title">
      <div class="month">
        {formatMonth(month)}
      </div>
      <div class="year">
        {year}
      </div>
    </div>
  </div>

  <div class="days">
    <div class="row weekdays">
      <div class="day">M</div>
      <div class="day">T</div>
      <div class="day">W</div>
      <div class="day">T</div>
      <div class="day">F</div>
      <div class="day">S</div>
      <div class="day">S</div>
    </div>

    {#each daysInMonth as week}
      <div class="row week">
        {#each week as day}
          <div class="day">
            {#if day !== null}
              <Button type="ghost" fullwidth>{day}</Button>
            {/if}
          </div>
        {/each}
      </div>
    {/each}
  </div>

  <code class="debug"
    ><pre>
mode: {JSON.stringify(mode)}
month: {month}
year: {year}
days: {JSON.stringify(days)}
</pre></code>
</div>

<style lang="scss">
.calendar {
  width: 100%;
  max-width: var(--block-size-xs);

  .navigation {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--padding-s) calc(100% / 28);

    .left-right,
    .title {
      display: flex;
      gap: var(--padding-xs);
    }

    .year {
      font-weight: 600;
    }

    .month {
      color: var(--text-muted);
    }
  }

  .days {
    display: table;
    width: 100%;

    .row {
      display: table-row;

      .day {
        display: table-cell;
        text-align: center;
        width: calc(100% / 7);
      }
    }
  }
}
</style>
