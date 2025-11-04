<script lang="ts">
    import { goto } from '$app/navigation';
    import { LobbyState, socket } from '$lib';
    import { initSessionByJoining } from '$lib/session.svelte';
    import { mapServerCardsToCards } from '$lib/utils';
    import type { PageData } from './$types';
    let { params }: PageData = $props();

    let roomId = $state(params.roomId ?? '');
    let name = $state('');

    let nameError = $state('');

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();

        let response = await socket.emitWithAck('joinLobby', {
            lobbyId: roomId,
            playerName: name.trim(),
        });

        if (response.success) {
            let { host, cards, players } = response.data;
            initSessionByJoining({
                cards: mapServerCardsToCards(cards),
                players,
                hostName: host,
                userName: name.trim(),
                lobbyId: roomId,
                isHost: false,
                state: LobbyState.WaitingForPlayers,
                correctAnswers: [],
            });
            goto(`/room/wait`);
        }
    }

    // A derived state to check if the button should be disabled
    let isButtonDisabled = $derived(!roomId.trim() || !name.trim());
</script>

<div class="flex h-full flex-col items-center justify-center">
    <div class="bg-surface-900 w-full max-w-md space-y-6 rounded-xl p-8 shadow-lg">
        <h2 class="text-surface-50 text-center text-3xl font-bold">Join a Room</h2>
        <form onsubmit={handleSubmit} class="space-y-6">
            <label class="label">
                <span class="label-text">Room ID</span>
                <input
                    type="text"
                    class="input"
                    placeholder="Enter Room ID..."
                    bind:value={roomId}
                />
            </label>
            <label class="label">
                <span class="label-text">Name</span>
                <input type="text" class="input" placeholder="Enter Name..." bind:value={name} />
                <span class="label-text">{nameError}</span>
            </label>
            <button
                type="submit"
                disabled={isButtonDisabled}
                class="btn preset-filled-primary-500 w-full"
            >
                Join Room
            </button>
        </form>
    </div>
</div>
