<script lang="ts">
	import { goto } from "$app/navigation";
	import CountdownCell from "$lib/countdownCell.svelte";
    import { invoke } from '@tauri-apps/api/tauri';

    interface countDown {
        name:number;
        emoji:string;
        date:string;
        colour:string;
    }

    const handleclick = () => {
        goto('/create');    
    }

    const handleSort = () => {
        invoke('log_to_console', { data }); 
        data.sort((a,b) => {
            if (parseInt(a.date) > parseInt(b.date)){
                return -1;    
            } 
            return 1;    });
        data=data;
        invoke('log_to_console', { data }); 
    }

    let data=[  {name:'Film', emoji: '🎞️', date: '1810989088312', colour:'green'},
                {name:'TV show', emoji: '📺', date: '1810989088312', colour:'Red'},
                {name:'Book', emoji: '📖', date: '1810989088312', colour:'Yellow'},
                {name:'Movie', emoji: '🎞️', date: '1810989088312', colour:'Grey'},
                {name:'Game', emoji: '🎮', date: '1810189088312', colour:'blue'},
                {name:'Holiday', emoji: '🏖️', date: '1810989018312', colour:'orange'}, ]


</script>

<div class="button-container">
    <button class="date_button">Countdowns</button>
    <button on:click={handleSort} class="sort_button">Sort</button>
</div>

{#key data}
    <div class="list_box">
        {#each data as item (item)}
            <CountdownCell cdData = {item}/>
        {/each}
    </div>
{/key}


<div class="add_container">
    <button on:click={handleclick} class="add_item_button">
        <img class="plus" src="/plus.png" alt="plus" />
    </button>
</div>


