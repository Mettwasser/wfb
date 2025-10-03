<script lang="ts">
    import { COLOR_THEMES, type Card, type DraggedItemInfo } from '$lib';
    import Grid from '$lib/components/Grid.svelte';
    import type { PageData } from './$types';

    let { params }: PageData = $props();

    // 2. Provide the card content as a simple array of strings.
    const initialCardData = [
        'View detailed analytics and performance metrics',
        'Configure user permissions and account settings',
        'Return to the main project dashboard overview',
        'Check your team-wide messages and notifications',
        'Open the shared team event and meeting calendar',
        'Review the latest audit and security logs',
        'Access the customer support ticket queue',
        'Manage billing and subscription information',
        'Generate and export financial reports',
        'Integrate with third-party applications',
        'Customize the user interface and themes',
        'Manage API keys and developer access',
        'View real-time system status and uptime',
        'Create and manage automated workflows',
        'Set up data backup and recovery options',
        'Explore the knowledge base and documentation',
        'Track project milestones and deadlines',
        'Collaborate on documents with team members',
        'Analyze user engagement and activity',
        'Configure two-factor authentication (2FA)',
        'Manage data import and export tasks',
        'Access developer tools and sandbox environment',
        'View and manage asset library',
        'Configure notification preferences',
        'Submit a feature request or feedback'
    ];

    // --- STATE ---

    // The full card objects are now generated dynamically.
    const allCards: Card[] = initialCardData.map((content, index) => {
        const theme = COLOR_THEMES[index % COLOR_THEMES.length];
        return {
            id: index + 1, // Generate a simple ID
            content,
            ...theme
        };
    });

    // The grid cells, 5x5 (25 cells)
    let gridCells: (Card | null)[] = $state(Array(25).fill(null));

    // This object holds information about the item being dragged.
    let draggedItemInfo: DraggedItemInfo = $state({
        item: null,
        source: null,
        sourceIndex: -1
    });

    // This holds the index of the grid cell the cursor is currently over.
    let dragOverIndex: number = $state(-1);

    // Derived state to check if all cards have been placed on the grid.
    let allCardsPlaced: boolean = $derived(
        allCards.every((card) => gridCells.some((cell) => cell?.id === card.id))
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
</script>

<!-- TEMPLATE -->
<div class="flex flex-row gap-8 w-full h-screen p-8 text-surface-200">
    <!-- Available Cards Palette (Left Side) -->
    <div
        class="w-1/4 flex-shrink-0 p-6 bg-surface-900 rounded-xl shadow-lg flex flex-col"
        ondragover={(e) => e.preventDefault()}
        ondrop={handlePaletteDrop}
    >
        <h2 class="text-xl font-bold border-b border-surface-700 pb-3 mb-4 flex-shrink-0">
            Room {params.roomId}
        </h2>
        <div class="flex flex-row flex-wrap gap-4 overflow-y-auto overscroll-contain pr-2">
            {#each allCards as card, i}
                {@const isOnGrid = gridCells.some((cell) => cell?.id === card.id)}

                {#if isOnGrid}
                    <!-- Placeholder for cards on the grid -->
                    <div
                        class="p-4 rounded-lg min-h-[90px] w-[calc(50%-0.5rem)] flex items-center justify-center text-center border-2 border-dashed {card.borderColor}"
                    >
                        <span class="font-medium opacity-75 {card.textColor}">{card.content}</span>
                    </div>
                {:else}
                    <!-- The actual draggable card -->
                    {@const isDragging = draggedItemInfo.item?.id === card.id}
                    <div
                        class="p-4 rounded-lg font-medium cursor-grab text-surface-50 text-center transition-all duration-200 select-none w-[calc(50%-0.5rem)] min-h-[90px] flex items-center justify-center {card.color} {isDragging
                            ? 'opacity-40 scale-105'
                            : ''}"
                        draggable="true"
                        ondragstart={(e) => handleDragStart(e as DragEvent, card, 'palette', i)}
                        ondragend={handleDragEnd}
                    >
                        <span class="pointer-events-none">{card.content}</span>
                    </div>
                {/if}
            {/each}

            {#if allCardsPlaced}
                <p class="text-surface-400 text-center py-4 w-full">All cards are on the grid!</p>
            {/if}
        </div>
    </div>

    <!-- Drop Grid (Right Side) - Now a separate component -->
    <Grid bind:gridCells bind:draggedItemInfo bind:dragOverIndex />
</div>
