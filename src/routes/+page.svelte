<script>
	import { goto } from "$app/navigation";
	import CountdownCell from "$lib/countdownCell.svelte";
    import { invoke } from '@tauri-apps/api/tauri';

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

    let data=[  {name:'Film', emoji: '🎞️', date: '1709497026699', colour:'green'},
                {name:'TV show', emoji: '📺', date: '1709497026752', colour:'Red'},
                {name:'Book', emoji: '📖', date: '1709497026753', colour:'Yellow'},
                {name:'Movie', emoji: '🎞️', date: '170949724021', colour:'Grey'},
                {name:'Game', emoji: '🎮', date: '1709754129', colour:'blue'},
                {name:'Holiday', emoji: '🏖️', date: '1719497325654', colour:'orange'}, ]


</script>

<div class="button-container">
    <button class="date_button">Countdowns</button>
    <button on:click={handleSort} class="sort_button">Sort</button>
</div>

{#key data}
    <div class="list_box">
        {#each data as item (item)}
            <CountdownCell countdown_Data={item}/>
        {/each}
    </div>
{/key}


<div class="add_container">
    <button on:click={handleclick} class="add_item_button">
        <img class="plus" src="/plus.png" alt="plus" />
    </button>
</div>


