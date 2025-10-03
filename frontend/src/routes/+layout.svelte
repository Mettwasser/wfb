<script lang="ts">
    import { io } from 'socket.io-client';
    import '../app.css';

    let { children } = $props();
</script>

{@render children?.()}
