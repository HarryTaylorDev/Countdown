<script lang="ts">
	///////////////////////////////////////////////////////////////////////////
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

	//////////////////////////////////////////////////////////////////////////
	import 'emoji-picker-element';
	import { countdown_list } from "../../stores.ts";
	import { onMount } from 'svelte';

	var emoji = '😀'
	let cd:countDown[] = [];

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
		// console.log('testing');
		// console.log(cd);
		

	});
	//////////////////////////////////////////////////////////////////////////

	import { invoke } from '@tauri-apps/api/tauri';

    async function logToSystemConsole(message: any) {
        await invoke('log_to_console', { message });
    }



	async function save_count_down(data: countDown[]) {
		let jsonstrings:string[]=[]
		data.forEach(line=>{
			jsonstrings.push(JSON.stringify(line));
		});
        await invoke('save_to_file', {jsonstrings});
		
    }

	interface countDown {
        name:string;
        emoji:string;
        date:number;
        colour:string;
    }

	export let data:countDown = {
		name:'',
		emoji:'',
		date:0,
		colour:''
	};


	function handleSubmit(e:SubmitEvent){
		
		const formData = new FormData(e.target as HTMLFormElement);
		
		var formDate:number = new Date(formData.get('date')?.toString()!).getTime();
		
		data = {
			name: formData.get('name')?.toString()!,
			emoji:emoji,
			date:formDate,
			colour:hex
		}

		cd.push(data);
		save_count_down(cd);
		goto('/');  

	}

	import { goto } from "$app/navigation";
	import { json } from '@sveltejs/kit';
	const handleclick = () => {
		logToSystemConsole("hello");
        goto('/');    
    }

</script>


<div class="form">
	<form on:submit|preventDefault={handleSubmit}>
		<div class="button-container">
			<button class="form_buttons" type="button" on:click={handleclick}>Home</button>
			<button class="form_buttons" type="submit">Submit</button><br>
		</div>
		<!-- Name -->
		<label for="name">Name:</label><br>
		<input autocomplete="off" type="text" id="name" name="name" required><br><br>
	
		<!-- Emoji -->
		<label for="emoji">Emoji:{emoji}</label><br>
		<emoji-picker></emoji-picker>
	
		<!-- Date -->
		<label for="date">Date:</label><br> 
		<input type="date" id="date" name="date" required><br><br>
	
		<!-- Colour -->
		<label for="color">Favorite Color:</label><br>
		<div class = "colour" style="background-color: {hex};"></div>
		<div class="colourPicker">
			<ColorPicker 				
				bind:hex
				bind:rgb
				bind:color
				isDialog={false} 
				components={ChromeVariant} 
				sliderDirection="horizontal"/><br>
		</div>
		

	  </form>
</div>
