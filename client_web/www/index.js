import init, { new_datapackage_store, new_session } from "./pkg/client_web.js";

const form = document.getElementById("conn-info");
const server = document.getElementById("server");
const port = document.getElementById("port");
const slot = document.getElementById("slot");
const password = document.getElementById("password");
const msgBuffer = document.getElementById("msg-buffer");
const villains = document.getElementById("villains");
const environments = document.getElementById("environments");
const heroes = document.getElementById("heroes");
const villainLocations = document.getElementById("villain-locations");
const environmentLocations = document.getElementById("environment-locations");
const variantLocations = document.getElementById("variant-locations");
const disconnect = document.getElementById("disconnect");
const tooltip = document.getElementById("tooltip");
const descToggle = document.getElementById("desc-toggle");

server.value = localStorage.getItem("server") ?? "archipelago.gg";
port.value = localStorage.getItem("port") ?? "38281";
slot.value = localStorage.getItem("slot") ?? "Player";
password.value = localStorage.getItem("password") ?? "";

form.addEventListener("submit", (e) => {
  e.preventDefault();

  localStorage.setItem("server", server.value);
  localStorage.setItem("port", port.value);
  localStorage.setItem("slot", slot.value);
  localStorage.setItem("password", password.value);

  tryConnect(e);
});

disconnect.addEventListener("click", () => {
  msgBuffer.innerText = "";
  villains.innerText = "";
  environments.innerText = "";
  heroes.innerText = "";
  villainLocations.innerText = "";
  environmentLocations.innerText = "";
  variantLocations.innerText = "";
  if (client) client.close();
  if (session) session.exit();
  client = null;
  state = null;
  roomInfo = null;
  datapackageStore = null;
  session = null;
});

descToggle.addEventListener("click", () => {
  showDesc = !showDesc;
  if (showDesc) {
    descToggle.innerText = "Hide Descriptions"
  } else {
    descToggle.innerText = "Show Descriptions"
  }
})

let tryConnect = (e) => console.log("Please wait for wasm to initialise.");
let printJson = (data) => console.log(data);

/**
 * @type WebSocket
 */
let client;
let state;
let roomInfo;
let datapackageStore;
let session;
let receivedItemIndex = 0;
let showDesc = false;

async function run() {
  await init();

  tryConnect = async () => {
    try {
      await initialConnect(true);
    } catch {
      await initialConnect(false);
    }

    client.onmessage = handleEvent;
  };
}

run();

function initialConnect(secure) {
  return new Promise((resolve, reject) => {
    client = new WebSocket(
      `${secure ? "wss" : "ws"}://${server.value}:${port.value}`
    );

    client.onopen = () => {
      if (client) {
        resolve();
      } else {
        reject(["Socket was closed unexpectedly."]);
      }
    };

    client.onerror = (event) => {
      reject([event]);
    };
  });
}

function roomInfoConnect() {
  datapackageStore = new_datapackage_store(JSON.stringify(roomInfo));
  const missing = datapackageStore.get_missing_games();
  if (missing.length > 0) {
    client.send(JSON.stringify([{ cmd: "GetDataPackage", games: missing }]));
  } else {
    datapackageConnect();
  }
}

function datapackageConnect(datapackage) {
  if (datapackage) {
    for (const [game, data] of Object.entries(datapackage.data.games)) {
      datapackageStore.add_game(game, JSON.stringify(data));
    }
  }
  client.send(
    JSON.stringify([
      {
        cmd: "Connect",
        password: password.value,
        game: "Sentinels of the Multiverse",
        name: slot.value,
        uuid: "",
        version: { major: 0, minor: 4, build: 6, class: "Version" },
        items_handling: 7,
        tags: [],
        slot_data: true,
      },
    ])
  );
}

function connectedConnect(connected) {
  session = new_session(
    datapackageStore,
    JSON.stringify(roomInfo),
    JSON.stringify(connected),
    slot.value
  );
  printJson = (data) => {
    const newMsg = document.createElement("li");
    newMsg.innerHTML = session.try_format_json(JSON.stringify(data));
    msgBuffer.appendChild(newMsg);
  };
  client.send(JSON.stringify([{ cmd: "Sync" }]));
}

function handleEvent(event) {
  const data = JSON.parse(event.data);

  for (const msg of data) {
    switch (msg.cmd) {
      case "RoomInfo":
        roomInfo = msg;
        roomInfoConnect();
        break;
      case "DataPackage":
        datapackageConnect(msg);
        break;
      case "Connected":
        connectedConnect(msg);
        break;
      case "ConnectionRefused":
        console.log(msg);
        break;
      case "PrintJSON":
        printJson(msg);
        break;
      case "ReceivedItems":
        const skip = receivedItemIndex - msg.index;
        session.recieved_items(
          msg.items.slice(skip).map((item) => BigInt(item.item))
        );
        receivedItemIndex += msg.items.length - skip;
        state = session.get_state();
        updateState();
        break;
    }
  }
}

function updateState() {
  villains.innerText = "";
  environments.innerText = "";
  heroes.innerText = "";
  villainLocations.innerText = "";
  environmentLocations.innerText = "";
  variantLocations.innerText = "";

  for (const item of state.villains()) {
    const newElem = document.createElement("li");
    newElem.innerText = item.name();
    newElem.addEventListener("mouseenter", itemTooltip(item));
    villains.appendChild(newElem);
  }

  for (const item of state.team_villains()) {
    const newElem = document.createElement("li");
    newElem.innerText = item.name();
    newElem.addEventListener("mouseenter", itemTooltip(item));
    villains.appendChild(newElem);
  }

  for (const item of state.environments()) {
    const newElem = document.createElement("li");
    newElem.innerText = item.name();
    newElem.addEventListener("mouseenter", itemTooltip(item));
    environments.appendChild(newElem);
  }

  for (const item of state.heroes()) {
    const newElem = document.createElement("li");
    newElem.innerHTML = item.name();
    newElem.addEventListener("mouseenter", (e) => {
      tooltip.innerHTML = session.get_filler_for_hero(item, showDesc);
      moveTooltip(e);
    });    
    heroes.appendChild(newElem);
  }

  const available = state.available();

  for (const location of available.villains()) {
    const newElem = document.createElement("li");
    newElem.innerHTML = location.name();
    newElem.addEventListener("click", () => {
      sendLocation(location);
      newElem.remove();
    });
    newElem.addEventListener("mouseenter", locationTooltip(location));
    villainLocations.appendChild(newElem);
  }

  for (const location of available.team_villains()) {
    const newElem = document.createElement("li");
    newElem.innerHTML = location.name();
    newElem.addEventListener("click", () => {
      sendLocation(location);
      newElem.remove();
    });
    newElem.addEventListener("mouseenter", locationTooltip(location));
    villainLocations.appendChild(newElem);
  }

  if (available.victory) {
    const newElem = document.createElement("li");
    newElem.innerHTML = "Oblivaeon (VICTORY)";
    newElem.addEventListener("click", () => {
      session.victory();
      client.send(JSON.stringify([{ cmd: "StatusUpdate", status: 30 }]));
      newElem.remove();
    });
    villainLocations.appendChild(newElem);
  }

  for (const location of available.environments()) {
    const newElem = document.createElement("li");
    newElem.innerHTML = location.name();
    newElem.addEventListener("click", () => {
      sendLocation(location);
      newElem.remove();
    });
    newElem.addEventListener("mouseenter", locationTooltip(location));
    environmentLocations.appendChild(newElem);
  }

  for (const location of available.variants()) {
    const newElem = document.createElement("li");
    newElem.innerHTML = location.name();
    newElem.addEventListener("click", () => {
      sendLocation(location);
      newElem.remove();
    });
    newElem.addEventListener("mouseenter", locationTooltip(location));
    variantLocations.appendChild(newElem);
  }
}

function sendLocation(location) {
  const ids = session.get_location_ids([location]);
  client.send(
    JSON.stringify([
      { cmd: "LocationChecks", locations: Array.from(ids).map(Number) },
    ])
  );
}

function itemTooltip(item) {
  return (e) => {
    tooltip.innerHTML = session.get_filler_for_item(item, showDesc);
    moveTooltip(e);
  };
}

function locationTooltip(location) {
  return (e) => {
    tooltip.innerHTML = session.get_filler_for_location(location, showDesc);
    moveTooltip(e);
  };
}

function moveTooltip(e) {
  if (tooltip.childNodes.length > 0) {
    tooltip.hidden = false;
  } else {
    tooltip.hidden = true;
  }

  tooltip.style.left = `${e.pageX + 25}px`;
  tooltip.style.top = `${e.pageY}px`;
}
