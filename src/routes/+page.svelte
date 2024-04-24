<script lang="ts">
	import { goto } from "$app/navigation";
	import CountdownCell from "$lib/countdownCell.svelte";
	import { message } from "@tauri-apps/api/dialog";
    import { invoke } from '@tauri-apps/api/tauri';
    import type { countDown } from '../common.ts';
   
    async function logToSystemConsole(message: any) {
        await invoke('log_to_console', { message });
    }

	// async function save_count_down(data: countDown) {
    //     await invoke('save_count_down', { data });
    // }

    async function load_data():Promise<countDown[]>{
        logToSystemConsole("about to load");

        try { 
            let file = 'CountDownData.txt'
            const JSONdata:string[] = await invoke('load_data', {file});  
            return JSONdata.map(str=> {
                return JSON.parse(str);
            })
        
        } catch(error) {
            console.error("Error loading data:", error);
            return [];
        }
    }

    const handleclick = () => {
        goto('/create');    
      
    }

    let cd:countDown[] = [];

    import { onMount } from 'svelte';
    import { countdown_list } from "../stores.ts";

    onMount(async () => {
        await countdown_list.subscribe((data)=> {
            cd = data;
        });

        if (cd.length == 0){
            const data = await load_data();
            countdown_list.update(() => data);
        }

        await countdown_list.update(()=> {
            return cd;
        });

		//cd = await load_data();
       // handleSort();
	});
 

    function handleSort(){

        cd.sort((a,b) => {
            if ((a.date) > (b.date)){
                return -1;    
            } 
            return 1;});
        cd=cd;
    }

    function check_complete():boolean{
        for (let item of cd){
            if (item.date < new Date().getTime()){
                console.log("false");                
                return false;
            }
        }

        console.log("true");
        
        return true;
    }

    // let data:countDown[] =  [   {name:'Film', emoji: '🎞️', date: 1810989088312, colour:'green'},
    //                             {name:'TV show', emoji: '📺', date: 1810989088312, colour:'purple'},
    //                             {name:'Book', emoji: '📖', date: 1810989088312, colour:'Yellow'},
    //                             {name:'Movie', emoji: '🎞️', date: 1810989088312, colour:'Grey'},
    //                             {name:'Game', emoji: '🎮', date: 1810189088312, colour:'blue'},
    //                             {name:'Holiday', emoji: '🏖️', date: 1810989018312, colour:'orange'}, ]
                                
</script>


<div class="button-container">
    <button class="date_button">Countdowns</button>
    <button type="button" on:click={handleSort} class="sort_button">Sort</button>
</div>


<div class="list_box">
    {#if cd}
        {#key cd}
            {#each cd as item (item)}
                {#if (item.date - new Date().getTime())>0}
                    <CountdownCell cdData = {item}/>
                {/if}
            {/each}
            {#if (cd.length>0 && !check_complete())}
                <hr class="div-with-hr">
            {/if}
            {#each cd as item (item)}
                {#if (item.date - new Date().getTime())<0}
                    <CountdownCell cdData = {item}/>
                {/if}
            {/each}      
        {/key}
    {:else}
    <p>Loading data...</p>
    {/if}
</div>

<div class="add_container">
    <button on:click={handleclick} class="add_item_button">
        <img class="plus" src="/plus.png" alt="plus" />
    </button>
</div>


