<script>
	import { goto } from "$app/navigation";

    const handleclick = () => {
        goto('/create');
    }
</script>

<div class="button-container">
    <button class="date_button">Countdowns</button>
    <button class="sort_button">Sort</button>
</div>

<div class="list_box">

</div>


<div class="add_container">
    <button on:click={handleclick} class="add_item_button">
        <img class="plus" src="/plus.png" alt="plus" />
    </button>
</div>


