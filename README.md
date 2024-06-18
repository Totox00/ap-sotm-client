A CLI client for Sentinels of the Multiverse for the Archipelago multiworld randomiser.
Credits to [archipelago_rs](https://github.com/ryanisaacg/archipelago_rs) by ryanisaacg for the protocol implementation <3

## Joining a MultiWorld Game
###### This is the same section as in the setup guide on the archipelago site

Once you have a compiled version of the client, simply launch it in a terminal and input the server address, server port, slot name, and password if applicable.
These values can also be provided using the -s, -p, -S, and -P flags respectively like `-p {port}`,
or set to the defaults by using those flags by not providing a value like `-P`
The defaults are:

| value          | flag | default        |
|----------------|------|----------------|
| server address | -s   | archipelago.gg |
| server port    | -p   | 38281          |
| slot name      | -S   | Player         | 
| password       | P    | (No password)  |

Once connected, you can move the cursor using arrow keys, send a location using Enter,
filter items and locations by typing, clear the filter using Ctrl+C, and disconnect using Ctrl+D.

Tab can also be used to toggle multi-sending, which makes it so sending an Ultimate location for
a villain also sends the easier difficulties, and Advanced and Challenge also send the Normal location.

Home can be used to return to the top left if you get lost.
