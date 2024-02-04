Nimiq Twitch Faucet is a rework into rust from the existing https://twitch.tv/nimiqlive auto-faucet.

The current implementation, which is closed source uses a combination of Python + NodeJS.

The core logic is, a user enters a code seen on screen - users gets added to an array, every x minutes a random user is picked from that array
That random user receives Y NIM automatically to a wallet address owned by themselves.

The current flow is to run a script.py & index.js file alongside each other.
Script.py handles:
- Reading input of a specifed Twitch chat via IRC
- Performing actions based off the input provided by individual users or specific users
- Loads a thread reference to a tipbot.py which handles user management (eligible users), current state of tip cycle
