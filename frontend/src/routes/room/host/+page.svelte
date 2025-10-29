<script lang="ts">
    import { goto } from '$app/navigation';
    import { socket } from '$lib';
    import Tooltip from '$lib/components/Tooltip.svelte';
    import { initSessionByHosting, session } from '$lib/session.svelte';
    import { mapServerCardToCard } from '$lib/utils';
    import { FolderInput, FolderOutput } from '@lucide/svelte';

    let importInput: HTMLInputElement;

    let username = $state('');
    let cards = $state(['', '', '']);

    let isValid = $derived(
        username.trim() !== '' && cards.length === 25 && cards.every((card) => card.trim() !== '')
    );

    // Function to add a new card input
    function addCard() {
        cards = [...cards, ''];
    }

    // Function to remove a card
    function removeCard(index: number) {
        cards = cards.filter((_, i) => i !== index);
    }

    async function handleImport(event: Event) {
        const input = event.currentTarget as HTMLInputElement;
        const files = input.files;

        if (!files || files.length === 0) {
            return;
        }

        const file = files[0];

        try {
            const fileText = await file.text();

            let split = fileText.split('\n').map((line) => line.trim());

            if (split.length < 25) {
                split = split.slice(0, 25);
            }

            cards = split;
        } catch (err) {
            console.error('File parsing error:', err);
        }
    }

    async function handleExport(event: Event) {
        // 1. Create a blob from the text
        const blob = new Blob([cards.join('\n')], { type: 'text/plain' });

        // 2. Create an object URL for the blob
        const url = URL.createObjectURL(blob);

        // 3. Create a temporary anchor element
        const a = document.createElement('a');
        a.href = url;
        a.download = 'cards.txt'; // The file name for the download
        a.style.display = 'none'; // Make it invisible

        // 4. Append to the DOM, click it, and then remove it
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);

        // 5. Clean up by revoking the object URL
        URL.revokeObjectURL(url);
    }

    async function createLobby() {
        let response = await socket.emitWithAck('hostLobby', {
            hostName: username.trim(),
            cards: cards.map((card) => card.trim()),
        });

        if (response.success) {
            initSessionByHosting(
                response.data.lobbyId,
                username,
                mapServerCardToCard(response.data.cards)
            );
            goto(`/room/wait`);
        }
    }
</script>

{#snippet importExportButtons()}
    <div>
        <Tooltip text="Export">
            <button class="btn preset-filled-secondary-300-700" onclick={handleExport}>
                <FolderOutput class="size-6" />
            </button>
        </Tooltip>
        <Tooltip text="Import">
            <button class="btn preset-filled-primary-300-700" onclick={() => importInput.click()}>
                <FolderInput class="size-6" />
            </button>
        </Tooltip>
    </div>
{/snippet}

{#snippet cardCreationGrid()}
    <div
        class="grid gap-y-2 grid-cols-2 gap-x-8"
        onkeydown={(e) => {
            if (e.key === 'Enter' && cards.length < 25) {
                addCard();
                // Wait for the DOM to update
                setTimeout(() => {
                    const inputs: NodeListOf<HTMLInputElement> =
                        document.querySelectorAll('input[data-card-grid]');
                    inputs[inputs.length - 1]?.focus();
                }, 0);
            } else if ((e.altKey || e.ctrlKey) && (e.key == 'Delete' || e.key == 'Backspace')) {
                removeCard(cards.length - 1);
                setTimeout(() => {
                    const inputs: NodeListOf<HTMLInputElement> =
                        document.querySelectorAll('input[data-card-grid]');
                    inputs[inputs.length - 1]?.focus();
                }, 0);
            }
        }}
    >
        {#each cards as card, i}
            <div class="flex gap-2">
                <input
                    data-card-grid
                    type="text"
                    placeholder="Enter card text"
                    class="input flex-1"
                    bind:value={cards[i]}
                />
                <button
                    class="btn preset-filled-error-300-700"
                    onclick={() => removeCard(i)}
                    tabindex="-1"
                >
                    Remove
                </button>
            </div>
        {/each}
    </div>
{/snippet}

<div class="flex justify-center items-center min-h-full">
    <div class="flex flex-col gap-12 w-1/2 p-4">
        <div class="flex justify-center">
            <h1 class="h1">Host a lobby</h1>
        </div>

        <!-- Username input -->
        <label class="label">
            <span class="label-text">Username</span>
            <input
                type="text"
                placeholder="Enter your username"
                class="input"
                autocomplete="off"
                bind:value={username}
            />
        </label>

        <!-- Cards section -->
        <div class="space-y-4">
            <div class="flex w-full justify-between items-end">
                <span class="label-text">Cards ({cards.length}/25)</span>

                <input type="file" class="hidden" bind:this={importInput} onchange={handleImport} />

                {@render importExportButtons()}
            </div>

            {@render cardCreationGrid()}

            {#if cards.length < 25}
                <button class="btn preset-filled-primary-300-700" onclick={addCard}>
                    Add Card
                </button>
            {/if}
        </div>

        <!-- Continue button -->
        <div class="flex flex-col justify-end items-end gap-4">
            <button
                class="btn preset-filled-success-300-700 w-full"
                disabled={!isValid}
                onclick={createLobby}
            >
                Continue
            </button>

            {#if !isValid}
                <p class="text-error-500">
                    Please provide a username and 25 non-empty cards to continue
                </p>
            {/if}
        </div>
    </div>
</div>
