<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"

  let name = "";
  let response = ""

  async function getEnv(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let fromRust = await invoke("get_env", { name })
    let fromVite = import.meta.env[name]

    response += `from rust: "${fromRust}"\n`;
    response += `from vite: "${fromVite}"\n`;
    response += `\n`;
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={getEnv}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <pre>{response}</pre>
</div>