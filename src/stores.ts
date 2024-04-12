import { writable } from 'svelte/store';


interface countDown {
    name:string,
    emoji:string,
    date:number,
    colour:string
}

export const edit_data = writable<countDown>();


export const countdown_list = writable<countDown[]>([]);