<script>
	import { goto } from "$app/navigation";
	import CountdownCell from "$lib/countdownCell.svelte";

    const handleclick = () => {
        goto('/create');
    }

    const data=[  {name:'test', colour:'red'},{name:'test2',colour:'blue'},{name:'test2',colour:'green'} ]
</script>

<div class="button-container">
    <button class="date_button">Countdowns</button>
    <button class="sort_button">Sort</button>
</div>

<div class="list_box">
    {#each data as item}
        <CountdownCell countdown_Data={item}/>
    {/each}
</div>


<div class="add_container">
    <button on:click={handleclick} class="add_item_button">
        <img class="plus" src="/plus.png" alt="plus" />
    </button>
</div>


