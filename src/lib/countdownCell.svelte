<script lang="ts">

    interface countDown {
        name:number;
        emoji:string;
        date:string;
        colour:string;
    }

    export let cdData;
    let countdown_Data: countDown = cdData;
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


    function calcDiff(str1: string, str2: string) {
        const date1 = new Date(parseInt(str1));
        const date2 = new Date(parseInt(str2));
        // Calculate the difference in milliseconds
        const differenceInMs = date2.getTime() - date1.getTime();

        // Convert milliseconds to days (consider rounding if needed)
        const differenceInDays = (differenceInMs / (1000 * 60 * 60 * 24));

        return differenceInDays.toFixed(1);
    }

  
    let daysRemaining = calcDiff(currentDateTime.getTime().toString(), countdown_Data.date).toString();

    
    let options: Intl.DateTimeFormatOptions = {
        year: 'numeric',
        month: 'numeric',
        day: 'numeric',
    };

    let readableDate = new Date(parseInt(countdown_Data.date)).toLocaleDateString(undefined, options);

</script>

<div style="background-color: {countdown_Data.colour};" class="countdown_Cell">
    <div class = "countdown_cell_style">
        <p class="cell_text">{countdown_Data.emoji}</p>
        <p class="cell_text">{countdown_Data.name}</p>
        <p class="cell_text">{readableDate}</p>
        <p class="cell_text">{daysRemaining} days left</p>
    </div>
</div>