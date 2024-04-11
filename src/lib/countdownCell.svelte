<script lang="ts">

    interface countDown {
        name:number;
        emoji:string;
        date:number;
        colour:string;
    }

    export let cdData;
    let countdown_Data: countDown = cdData;
    import { onMount } from 'svelte';

    let currentDateTime = new Date();

    onMount(() => {
        const interval = setInterval(() => {
        
            currentDateTime = new Date();
            timeRemaining = calcDiff(currentDateTime.getTime(), countdown_Data.date).toString(); 
        }, 1000);

        return () => {
            clearInterval(interval);
        };
    });


    function calcDiff(time1: number, time2: number):string {
        const date1 = new Date(time1);
        const date2 = new Date(time2);
        // Calculate the difference in milliseconds
        const differenceInMs = date2.getTime() - date1.getTime();

        if (differenceInMs > (1000 * 60 * 60 * 24)){
            const differenceInDays = (differenceInMs / (1000 * 60 * 60 * 24));
            if (differenceInDays < 2){
                return Math.floor(differenceInDays).toString() + " Day Left";
            }
            return Math.floor(differenceInDays).toString() + " Days Left";
        }
        
        if(differenceInMs > (1000 * 60 * 60)){
            const differenceInHours = (differenceInMs / (1000 * 60 * 60));
            if (differenceInHours < 2){
                return Math.floor(differenceInHours).toString() + " Hour Left";
            }
            return Math.floor(differenceInHours).toString() + " Hours Left";
        }
        
        if(differenceInMs > (1000 * 60)){
            const differenceInMinutes = (differenceInMs / (1000 * 60));
            if (differenceInMinutes < 2){
                return Math.floor(differenceInMinutes).toString() + " Minute Left";
            }
            return Math.floor(differenceInMinutes).toString() + " Minutes Left";
        }

        if(differenceInMs > (1000)){
            const differenceInSeconds = (differenceInMs / (1000));
            if (differenceInSeconds < 2){
                return Math.floor(differenceInSeconds).toString() + " Second Left";
            }
            return Math.floor(differenceInSeconds).toString() + " Seconds Left";
        }
        return "Complete!"

    }

  
    let timeRemaining:string = calcDiff(currentDateTime.getTime(), countdown_Data.date).toString();

    
    let options: Intl.DateTimeFormatOptions = {
        year: 'numeric',
        month: 'numeric',
        day: 'numeric',
    };

    let readableDate = new Date(countdown_Data.date).toLocaleDateString(undefined, options);

</script>

<div style="background-color: {countdown_Data.colour};" class="countdown_Cell">
    <div class = "countdown_cell_style">
        <p class="cell_text">{countdown_Data.emoji}</p>
        <p class="cell_text">{countdown_Data.name}</p>
        <p class="cell_text">{readableDate}</p>
        <p class="cell_text">{timeRemaining}</p>
    </div>
</div>