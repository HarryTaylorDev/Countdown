<script lang="ts">
    import { goto } from "$app/navigation";
        const handleclick = () => {
            goto('/');    
    }

    import type { countDown } from '../../common.ts';

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
    async function save_count_down(fileName: string, data: countDown[]) {
		let jsonstrings:string[]=[]
		data.forEach(line=>{
			jsonstrings.push(JSON.stringify(line));
		});
        await invoke('save_to_file', {fileName, jsonstrings});
		
    }
    

    function delete_countdown(){
        const index = cd.indexOf(data);
        if (index > -1) { // only splice array when item is found
            cd.splice(index, 1); // 2nd parameter means remove one item only
        }
        
        save_count_down("CountDownData.txt",cd);

        goto('/');  

    }
    /////////////////////////
    import ColorPicker, { ChromeVariant } from 'svelte-awesome-color-picker';
	import { colord } from "colord";

	let hex = "#000000"

	let rgb = {
		"r": 72,
		"g": 72,
		"b": 72,
		"a": 1
	}	

	let color = colord("#ff0000");
    //////////////////////////////////
    import 'emoji-picker-element';
	import { onMount } from 'svelte';

	var emoji = '😀'
	onMount(async () => {
		const emojiPicker = document.querySelector('emoji-picker');
		if (emojiPicker) {			
			emojiPicker.addEventListener('emoji-click', event => emoji = event.detail.unicode ?? '');
		} else {
			console.error('emoji-picker element not found');
		}

		await countdown_list.subscribe((data)=> {
            cd = data;
        });
        emoji = data.emoji;
        hex = data.colour;
	});

    function formatDate(num:number) {
        let date:Date = new Date(num);
        const year = date.getFullYear();
        const month = (date.getMonth() + 1).toString().padStart(2, '0'); 
        const day = date.getDate().toString().padStart(2, '0');

        return `${year}-${month}-${day}`; 
    }

    function handleupdate(e:SubmitEvent){
        const formData = new FormData(e.target as HTMLFormElement);
		
		var formDate:number = new Date(formData.get('date')?.toString()!).getTime();
		
		let newdata:countDown = {
			name: formData.get('name')?.toString()!,
			emoji:emoji,
			date:formDate,
			colour:hex
		}

        var index = cd.indexOf(data);

        if (index !== -1) {
            cd[index] = newdata;
        }
        
		// cd.push(data);
		save_count_down("CountDownData.txt",cd);
        goto("/");
    }

</script>
<page style="background-color: {hex};"></page>
<body style="background-color: {hex};">
    <button on:click={handleclick}>home</button> 
    

    <!-- ////////////////////////////////////////////// -->
    <form on:submit|preventDefault={handleupdate}>
        <button type="submit">Update</button> <br>
        <label for="name">Name:</label><br>
		<input autocomplete="off" type="text" id="name" name="name" value = {data.name} required><br><br>
	
        <hr class="div-with-hr">

		<!-- Emoji -->
		<label for="emoji">Emoji:{emoji}</label><br>
		<emoji-picker></emoji-picker>
	
        <hr class="div-with-hr">

		<!-- Date -->
		<label for="date">Date:</label><br> 
		<input type="date" id="date" name="date" value = {formatDate(data.date)} required><br><br>
	
        <hr class="div-with-hr">

		<!-- Colour -->
		<label for="color">Favorite Color:</label><br>
		<div class = "colour" style="background-color: {hex};"></div>
		<div class="colourPicker">
			<ColorPicker 				
				bind:hex		
				isDialog={false} 
				components={ChromeVariant} 
				sliderDirection="horizontal"/><br>
		</div>

    </form>


    <button on:click={delete_countdown}>delete</button> <br>
</body>
