<script lang="ts">
  import * as Form from "$lib/components/ui/form/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { formSchema, type FormSchema } from "./schema";
  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import { zod4 } from 'sveltekit-superforms/adapters';
 
  let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
    $props();
 
  const form = superForm(data.form, {
    validators: zod4(formSchema),
  });
 
  const { form: formData, enhance } = form;
</script>
 
<form method="POST" use:enhance enctype="multipart/form-data">
  <Form.Field {form} name="file">
    <Form.Control>
      {#snippet children({ props })}
        <Form.Label>Asset to upload.</Form.Label>
        <Input {...props} type="file"/>
      {/snippet}
    </Form.Control>
    <Form.Description>Upload new asset.</Form.Description>
    <Form.FieldErrors />
  </Form.Field>
  <Form.Button>Upload</Form.Button>
</form>