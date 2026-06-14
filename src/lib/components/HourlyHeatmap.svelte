<script>
  import { locale, formatDurationLocalized, t } from '$lib/i18n/index.js';

  export let data = [];
  export let embedded = false;

  $: currentLocale = $locale;
  const keyHours = [0, 6, 12, 18, 23];
  const weekdayKeys = [
    'weekdayMon', 'weekdayTue', 'weekdayWed', 'weekdayThu',
    'weekdayFri', 'weekdaySat', 'weekdaySun',
  ];

  function formatHourLabel(hour) {
    return `${String(hour).padStart(2, '0')}:00`;
  }
  function formatHourRangeLabel(hour) {
    return `${formatHourLabel(hour)} - ${formatHourLabel((hour + 1) % 24)}`;
  }
  // 取 MM-DD 作为行标签副信息
  function formatMd(date) {
    return date.length >= 10 ? date.slice(5).replace('-', '/') : date;
  }

  // 全局最大值（所有格子），用于深浅分档
  $: maxDuration = Math.max(1, ...data.flatMap((d) => d.hourly || []));

  // 5 档：0=空，1-4=indigo 由浅到深
  function intensity(duration) {
    if (!duration || duration <= 0) return 0;
    const ratio = duration / maxDuration;
    if (ratio < 0.25) return 1;
    if (ratio < 0.5) return 2;
    if (ratio < 0.75) return 3;
    return 4;
  }
  const cellBg = [
    'bg-slate-100 dark:bg-slate-800/60',
    'bg-indigo-100 dark:bg-indigo-900/40',
    'bg-indigo-300 dark:bg-indigo-700/50',
    'bg-indigo-500 dark:bg-indigo-500/70',
    'bg-indigo-600 dark:bg-indigo-400',
  ];

  $: chartShellClass = embedded
    ? 'rounded-[24px] bg-transparent p-0'
    : 'rounded-2xl border border-slate-100 bg-white p-4 dark:border-slate-700/60 dark:bg-slate-800/80';
</script>

<div class="space-y-4" data-locale={currentLocale}>
  <div class={chartShellClass}>
    <div class="mb-3">
      <p class="text-sm font-semibold text-slate-700 dark:text-slate-200">
        {t('hourlyChart.heatmap.titleWeek')}
      </p>
      <p class="mt-1 text-xs text-slate-500 dark:text-slate-400">
        {t('hourlyChart.heatmap.subtitleWeek')}
      </p>
    </div>

    {#if data && data.length}
      <div class="overflow-x-auto">
        <div class="min-w-[640px]">
          <!-- 列标签：小时 -->
          <div class="grid gap-1" style="grid-template-columns: 3.5rem repeat(24, minmax(0,1fr));">
            <div></div>
            {#each Array(24) as _, hour}
              <div class={`text-center text-[10px] font-medium ${keyHours.includes(hour) ? 'text-slate-400 dark:text-slate-500' : 'text-transparent'}`}>
                {keyHours.includes(hour) ? formatHourLabel(hour) : '.'}
              </div>
            {/each}
          </div>
          <!-- 7 行：每天 -->
          {#each data as row}
            <div class="grid gap-1 mt-1" style="grid-template-columns: 3.5rem repeat(24, minmax(0,1fr));">
              <div class="flex items-center whitespace-nowrap text-[11px] font-medium text-slate-500 dark:text-slate-400">
                {t(`hourlyChart.heatmap.${weekdayKeys[row.weekday] ?? 'weekdayMon'}`)} {formatMd(row.date)}
              </div>
              {#each row.hourly as dur, hour}
                <div
                  class={`h-5 rounded-[3px] ${cellBg[intensity(dur)]}`}
                  title={`${row.date} ${formatHourRangeLabel(hour)} · ${formatDurationLocalized(dur)}`}
                ></div>
              {/each}
            </div>
          {/each}
          <!-- 图例：少 → 多 -->
          <div class="mt-3 flex items-center justify-end gap-1.5 text-[11px] text-slate-400 dark:text-slate-500">
            <span>{t('hourlyChart.heatmap.less')}</span>
            {#each [0, 1, 2, 3, 4] as level}
              <span class={`h-3 w-3 rounded-[3px] ${cellBg[level]}`}></span>
            {/each}
            <span>{t('hourlyChart.heatmap.more')}</span>
          </div>
        </div>
      </div>
    {:else}
      <div class="py-8 text-center text-sm text-slate-400 dark:text-slate-500">
        {t('hourlyChart.heatmap.subtitleWeek')}
      </div>
    {/if}
  </div>
</div>
