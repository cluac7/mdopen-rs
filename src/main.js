const { invoke } = window.__TAURI__.tauri;
const { getMatches } = window.__TAURI__.cli;

let mdcontent = document.getElementById("mdcontent");

getMatches().then((matches) => {
  let sourcepath = matches.args.sourcepath.value;
  console.log(sourcepath);
  mdcontent.textContent = sourcepath;
});

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

// window.addEventListener("DOMContentLoaded", () => {
//   greetInputEl = document.querySelector("#greet-input");
//   greetMsgEl = document.querySelector("#greet-msg");
//   document.querySelector("#greet-form").addEventListener("submit", (e) => {
//     e.preventDefault();
//     greet();
//   });
// });
