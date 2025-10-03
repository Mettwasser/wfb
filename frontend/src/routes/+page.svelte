<script lang="ts">
    // --- TYPES ---
    interface Card {
        id: number;
        content: string;
        color: string;
        borderColor: string;
        textColor: string;
    }

    type DraggedItemInfo = {
        item: Card | null;
        source: 'palette' | 'grid' | null;
        sourceIndex: number;
    };

    // --- CONFIGURATION ---

    // 1. Define your color palette here. These themes will be cycled through.
    //    Ensure your tailwind.config.js safelists these classes.
    const colorThemes = [
        { color: 'bg-blue-500', borderColor: 'border-blue-500', textColor: 'text-blue-500' },
        { color: 'bg-violet-500', borderColor: 'border-violet-500', textColor: 'text-violet-500' },
        {
            color: 'bg-emerald-500',
            borderColor: 'border-emerald-500',
            textColor: 'text-emerald-500'
        },
        { color: 'bg-red-500', borderColor: 'border-red-500', textColor: 'text-red-500' },
        { color: 'bg-orange-500', borderColor: 'border-orange-500', textColor: 'text-orange-500' }
    ];

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
        const theme = colorThemes[index % colorThemes.length];
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

    function handleDragStart(
        event: DragEvent,
        item: Card,
        source: 'palette' | 'grid',
        index: number
    ) {
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

    function handleDragOver(event: DragEvent, index: number) {
        event.preventDefault(); // Necessary to allow dropping
        if (event.dataTransfer) {
            event.dataTransfer.dropEffect = 'move';
        }
        dragOverIndex = index;
    }

    function handleDrop(event: DragEvent, targetIndex: number) {
        event.preventDefault();
        if (!draggedItemInfo.item) return;

        const { item, source, sourceIndex } = draggedItemInfo;
        const targetItem = gridCells[targetIndex];

        // Case 1: Dragging from the palette to the grid
        if (source === 'palette') {
            // Place the new card. The old card (if any) will be
            // displaced and automatically reappear in the palette's placeholder.
            gridCells[targetIndex] = item;
            // Case 2: Dragging from within the grid (rearranging)
        } else if (source === 'grid') {
            // Simple swap logic
            gridCells[targetIndex] = item;
            gridCells[sourceIndex] = targetItem; // targetItem can be null
        }

        // Clean up the drag-over visual indicator
        dragOverIndex = -1;
    }

    function handlePaletteDrop(event: DragEvent) {
        event.preventDefault();
        if (!draggedItemInfo.item || draggedItemInfo.source !== 'grid') return;

        // Remove the item from the grid. It will automatically reappear in the palette.
        gridCells[draggedItemInfo.sourceIndex] = null;
    }

    function handleReturnCard(event: MouseEvent, cardId: number) {
        event.preventDefault(); // Prevent context menu
        const index = gridCells.findIndex((cell) => cell?.id === cardId);
        if (index !== -1) {
            gridCells[index] = null;
        }
    }
</script>

<!-- TEMPLATE -->
<div class="flex flex-row gap-8 w-full h-screen p-8 text-gray-200 bg-gray-900">
    <!-- Available Cards Palette (Left Side) -->
    <div
        class="w-1/4 flex-shrink-0 p-6 bg-gray-800 rounded-xl shadow-lg flex flex-col"
        ondragover={(e) => e.preventDefault()}
        ondrop={handlePaletteDrop}
    >
        <h2 class="text-xl font-bold border-b border-gray-700 pb-3 mb-4 flex-shrink-0">
            Available Cards
        </h2>
        <div class="flex flex-row flex-wrap gap-4 overflow-y-auto pr-2">
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
                        class="p-4 rounded-lg font-medium cursor-grab text-white text-center transition-all duration-200 select-none w-[calc(50%-0.5rem)] min-h-[90px] flex items-center justify-center {card.color} {isDragging
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
                <p class="text-gray-400 text-center py-4 w-full">All cards are on the grid!</p>
            {/if}
        </div>
    </div>

    <!-- Drop Grid (Right Side) -->
    <div class="flex-1 flex p-6 bg-gray-800 rounded-xl shadow-lg">
        <div class="grid grid-cols-5 grid-rows-5 gap-4 w-full h-full">
            {#each gridCells as cell, i}
                <div
                    class="bg-gray-700/50 border-2 border-dashed border-gray-600 rounded-lg flex justify-center items-center transition-colors {dragOverIndex ===
                    i
                        ? 'bg-gray-700 border-blue-500'
                        : ''}"
                    ondragover={(e) => handleDragOver(e as DragEvent, i)}
                    ondragleave={() => (dragOverIndex = -1)}
                    ondrop={(e) => handleDrop(e as DragEvent, i)}
                >
                    {#if cell}
                        {@const isDragging = draggedItemInfo.item?.id === cell.id}
                        <div
                            class="p-4 rounded-lg font-medium cursor-grab text-white text-center transition-all duration-200 select-none w-full h-full flex items-center justify-center {cell.color} {isDragging
                                ? 'opacity-40'
                                : ''}"
                            draggable="true"
                            ondragstart={(e) => handleDragStart(e as DragEvent, cell, 'grid', i)}
                            ondragend={handleDragEnd}
                            oncontextmenu={(e) => handleReturnCard(e as MouseEvent, cell.id)}
                        >
                            <span class="pointer-events-none">{cell.content}</span>
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    </div>
</div>
