<script lang="ts">
	import { goto } from "$app/navigation";
	import CountdownCell from "$lib/countdownCell.svelte";
    import { invoke } from '@tauri-apps/api/tauri';
   
    async function logToSystemConsole(message: any) {
        await invoke('log_to_console', { message });
    }

	async function save_count_down(data: countDown) {
        await invoke('save_count_down', { data });
    }

    interface countDown {
        name:string;
        emoji:string;
        date:number;
        colour:string;
    }

    const handleclick = () => {
        logToSystemConsole("here");
        //save_count_down(data);
        goto('/create');    
      
    }



    
    function testing(){
        save_count_down(data[0]);
        
        data.sort((a,b) => {
            if ((a.date) > (b.date)){
                return -1;    
            } 
            return 1;});
        data=data;

	}

    function handleSort(){
        save_count_down(data[0]);
    
        logToSystemConsole("here");	
        //invoke('log_to_console', { message: "hello" });

        data.sort((a,b) => {
            if ((a.date) > (b.date)){
                return -1;    
            } 
            return 1;});
        data=data;
    }

    let data:countDown[] =  [   {name:'Film', emoji: '🎞️', date: 1810989088312, colour:'green'},
                                {name:'TV show', emoji: '📺', date: 1810989088312, colour:'purple'},
                                {name:'Book', emoji: '📖', date: 1810989088312, colour:'Yellow'},
                                {name:'Movie', emoji: '🎞️', date: 1810989088312, colour:'Grey'},
                                {name:'Game', emoji: '🎮', date: 1810189088312, colour:'blue'},
                                {name:'Holiday', emoji: '🏖️', date: 1810989018312, colour:'orange'}, ]
                                
</script>

<button type="button" on:click={testing}>TEST</button>


<div class="button-container">
    <button class="date_button">Countdowns</button>
    <button type="button" on:click={handleSort} class="sort_button">Sort</button>
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


