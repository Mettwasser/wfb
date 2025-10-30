<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/state';
    import { FRONTEND_URL, socket } from '$lib';
    import { session } from '$lib/session.svelte';
    import { Clipboard, LoaderCircle, SwordIcon } from '@lucide/svelte/icons';
    import { onMount } from 'svelte';

    const inviteLinkText = 'Copy invite link';

    let shouldCensorRoomId = $state(true);
    let copyBtnText = $state(inviteLinkText);

    function copyToClipboard() {
        navigator.clipboard.writeText(`${FRONTEND_URL}/room/${session.info!.lobbyId}/join`);
        copyBtnText = 'Copied!';
        setTimeout(() => (copyBtnText = inviteLinkText), 2000);
    }

    function nextStage() {
        socket.emit('triggerNextStage', session.info!.lobbyId);
    }

    onMount(() => {
        socket.on('userJoined', (user) => {
            session.info!.players.push(user);
        });

        socket.on('nextStage', (_) => {
            if (session.info!.isHost) {
                goto('/room/crafting/host');
            } else {
                goto('/room/crafting');
            }
        });

        return () => {
            socket.removeListener('userJoined');
            socket.removeListener('nextStage');
        };
    });
</script>

<div class="flex h-full flex-col items-center justify-center gap-y-5 px-4">
    <h2 class="flex items-center justify-center gap-x-5 text-3xl font-bold">
        <LoaderCircle class=" text-primary-500 size-8 animate-spin" />
        <div class="md:text-normal text-sm">Room joined, please wait...</div>
    </h2>
    <div class="flex flex-col items-center gap-2">
        Host:
        <div class="text-primary-100 text-xl font-bold">
            {session.info!.hostName}
        </div>
    </div>
    <div class="bg-surface-900 w-2/3 rounded-xl p-8 shadow-lg">
        <div class="grid max-h-150 grid-cols-5 gap-4 overflow-y-auto p-4">
            {#each session.info!.players as p (p)}
                <div>
                    <span
                        class="card flex justify-center gap-2 rounded-lg p-3 {p ===
                        session.info!.userName
                            ? 'bg-success-300-700 card'
                            : 'preset-tonal-surface'}"
                    >
                        <SwordIcon class="text-primary-100 inline pr-2" />{p}
                    </span>
                </div>
            {:else}
                <div class="text-center text-surface-400 col-span-full">No players yet</div>
            {/each}
        </div>
    </div>
    <div class="flex flex-col gap-4 text-center text-2xl">
        <div class="flex gap-x-2">
            Room <div
                class="!w-24 {shouldCensorRoomId
                    ? 'cursor-pointer bg-black text-transparent'
                    : 'text-primary-100 font-bold'}"
                onclick={() => (shouldCensorRoomId = !shouldCensorRoomId)}
            >
                {session.info!.lobbyId}
            </div>
        </div>

        <button class="btn preset-filled-primary-300-700" onclick={copyToClipboard}>
            <Clipboard />
            <p class="w-30">
                {copyBtnText}
            </p>
        </button>
    </div>
    {#if session.info!.isHost}
        <div class="flex w-2/3 justify-end">
            <button class="btn preset-gradient" onclick={nextStage}>Next Stage</button>
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
