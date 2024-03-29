<script>
	import ColorPicker, { ChromeVariant } from 'svelte-awesome-color-picker';
	import { colord } from "colord";

	export let data = {};

	let hex = "#217ae6"

	let rgb = {
		"r": 31,
		"g": 31,
		"b": 31,
		"a": 1
	}	

	let color = colord("#ff0000");

</script>

<a href="/">Home</a><a href="/">Home</a>

<div class="form">
	<form>
		<label for="name">Name:</label><br>
		<input type="text" id="name" name="name" required><br><br>
	
		<label for="emoji">Emoji:</label><br>
		<input type="text" id="emoji" name="emoji" pattern="[\u1F300-\u1F5FF\u1F600-\u1F64F\u1F680-\u1F6FF\u1F900-\u1F9FF\u2600-\u26FF\u2700-\u27BF]+" placeholder="😀" title="Please enter a single emoji."> <br><br> 

		

	
		<label for="date">Date:</label><br> 
		<input type="date" id="date" name="date" required><br><br>
	
		<label for="color">Favorite Color:</label><br>

		<div class="colourPicker">
			<ColorPicker 				
				bind:hex
				bind:rgb
				bind:color
				isDialog={false} 
				components={ChromeVariant} 
				sliderDirection="horizontal"/><br>
		</div>

		<div class = "colour" style="background-color: {hex};"></div>

		<button type="submit">Submit</button>
	  </form>
</div>
