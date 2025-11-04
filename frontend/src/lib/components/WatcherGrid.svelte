<script lang="ts">
    import type { Card } from '$lib';
    import { session } from '$lib/session.svelte';

    interface Props {
        gridCells: Card[];
    }

    let { gridCells = $bindable() }: Props = $props();
</script>

<div class="bg-surface-900 card flex flex-1 p-6">
    <div class="grid h-full w-full grid-cols-5 grid-rows-5 gap-4">
        {#each gridCells as cell (cell.id)}
            <div
                class="bg-surface-800/50 border-surface-700 flex items-center justify-center rounded-lg border-2 border-dashed transition-colors"
            >
                <div
                    class="flex h-full w-full items-center justify-center rounded-lg p-4 text-center font-medium text-white transition-all duration-200 select-none
                    {session.info.correctAnswers.includes(cell.id)
                        ? 'bg-success-700/70'
                        : 'bg-surface-700'}"
                >
                    <span class="pointer-events-none">{cell.description}</span>
                </div>
            </div>
        {/each}
    </div>
</div>
