<script lang="ts">
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
</script>


<dir class="grid auto-rows-min grid-cols-2 gap-4">
    {#each data.models as model}
        <span>{model.id}</span>
        <span>{model.name}</span>
    {/each}

</dir>