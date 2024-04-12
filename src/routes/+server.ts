
// import { invoke } from '@tauri-apps/api/tauri';
// import { countdown_list } from "../stores";

// interface countDown {
//     name:string,
//     emoji:string,
//     date:number,
//     colour:string
// }
// async function load_data():Promise<countDown[]>{
//     try { 
//         const JSONdata:string[] = await invoke('load_data');  
//         return JSONdata.map(str=> {
//             return JSON.parse(str);
//         })
    
//     } catch(error) {
//         console.error("Error loading data:", error);
//         return [];
//     }

// }

// const data = await load_data();
// countdown_list.update(() => data);