<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  // https://tauri.app/v1/api/js/shell/#example-scope-configuration
  import { Command } from "@tauri-apps/plugin-shell";
  // https://kit.svelte.dev/docs/modules#$env-dynamic-public
  import { env } from '$env/dynamic/public';

  let name = "";
  let response = ""

  const getEnvShell = async (arg:string|string[]) => {
    try {
      const cmd = await Command.create(
          "run-printenv",
          arg
      ).execute();
    
      if(cmd.code == 0) {
        return cmd.stdout.trim()
      }
    }catch(err){
      console.error('getEnvShell', err);
    }

    return ""
  }

  const getPrivateSvelteKit = async (name:string) => {
		try {
      const response = await fetch(`/api/getEnv/?name=${name}`);
    
      const {variable} = await response.json();
      return variable
    }catch(err){
      console.error('getPrivateSvelteKit', err);
    }

    return ""
  }

  const getFromRust = async (name:string) => {
    try{
      return await invoke("get_env", { name })
    }catch(err){
      console.error('getFromRust', err);
    }

    return ""
  }

  const getFromVite = (name:string) => {
    try {
      return import.meta.env[name] || ""
    }catch(err){
      console.error('getFromVite', err);
    }

    return ""
  }

  const getFromSvelte = (name:string) => {
    try {
      return (env as any)[name] || ""
    }catch(err){
      console.error('getFromSvelte', err);
    }

    return ""
  }

  async function getEnv(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let fromRust = await getFromRust(name);
    let fromVite = getFromVite(name);
    let fromSvelte = getFromSvelte(name);
    let fromShell = await getEnvShell(name);
    let fromPrivateSvelte = await getPrivateSvelteKit(name);

    // console.log({
    //   fromPrivateSvelte,
    //   fromRust,
    //   fromShell,
    //   fromSvelte,
    //   fromVite
    // })

    response = `from rust: "${fromRust}"\n`;
    response += `from vite: "${fromVite}"\n`;
    response += `from sveltekit: "${fromSvelte}"\n`;
    response += `from private sveltekit: "${fromPrivateSvelte}"\n`;
    response += `from shell: "${fromShell}"\n`;
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