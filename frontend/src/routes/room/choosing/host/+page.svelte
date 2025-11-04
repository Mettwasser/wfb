<script lang="ts">
    import { socket } from '$lib';
    import { session } from '$lib/session.svelte';

    async function chooseCard(cardId: number) {
        await socket.emitWithAck('submitAnswer', {
            cardId,
            lobbyId: session.info.lobbyId,
        });
    }
</script>

<div class="flex min-h-full w-full items-center justify-center">
    <!-- Card choosing -->
    <div class="grid grid-cols-5 grid-rows-5 gap-8">
        {#each session.info.cards as card (card.id)}
            <div
                class="
                {session.info.correctAnswers.includes(card.id)
                    ? 'bg-success-800'
                    : 'bg-surface-700'}    
                flex w-52 cursor-pointer items-center justify-center rounded-lg p-4 text-center font-medium text-wrap text-white transition-all duration-200 select-none"
                onclick={() => chooseCard(card.id)}
            >
                {card.description}
            </div>
        {/each}
    </div>
</div>
