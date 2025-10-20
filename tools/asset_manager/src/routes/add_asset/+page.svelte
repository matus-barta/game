<script lang="ts">
  import type { PageData } from "./$types.js";
  import Form from "./form.svelte";
  let { data }: { data: PageData } = $props();
</script>
 
<Form {data} />