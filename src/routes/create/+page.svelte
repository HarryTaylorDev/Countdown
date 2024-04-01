<script lang="ts">
	///////////////////////////////////////////////////////////////////////////
	import ColorPicker, { ChromeVariant } from 'svelte-awesome-color-picker';
	import { colord } from "colord";

	let hex = "#217ae6"

	let rgb = {
		"r": 31,
		"g": 31,
		"b": 31,
		"a": 1
	}	

	let color = colord("#ff0000");

	//////////////////////////////////////////////////////////////////////////
	// import EmojiSelector from 'svelte-emoji-selector';

	// let textContent = '';

	// function onEmoji(event) {
	// 	textContent += event.detail;
	// }
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
	export let data = {};

</script>


<div class="form">
	<form>
		<div class="button-container">
			<button class="form_buttons" >Home</button><br>
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
