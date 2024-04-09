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
	
	import { onMount } from 'svelte';

	var emoji = '😀'

	onMount(() => {
		const emojiPicker = document.querySelector('emoji-picker');
		if (emojiPicker) {			
			emojiPicker.addEventListener('emoji-click', event => emoji = event.detail.unicode ?? '');
		} else {
			console.error('emoji-picker element not found');
		}
	});
	//////////////////////////////////////////////////////////////////////////

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

		save_count_down(data)
		goto('/');  
		// logToSystemConsole("new CD")
		// logToSystemConsole(data.name)
		// logToSystemConsole(data.emoji)
		// logToSystemConsole(data.date.toString())
		// logToSystemConsole(data.colour)
		//logToSystemConsole("\n")
	}

	function testing(){
		var data:countDown = {name:'Film', emoji: '🎞️', date: 1810989088312, colour:'green'};
		save_count_down(data);

	}

	import { goto } from "$app/navigation";
	const handleclick = () => {
		logToSystemConsole("hello");
        goto('/');    
    }

</script>


<button type="button" on:click={testing}>TEST</button>


<div class="form">
	<form on:submit|preventDefault={handleSubmit}>
		<div class="button-container">
			<button class="form_buttons" type="button" on:click={handleclick}>Home</button>
			<button class="form_buttons" type="submit">Submit</button><br>
		</div>
		<!-- Name -->
		<label for="name">Name:</label><br>
		<input type="text" id="name" name="name" required><br><br>
	
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
