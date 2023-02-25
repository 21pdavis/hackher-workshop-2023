## Hi, everyone!

This doc outlines some basic setup steps to allow you to run the “cargo build” command in the cloned git repository. This doc will guide you through installing a C compiler and Rust on your respective OS. Some of this might already be done on your computer, so make sure to try and clone the repo first to see what you’re missing! Please Slack message me if you have any difficulties or questions!

**If you just want to follow along with the 4:30 presentation instead of doing this setup, that’s totally fine, too!**

### If you don’t have Git installed:
https://git-scm.com/downloads

###Try and clone this git repo and run the following commands (paste the below commands into your terminal):
1. `git clone https://github.com/21pdavis/hackher-workshop-2023.git`
2. `cd hackher-workshop-2023`
3. `cargo build`

**If the 'cargo build' command did not work for you, here's everything you'll need if you want to follow along (but don't worry if it doesn't work - you can come back to it later and just watch me!):**

### Install Rust (All Operating Systems):
Head over to the Rust website and follow the install instructions:
https://www.rust-lang.org/tools/install
Install the Rustup exe (Windows) or paste and run the 'curl' command:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
(MacOS or Linux)

### Windows Users:

You'll want to install visual studio community and install a version of "Visual Studio Build Tools" to get the MSVC compiler (you might already have this): [https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017)

### Mac Users:
You'll want to run the following commands:
1. `/bin/bash -c "$(curl -fsSL [https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh](https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh))"`
Follow the installation instructions above, then open a new terminal and run:
2. `brew install gcc`
3. `brew install sdl2`

### Linux/WSL Users:
You'll want to run the following commands:
1. `sudo apt update`
2. `sudo apt install build-essential`
3. `sudo apt-get install libsdl2-dev` (and say 'Y' to all prompts)
