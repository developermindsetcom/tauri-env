<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  // https://tauri.app/v1/api/js/shell/#example-scope-configuration
  import { Command } from "@tauri-apps/plugin-shell";
  // https://kit.svelte.dev/docs/modules#$env-dynamic-public
  import { env } from '$env/dynamic/public';

  let name = "";
  let response = ""

  const getEnvShell = async (arg:string|string[]) => {
    const cmd = await Command.create(
        "run-printenv",
        arg
    ).execute();
  
    if(cmd.code == 0) {
      return cmd.stdout.trim()
    }

    return ""
  }

  const getPrivateSvelteKit = async (name:string) => {
		const response = await fetch('/api/getEnv', {
			method: 'POST',
			body: JSON.stringify({ name }),
			headers: {
				'content-type': 'application/json',
			},
		});
	
		const {variable} = await response.json();
    return variable
  }

  async function getEnv(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let fromRust = await invoke("get_env", { name })
    let fromVite = import.meta.env[name]
    let fromShell = await getEnvShell(name);
    let fromSvelte = (env as any)[name]
    let fromPrivateSvelte = await getPrivateSvelteKit(name);

    response = `from rust: "${fromRust}"\n`;
    response += `from vite: "${fromVite}"\n`;
    response += `from shell: "${fromShell}"\n`;
    response += `from sveltekit: "${fromSvelte}"\n`;
    response += `from private sveltekit: "${fromPrivateSvelte}"\n`;
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