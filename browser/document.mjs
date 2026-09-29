import {getPreference,setPreference} from './preferences.mjs';
const button=document.getElementById('theme-toggle');
function apply(theme){document.documentElement.dataset.theme=theme==='dark'?'dark':'light';button.textContent=theme==='dark'?'Light':'Dark';button.setAttribute('aria-label',theme==='dark'?'Switch to light theme':'Switch to dark theme');}
button.onclick=()=>{const theme=document.documentElement.dataset.theme==='dark'?'light':'dark';apply(theme);setPreference('theme',theme);};
getPreference('theme').then(value=>{if(value)apply(value);});
