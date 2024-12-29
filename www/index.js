import init, { new_datapackage_store, new_session } from "./pkg/client.js";

const form = document.getElementById("conn-info");
const server = document.getElementById("server");
const port = document.getElementById("port");
const slot = document.getElementById("slot");
const password = document.getElementById("password");
const msgBuffer = document.getElementById("msg-buffer");
const clientElem = document.getElementById("client");
const clear = document.getElementsByClassName("clear");

server.value = localStorage.getItem("server") ?? "archipelago.gg";
port.value = localStorage.getItem("port") ?? "38281";
slot.value = localStorage.getItem("slot") ?? "Player";

form.addEventListener("submit", (e) => {
  e.preventDefault();

  localStorage.setItem("server", server.value);
  localStorage.setItem("port", port.value);
  localStorage.setItem("slot", slot.value);

  tryConnect(e);
});

function disconnect(e) {
  clientElem.hidden = true;
  closing = true;
  e.stopPropagation();
  for (const elem of clear) elem.innerText = "";
  client.send(
    JSON.stringify([{ cmd: "Get", keys: [`sotm-save-${slot.value}`] }])
  );
}

document.getElementById("disconnect").addEventListener("click", disconnect);

clientElem.addEventListener("click", (e) => {
  if (e.target.id) {
    const action = session.handle_click(e.target.id);

    client.send(
      JSON.stringify([
        {
          cmd: "LocationChecks",
          locations: Array.from(action.locations()).map(Number),
        },
      ])
    );

    if (action.victory) {
      client.send(JSON.stringify([{ cmd: "StatusUpdate", status: 30 }]));
    }
  }
});

let tryConnect = (e) => console.log("Please wait for wasm to initialise.");
let printJson = (data) => console.log(data);

let client;
let roomInfo;
let datapackageStore;
let session;
let receivedItemIndex = 0;
let closing = false;

async function run() {
  await init();

  tryConnect = async () => {
    form.hidden = true;
    try {
      await initialConnect(true);
    } catch {
      try {
        await initialConnect(false);
      } catch {
        form.hidden = false;
      }
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

async function roomInfoConnect() {
  datapackageStore = new_datapackage_store(JSON.stringify(roomInfo));
  await datapackageStore.get_fs();
  await datapackageStore.load_cached_datapackages();
  const missing = datapackageStore.get_missing_games();
  if (missing.length > 0) {
    console.log(`Missing datapackages for games ${missing}`);
    client.send(JSON.stringify([{ cmd: "GetDataPackage", games: missing }]));
  } else {
    datapackageConnect();
  }
}

async function datapackageConnect(datapackage) {
  if (datapackage) {
    for (const [game, data] of Object.entries(datapackage.data.games)) {
      await datapackageStore.add_game(game, JSON.stringify(data));
    }
  }
  if (!localStorage.getItem("apSotmUuid"))
    localStorage.setItem("apSotmUuid", Math.random() * (1 << 16));
  client.send(
    JSON.stringify([
      {
        cmd: "Connect",
        password: password.value,
        game: "Sentinels of the Multiverse",
        name: slot.value,
        uuid: localStorage.getItem("apSotmUuid"),
        version: { major: 0, minor: 3, build: 0, class: "Version" },
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
    msgBuffer.prepend(newMsg);
  };
  client.send(
    JSON.stringify([
      { cmd: "Sync" },
      { cmd: "Get", keys: [`sotm-save-${slot.value}`] },
    ])
  );
  const deathlinkType = session.deathlink();
  if (deathlinkType > 0) {
    client.send(
      JSON.stringify([{ cmd: "ConnectUpdate", tags: ["DeathLink"] }])
    );
    deathlink.hidden = false;
    deathlink.innerText = [
      "Deathlink Inactive",
      "Send Deathlink (Individual)",
      "Send Deathlink (Team)",
    ][deathlinkType];
  }
  clientElem.hidden = false;
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
        form.hidden = false;
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
        break;
      case "Bounced":
        // Handle deathlink
        break;
      case "Retrieved":
        const save_str = msg.keys[`sotm-save-${slot.value}`];
        if (save_str) session.update_save(save_str);
        if (closing) {
          client.send(
            JSON.stringify([
              {
                cmd: "Set",
                key: `sotm-save-${slot.value}`,
                default: "",
                want_reply: true,
                operations: [
                  { operation: "replace", value: session.save_string() },
                ],
              },
            ])
          );
        }
        break;
      case "SetReply":
        if (closing) {
          if (client) client.close();
          if (session) session.exit();
          client = null;
          roomInfo = null;
          datapackageStore = null;
          session = null;
          receivedItemIndex = 0;
          closing = false;
          form.hidden = false;
        }
    }
  }
}
