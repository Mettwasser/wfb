<script lang="ts">
    import type { PageData } from '../$types';

    let { params }: PageData = $props();

    let name = $state('');

    function handleSubmit(event: SubmitEvent) {
        event.preventDefault();

        if (!params.roomId.trim()) return;
        // In a real application, you would handle the room joining logic here.
        // For example, emitting an event or calling a service.
        alert(`Joining room: ${params.roomId} as ${name}`);
    }

    // A derived state to check if the button should be disabled
    let isButtonDisabled = $derived(!params.roomId.trim());
</script>

<div class="flex items-center justify-center flex-col h-full">
    <div class="w-full max-w-md p-8 space-y-6 bg-surface-900 rounded-xl shadow-lg">
        <h2 class="text-3xl font-bold text-center text-surface-50">Join a Room</h2>
        <form onsubmit={handleSubmit} class="space-y-6">
            <label class="label">
                <span class="label-text">Name</span>
                <input type="text" class="input" placeholder="Enter Name..." bind:value={name} />
            </label>
            <button
                type="submit"
                disabled={isButtonDisabled}
                class="btn preset-filled-primary-500 w-full"
            >
                <b>Join Room</b>
            </button>
        </form>
    </div>
</div>
