<script lang="ts">
    import { Tooltip } from '@skeletonlabs/skeleton-svelte';
    import type { Snippet } from 'svelte';

    let { children, text }: { children: Snippet; text: Snippet | string } = $props();
</script>

<Tooltip
    arrow
    openDelay={750}
    closeDelay={100}
    positioning={{ placement: 'top' }}
    contentBase="card preset-filled-surface-300-700 p-4 text-center"
    arrowBackground="var(--color-surface-300-700)"
>
    {#snippet trigger()}
        {@render children()}
    {/snippet}
    {#snippet content()}
        {#if typeof text === 'string'}
            {@html text}
        {:else}
            {@render text()}
        {/if}
    {/snippet}
</Tooltip>
