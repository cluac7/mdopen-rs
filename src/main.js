const { invoke } = window.__TAURI__.tauri;
const { getMatches } = window.__TAURI__.cli;

let mdcontent = document.getElementById("mdcontent");

getMatches().then((matches) => {
  let sourcepath = matches.args.sourcepath.value;
  invoke("open_file", { path: sourcepath }).then(
    (contents) => (mdcontent.innerHTML = contents),
  );
});
