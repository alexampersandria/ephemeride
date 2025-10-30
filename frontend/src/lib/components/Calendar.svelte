<script lang="ts">
import type { CalendarProps } from '$lib/types/components/calendar'
import {
  calendarDaysInMonth,
  calendarDefaults,
  formatMonth,
} from '$lib/utils/log'
import { ChevronLeft, ChevronRight } from 'lucide-svelte'
import type { Color } from '$lib/types/color'

let {
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

const colorForDay = (day: number | null): Color | null => {
  if (day === null) return null

  const date = `${year}-${String(month).padStart(2, '0')}-${String(
    day,
  ).padStart(2, '0')}`
  const dayEntry = days.find(d => d.date === date)
  return dayEntry ? dayEntry.color : null
}
</script>

<div class="calendar">
  <div class="navigation">
    <button class="month-navigation" onclick={() => navigate(-1)}>
      <ChevronLeft />
    </button>

    <div class="title">
      <div class="month">
        {formatMonth(month)}
      </div>
      <div class="year">
        {year}
      </div>
    </div>

    <button class="month-navigation" onclick={() => navigate(1)}>
      <ChevronRight />
    </button>
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
          <div class="day {colorForDay(day)}">
            {#if day !== null}
              <a
                href={`/entry/${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`}
                class="day-button">{day}</a>
            {/if}
          </div>
        {/each}
      </div>
    {/each}
  </div>
</div>

<style lang="scss">
.calendar {
  width: 100%;

  .navigation {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--padding-s) 0;
    gap: var(--padding-xs);

    .month-navigation {
      background-color: transparent;
      border: none;
      color: var(--text-muted);
      cursor: pointer;
      padding: var(--padding-xs);
      border-radius: var(--button-radius);

      &:hover {
        background-color: var(--background-accent);
        color: var(--text-primary);
      }

      &:active {
        background-color: var(--background-primary);
        color: var(--text-primary);
        transform: var(--click-transform);
      }
    }

    .title {
      display: flex;
      flex-shrink: 1;
      overflow: hidden;
      gap: var(--padding-xs);

      .year {
        font-weight: 600;
      }

      .month {
        color: var(--text-muted);
        overflow: hidden;
        text-overflow: ellipsis;
      }
    }
  }

  .days {
    display: table;
    width: 100%;

    .row {
      display: table-row;

      &.weekdays {
        font-weight: 600;

        .day {
          padding-bottom: var(--padding-s);
        }
      }

      .day {
        display: table-cell;
        text-align: center;
        width: calc(100% / 7);

        .day-button {
          background-color: transparent;
          color: var(--text-muted);
          border: none;
          border-radius: var(--button-radius);
          height: var(--button-height);
          width: 100%;
          display: inline-block;
          line-height: var(--button-height);
          font-size: var(--font-size-s);

          &:hover {
            background-color: var(--background-accent);
            color: var(--text-primary);
            cursor: pointer;
          }

          &:active {
            background-color: var(--background-primary);
            color: var(--text-primary);
            transform: var(--click-transform);
          }
        }
      }
    }
  }
}
</style>
