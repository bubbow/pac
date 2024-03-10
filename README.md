# pac
Pac is a very simple video downloader built in rust.

Features:
	MP4 AVI MOV MP3 M4A WAV and OGG support....
	Downloading of videos off the web? i guess

This is a pretty basic application and you can probably find better alternatives

## Dependencies
windows:
Download YT-DLP and add it to your path
or install it with winget or something like that
`winget install yt-dlp`

Debian based distro (aka: ubuntu, mint, pop_os):
`sudo apt install yt-dlp`

Fedora based distro:
`sudo dnf install yt-dlp`

Arch based distro:
`sudo pacman -S yt-dlp`

For Linux you'll also need zenity or kdialouge. You probably already have it so don't worry.

Macos:
I dont have a mac so im guessing its:
`brew install yt-dlp`
just make sure you have brew installed, its easy search it up.
or you could install the executable and do it that way but i have no clue
## Install

Install the executeables in the releases tab.

OR

install rust and compile it with cargo 
`cargo build --release`

Make sure you have all the dependencies above.

 