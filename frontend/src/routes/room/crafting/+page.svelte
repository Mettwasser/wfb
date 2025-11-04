<script lang="ts">
    import { goto } from '$app/navigation';
    import { COLOR_THEMES, socket, toaster, type Card, type DraggedItemInfo } from '$lib';
    import Grid from '$lib/components/Grid.svelte';
    import { session } from '$lib/session.svelte';
    import type { SubmitBoardRequest } from '$lib/socket_event_types';
    import { getCardIds } from '$lib/utils';
    import { Check, LoaderCircle } from '@lucide/svelte';
    import { Toaster } from '@skeletonlabs/skeleton-svelte';
    import { onMount } from 'svelte';

    onMount(() => {
        socket.on('nextStage', (_) => {
            goto('/room/choosing');
        });

        return () => {
            socket.removeListener('nextStage');
        };
    });

    // --- STATE ---

    // The full card objects are now generated dynamically.
    const allCards: Card[] = session.info.cards;

    // The grid cells, 5x5 (25 cells)
    let gridCells: (Card | null)[] = $state(Array(25).fill(null));

    // This object holds information about the item being dragged.
    let draggedItemInfo: DraggedItemInfo = $state({
        item: null,
        source: null,
        sourceIndex: -1,
    });

    // This holds the index of the grid cell the cursor is currently over.
    let dragOverIndex: number = $state(-1);

    let isSubmitting = $state(false);
    let hasSubmitted = $state(false);

    // Derived state to check if all cards have been placed on the grid.
    let enableSubmitButton: boolean = $derived(
        allCards.every((card) => gridCells.some((cell) => cell?.id === card.id)) &&
            !isSubmitting &&
            !hasSubmitted
    );

    // --- HANDLERS ---

    function handleDragStart(event: DragEvent, item: Card, source: 'palette', index: number) {
        const target = event.currentTarget as HTMLElement;
        if (event.dataTransfer) {
            event.dataTransfer.effectAllowed = 'move';
            const rect = target.getBoundingClientRect();
            event.dataTransfer.setDragImage(target, rect.width / 2, rect.height / 2);
        }

        draggedItemInfo.item = item;
        draggedItemInfo.source = source;
        draggedItemInfo.sourceIndex = index;
    }

    function handleDragEnd() {
        // Reset state after drag ends
        draggedItemInfo.item = null;
        draggedItemInfo.source = null;
        draggedItemInfo.sourceIndex = -1;
        dragOverIndex = -1; // Also clear the hover state
    }

    function handlePaletteDrop(event: DragEvent) {
        event.preventDefault();
        if (!draggedItemInfo.item || draggedItemInfo.source !== 'grid') return;

        // Remove the item from the grid. It will automatically reappear in the palette.
        gridCells[draggedItemInfo.sourceIndex] = null;
    }

    async function submitBoard() {
        isSubmitting = true;

        let data: SubmitBoardRequest = {
            lobbyId: session.info.lobbyId,
            // SAFETY: button is disabled unless every card is on the board
            cards: getCardIds(gridCells as Card[]),
        };

        console.log(data);

        const response = await socket.emitWithAck('submitBoard', data);

        if (response.success) {
            hasSubmitted = true;
        } else {
            toaster.error({ title: 'Failed to submit board.' });
        }

        isSubmitting = false;
    }
</script>

<Toaster {toaster} />

<!-- TEMPLATE -->
<div class="text-surface-200 flex h-screen w-full flex-row gap-8 p-8">
    <!-- Available Cards Palette (Left Side) -->
    <div
        class="bg-surface-900 card flex w-1/4 flex-shrink-0 flex-col p-6 shadow-lg"
        ondragover={(e) => e.preventDefault()}
        ondrop={handlePaletteDrop}
    >
        <h2 class="border-surface-700 mb-4 flex-shrink-0 border-b pb-3 text-xl font-bold">
            Room {session.info.lobbyId}
        </h2>
        <div class="flex flex-row flex-wrap gap-4 overflow-y-auto overscroll-contain pr-2">
            {#each allCards as card, i}
                {@const isOnGrid = gridCells.some((cell) => cell?.id === card.id)}

                {#if !isOnGrid}
                    {@const isDragging = draggedItemInfo.item?.id === card.id}
                    <div
                        class="text-surface-50 flex min-h-[90px] w-[calc(50%-0.5rem)] cursor-grab items-center justify-center rounded-lg p-4 text-center font-medium transition-all duration-200 select-none {card.color} {isDragging
                            ? 'scale-105 opacity-40'
                            : ''}"
                        draggable="true"
                        ondragstart={(e) => handleDragStart(e as DragEvent, card, 'palette', i)}
                        ondragend={handleDragEnd}
                    >
                        <span class="pointer-events-none">{card.description}</span>
                    </div>
                {/if}
            {/each}

            {#if enableSubmitButton}
                <p class="text-surface-400 w-full py-4 text-center">All cards are on the grid!</p>
            {/if}
        </div>
    </div>

    <div class="flex w-full flex-col gap-4">
        <!-- Drop Grid (Right Side) - Now a separate component -->
        <Grid bind:gridCells bind:draggedItemInfo bind:dragOverIndex />
        <div class="flex items-center justify-end gap-4">
            <span class="text-xs">
                <b>Note:</b>
                once locked in, you CANNOT go back
            </span>
            <button
                class="btn preset-filled-primary-500"
                disabled={!enableSubmitButton}
                onclick={submitBoard}
            >
                {#if isSubmitting}
                    <span><LoaderCircle class="animate-spin" /></span>
                {:else if hasSubmitted}
                    <span><Check /></span>
                {/if}
                Lock in
            </button>
        </div>
    </div>
</div>
