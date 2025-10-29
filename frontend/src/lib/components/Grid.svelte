<script lang="ts">
    import type { Card, DraggedItemInfo } from '$lib';

    interface Props {
        gridCells: (Card | null)[];
        draggedItemInfo: DraggedItemInfo;
        dragOverIndex: number;
    }

    let {
        gridCells = $bindable(),
        draggedItemInfo = $bindable(),
        dragOverIndex = $bindable(),
    }: Props = $props();

    function handleDragStart(event: DragEvent, item: Card, source: 'grid', index: number) {
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

    function handleReturnCard(event: MouseEvent, cardId: number) {
        event.preventDefault(); // Prevent context menu
        const index = gridCells.findIndex((cell: Card | null) => cell?.id === cardId);
        if (index !== -1) {
            gridCells[index] = null;
        }
    }
</script>

<div class="bg-surface-900 card flex flex-1 p-6">
    <div class="grid h-full w-full grid-cols-5 grid-rows-5 gap-4">
        {#each gridCells as cell, i}
            <div
                class="bg-surface-800/50 border-surface-700 flex items-center justify-center rounded-lg border-2 border-dashed transition-colors {dragOverIndex ===
                i
                    ? 'bg-surface-800 border-blue-500'
                    : ''}"
                ondragover={(e) => handleDragOver(e as DragEvent, i)}
                ondragleave={() => (dragOverIndex = -1)}
                ondrop={(e) => handleDrop(e as DragEvent, i)}
            >
                {#if cell}
                    {@const isDragging = draggedItemInfo.item?.id === cell.id}
                    <div
                        class="flex h-full w-full cursor-grab items-center justify-center rounded-lg p-4 text-center font-medium text-white transition-all duration-200 select-none {cell.color} {isDragging
                            ? 'opacity-40'
                            : ''}"
                        draggable="true"
                        ondragstart={(e) => handleDragStart(e as DragEvent, cell, 'grid', i)}
                        ondragend={handleDragEnd}
                        oncontextmenu={(e) => handleReturnCard(e as MouseEvent, cell.id)}
                    >
                        <span class="pointer-events-none">{cell.description}</span>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>
