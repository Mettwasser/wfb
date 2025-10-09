<script lang="ts">
    import type { PageData } from '../$types';
    import { SwordIcon } from '@lucide/svelte/icons';
    import { ProgressRing } from '@skeletonlabs/skeleton-svelte';

    let { params }: PageData = $props();

    let responsefake = {
        players: [
            'Player 1',
            'Player 2',
            'Player 3',
            'Player 4',
            'Player 5',
            'Player 6',
            'Player 7',
            'Player 8',
            'Player 9',
            'Player 10',
        ],
        host: 'My Host Name',
    };
</script>

<div class="flex gap-y-5 items-center justify-center flex-col h-full px-4">
    <h2 class="flex gap-x-5 justify-center items-center text-3xl font-bold">
        <ProgressRing
            value={null}
            size="size-10"
            meterStroke="stroke-secondary-600-400"
            trackStroke="stroke-secondary-50-950"
        />
        <div class="text-sm md:text-normal">Room joined, please wait...</div>
    </h2>
    <div class="text-xl font-bold text-tertiary-100">
        {responsefake.host}
    </div>
    <div class="w-full p-8 space-y-6 bg-surface-900 rounded-xl shadow-lg">
        <div class="max-h-150 grid grid-cols-1 md:grid-cols-5 overflow-y-scroll">
            {#each responsefake.players as p}
                <div class="px-2 py-1 cursor-pointer hover:bg-surface-500">
                    <span>
                        <SwordIcon class="inline pr-2 text-secondary-100" />{p}
                    </span>
                </div>
            {/each}
        </div>
    </div>
    <div class="mt-3 text-center text-2xl">
        Room <span class="font-bold text-secondary-100">{params.roomId}</span>
    </div>
</div>
