<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { goto } from "$app/navigation";

    interface Project {
        name: string;
        total_seconds: number;
        last_worked: number;
        num_tasks: number;
    }

    let projects = invoke<Project[]>("list_projects");
</script>

<main class="px-2 min-h-full flex flex-col items-center justify-center py-2">
    {#await projects then projects}
        {#if projects.length > 0}
            <Card.Root class="shadow-md mb-2 max-w-7xl">
                <Card.Header>
                    <Card.Title class="text-lg font-bold text-center">
                        You have {projects.length} project{projects.length > 1
                            ? "s"
                            : ""}
                    </Card.Title>
                </Card.Header>
                <Card.Content
                    class={[
                        "grid items-center gap-x-10 gap-y-6",
                        {
                            "grid-cols-1": projects.length === 1,
                            "grid-cols-2": projects.length > 1,
                        },
                    ]}
                >
                    {#each projects as project}
                        <div
                            class="grid grid-cols-[auto,1fr] items-center gap-x-6 gap-y-2"
                        >
                            <div>
                                <h3 class="font-semibold mb-1">
                                    {project.name}
                                </h3>
                                <div class="flex flex-col text-xs">
                                    <span>
                                        Hours spent: {project.total_seconds}
                                    </span>
                                    <span>Last worked: 2 hours ago</span>
                                    <span>Tasks: 3</span>
                                </div>
                            </div>
                            <Button class="font-normal">Open</Button>
                        </div>
                    {/each}
                </Card.Content>
            </Card.Root>
            <a class="text-sm text-blue-600" href="/create-project">
                Create project
            </a>
        {:else}
            <div class="flex flex-col gap-2 items-center">
                0 projects found
                <Button
                    class="text-base font-normalu"
                    onclick={() => goto("/create-project")}
                >
                    Create project
                </Button>
            </div>
        {/if}
    {/await}
</main>
