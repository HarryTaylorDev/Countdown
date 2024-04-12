<script lang="ts">
    import { goto } from "$app/navigation";
        const handleclick = () => {
            goto('/');    
    }

    interface countDown {
        name:String;
        emoji:string;
        date:number;
        colour:string;
    }

    import {edit_data} from '../../stores.ts';
    let data:countDown;
    edit_data.subscribe((value) => {
        data = value;
    });

    let cd:countDown[] = [];
    import { countdown_list } from "../../stores.ts";

    countdown_list.subscribe((data)=> {
            cd = data;
    });
    
    
	import { invoke } from '@tauri-apps/api/tauri';
    async function save_count_down(data: countDown[]) {
		let jsonstrings:string[]=[]
		data.forEach(line=>{
			jsonstrings.push(JSON.stringify(line));
		});
        await invoke('save_to_file', {jsonstrings});
		
    }
    

    function delete_countdown(){
        const index = cd.indexOf(data);
        if (index > -1) { // only splice array when item is found
            cd.splice(index, 1); // 2nd parameter means remove one item only
        }
        
        save_count_down(cd);

        goto('/');  

    }

</script>

<body style="background-color: {data.colour};">
    <button on:click={handleclick}>home</button> <br>


    <p class="cell_text">{data.emoji}</p>
    <p class="cell_text">{data.name}</p>

    <button on:click={delete_countdown}>delete</button>
</body>
