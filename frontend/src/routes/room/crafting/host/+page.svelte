<script lang="ts">
    import { socket } from '$lib';
    import { session } from '$lib/session.svelte';
    import { Check, LoaderCircle } from '@lucide/svelte';
    import { onMount } from 'svelte';
    import { SvelteSet } from 'svelte/reactivity';

    let playersReady = new SvelteSet();

    onMount(() => {
        socket.on('boardSubmitted', (username) => playersReady.add(username));

        return () => {
            socket.removeListener('boardSubmitted');
        };
    });
</script>

<div class="flex h-full flex-col items-center justify-center gap-y-5 px-4">
    <h2 class="flex items-center justify-center gap-x-5 text-3xl font-bold">
        <div class="md:text-normal h3">
            Waiting for players to submit their <span class="text-primary-400">bingo-board</span>
        </div>
    </h2>
    <div class="bg-surface-900 w-2/3 rounded-xl p-8 shadow-lg">
        <div class="grid max-h-150 grid-cols-5 gap-4 overflow-y-auto p-4">
            {#each session.info!.players as p (p)}
                <div>
                    <span
                        class="card preset-tonal-surface flex justify-center gap-2 rounded-lg p-3"
                    >
                        {#if playersReady.has(p)}
                            <Check class="text-success-500 inline" />
                        {:else}
                            <LoaderCircle class="text-primary-500 inline animate-spin" />
                        {/if}
                        {p}
                    </span>
                </div>
            {:else}
                <div class="text-center text-surface-400 col-span-full">No players yet</div>
            {/each}
        </div>
    </div>

    {#if session.info!.isHost}
        <div class="flex w-2/3 justify-end">
            <button class="btn preset-gradient" onclick={() => {}}>Next Stage</button>
        </div>
    {/if}
</div>

<style>
    .preset-gradient {
        background-image: linear-gradient(
            -45deg,
            var(--color-primary-500),
            var(--color-secondary-500)
        );
        color: var(--color-primary-contrast-500);
    }
</style>
