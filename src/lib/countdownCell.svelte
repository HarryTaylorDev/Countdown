<script>

    /**
	 * @type {{ date: string; colour: string; emoji: string; name: string; }}
	 */
     export let countdown_Data;
    import { onMount } from 'svelte';

    let currentDateTime = new Date();

    onMount(() => {
        const interval = setInterval(() => {
        
            currentDateTime = new Date();
            daysRemaining = calcDiff(currentDateTime.getTime().toString(), countdown_Data.date).toString(); 
        }, 1000);

        return () => {
            clearInterval(interval);
        };
    });

    /**
	 * @param {string} str1
	 * @param {string} str2
	 */
    function calcDiff(str1, str2) {
        const date1 = new Date(parseInt(str1));
        const date2 = new Date(parseInt(str2));
        // Calculate the difference in milliseconds
        const differenceInMs = date2.getTime() - date1.getTime();

        // Convert milliseconds to days (consider rounding if needed)
        const differenceInDays = (differenceInMs / (1000 * 60 * 60 * 24));

        return differenceInDays;
    }

    let daysRemaining = calcDiff(currentDateTime.getTime().toString(), countdown_Data.date).toString();

</script>

<div style="background-color: {countdown_Data.colour};" class="countdown_Cell">
        <p class="cell_text">{countdown_Data.emoji}</p>
        <p class="cell_text">{countdown_Data.name}</p>
        <p class="cell_text">{countdown_Data.date}</p>
        <p class="cell_text">{daysRemaining}</p>
</div>