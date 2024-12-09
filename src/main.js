const { open } = window.__TAURI__.shell;
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let greetInputEl;
let greetMsgEl;

async function greet() {
  try {
    greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  } catch (error) {
    console.error("Failed to invoke greet:", error);
  }
}

async function get_link() {
  try {
    const link = await invoke("greet", { name: greetInputEl.value });
    console.log("Received link:", link);
    if (link) {
      await open(link);
    } else {
      console.error("No link returned from get_link command");
    }
  } catch (error) {
    console.error("Failed to get or open link:", error);
  }
}

window.addEventListener('DOMContentLoaded', async () => {
  greetInputEl = document.querySelector('#greet-input');
  greetMsgEl = document.querySelector('#greet-msg');
  document.querySelector('#greet-form').addEventListener('submit', (e) => {
    e.preventDefault();
    greet();
  });

  document.querySelector('#oauth-button').addEventListener('click', (e) => {
    e.preventDefault();
    get_link();
  });
});
