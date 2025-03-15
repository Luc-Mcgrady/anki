<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    export let shown: boolean;
</script>

<div class="modal-header">
    <h5 class="modal-title"><slot /></h5>
    <button
        type="button"
        class="btn-close"
        aria-label="Close"
        on:click={() => (shown = false)}
    ></button>
</div>

<style>
    .modal-header {
        position: sticky;
        top: 0;
        background-color: var(--bs-body-bg);
        z-index: 100;
    }
</style>
