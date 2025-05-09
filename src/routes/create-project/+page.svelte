<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import Label from "$lib/components/ui/label/label.svelte";
    import * as Card from "$lib/components/ui/card";
    import { open } from "@tauri-apps/plugin-dialog";
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";

    let path = $state<string | null>(null);
    let name = $state("");
    let error = $state("");

    async function locationPrompt() {
        path = await open({
            multiple: false,
            directory: true,
            canCreateDirectories: true,
        });
    }

    async function create() {
        if (!path || !name) {
            error = "Please provide values";
            return;
        }

        error = "";
        await invoke("create_project", { dir: path, name }).catch(
            (err: string) => (error = err),
        );
        if (error) return;

        goto("/");
    }

    let nameInput = $state<HTMLInputElement | null>(null);
    $effect(() => {
        if (nameInput) nameInput.setAttribute("autocomplete", "nope");
    });
</script>

<main class="px-2 py-1 h-full flex flex-col items-center justify-center">
    <Card.Root class="shadow-md mb-2 max-w-7xl">
        <Card.Header>
            <Card.Title class="text-lg font-bold text-center">
                Create project
            </Card.Title>
        </Card.Header>
        <Card.Content class="flex flex-col gap-4 items-start">
            <div class="grid grid-cols-[auto,1fr] gap-x-4 gap-y-2 items-center">
                <Label for="location" class="font-normal">
                    Select location:
                </Label>
                <div class="flex items-center gap-2">
                    {#if path}
                        <span class="truncate text-sm">{path}</span>
                    {/if}
                    <Button
                        class="h-8 px-3 font-normal shadow-sm"
                        variant="secondary"
                        onclick={locationPrompt}
                        id="location"
                    >
                        {!path ? "Browse" : "Change"}
                    </Button>
                </div>
                <Label for="__my_name" class="font-normal">Name:</Label>
                <Input
                    id="__my_name"
                    name="__my_name"
                    class="focus-visible:ring-transparent focus-visible:border-gray-400"
                    bind:ref={nameInput}
                    placeholder="My Project..."
                    bind:value={name}
                />
            </div>
            <div class="flex gap-1.5 items-center text-sm">
                <Button class="h-8 px-3 font-normal" onclick={create}>
                    Create
                </Button>
                {#if error}
                    <span class="text-red-700">{error}</span>
                {/if}
            </div>
        </Card.Content>
    </Card.Root>
    <a class="text-sm text-blue-600" href="/">Back to Projects</a>
</main>
