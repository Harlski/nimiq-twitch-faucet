# nimiq-twitch-faucet

This is a Work in Progress for public visiblity into the rework of the Twitch Chat/Faucet Bot for NimiqLIVE.

Usage:
Clone into repository & cd to directory.
cargo run -- --nick yourusername --token youroauthcode --channel #yourchannel

Use https://twitchapps.com/tmi/ to get your auth code - or follow dev docs: https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/

The current reworked implementations.
- Connect to Twitch channel chat via TMI
- Read and send messages to channel
- Keep track of eligible users & if they are subscriber/not subscribed
- Clear list of eligible users
- Select a winner from list of eligible users
- Generate a random code and refresh the random code.
- Detect when a user types that code & perform some actions


Still need to do:
- Hook the Entry Code somewhere to make it visible on screen. 
- User nomination of their Nimiq Wallet address. (How to store it too)
- Create a connection to the Nimiq blockchain
- Send a transaction to a users .wallet_address()
