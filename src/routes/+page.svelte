<script>
	import { goto } from "$app/navigation";
	import CountdownCell from "$lib/countdownCell.svelte";
    import { invoke } from '@tauri-apps/api/tauri';

    const handleclick = () => {
        goto('/create');    
    }

    const handleSort = () => {
        invoke('log_to_console', { data }); 
        data.sort((a,b) => a.name.localeCompare(b.name));
        invoke('log_to_console', { data }); 
    }

    const data=[  {name:'atest', colour:'red'},{name:'ctest2',colour:'blue'},{name:'btest2',colour:'green'} ]
</script>

<div class="button-container">
    <button class="date_button">Countdowns</button>
    <button on:click={handleSort} class="sort_button">Sort</button>
</div>

{#key data}
    <div class="list_box">
        {#each data as item}
            <CountdownCell countdown_Data={item}/>
        {/each}
    </div>
{/key}


<div class="add_container">
    <button on:click={handleclick} class="add_item_button">
        <img class="plus" src="/plus.png" alt="plus" />
    </button>
</div>


