# spotify_party_api
Backend API to control the playback of songs through the Spotify Web API with Rust

## Quick Summary
Have you ever been in a party where the host is the one with the music and everyone keeps asking the host to play a song, add something to the queue, to play it louder etc. (Or you might have been the host, and you know this is pretty annoying).
I have coded a solution for that, this backend server solves that specific problem. With this tool you can have a single host (owner of a party) that is playing the music and other people join the so called Spotify Party Online and the owner can give them permission to add songs to the queue, modify the volume, play something among many other things. 

You just need to create an account and create a new party and then your friends can ask you to join the party and you can give them permissions to do things with the playlist. 


## Actions
At the moment the memebers of the party can do the following actions:
(Action (API endpoint))
* Pause/Play the current song (/pausePlayback , /startPlayback)
* Play the next song (/playNext)
* Play the previous song (/playPrevious)
* Activate Shuffle mode (/shuffleOn)
* Deactivate Shuffle mode (/shuffleOff)
* Modify the volume (/modifyVolume?volume_percent=<>)