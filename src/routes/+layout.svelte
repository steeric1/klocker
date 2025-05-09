<script>
    import "../app.css";
    import * as Menubar from "$lib/components/ui/menubar";
    import { goto } from "$app/navigation";
    import { register } from "@tauri-apps/plugin-global-shortcut";

    const { children } = $props();

    register("CommandOrControl+N", () => {
        goto("/create-project");
    });
</script>

<Menubar.Root
    class="fixed w-full m-0 rounded-none h-7 p-0 border-0 border-b-[1px] border-solid border-b-gray-300"
>
    <Menubar.Menu>
        <Menubar.Trigger class="font-normal px-2 hover:bg-gray-100">
            File
        </Menubar.Trigger>
        <Menubar.Content
            sideOffset={0}
            class="p-0 rounded-none border-gray-300"
        >
            <Menubar.Item onclick={() => goto("/create-project")}>
                Create Project <Menubar.Shortcut>Ctrl+N</Menubar.Shortcut>
            </Menubar.Item>
        </Menubar.Content>
    </Menubar.Menu>
</Menubar.Root>

<div class="pt-7 h-full">
    {@render children()}
</div>
