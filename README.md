# Hello, Rust 

Tested ok on fedora linux

```
kushal@kusfedoravm:~/bin$ cd ~/bin; cat update_helloworld.sh; ./update_helloworld.sh
#!/bin/bash

# Define the repository owner and name
OWNER="kusl"
REPO="hellorust20240711"

# Fetch the latest release tag including pre-releases using the GitHub API
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$OWNER/$REPO/releases" | grep -m 1 "tag_name" | cut -d '"' -f 4)
if [ -z "$LATEST_RELEASE" ]; then
    echo "Error: Could not fetch the latest release tag."
    exit 1
fi
echo "Latest release: $LATEST_RELEASE"

# Construct the download URL
DOWNLOAD_URL="https://github.com/$OWNER/$REPO/releases/download/$LATEST_RELEASE/helloworld"
echo "Download URL: $DOWNLOAD_URL"

# Download the latest release
cd ~/bin/
rm -f helloworld
wget $DOWNLOAD_URL -O helloworld
if [ $? -ne 0 ]; then
    echo "Error: Failed to download the file."
    exit 1
fi
chmod +x helloworld
./helloworld
Latest release: 202408081133
Download URL: https://github.com/kusl/hellorust20240711/releases/download/202408081133/helloworld
helloworld           100% [==============================================================================================================================================================>]    3.66M    7.29MB/s
                          [Files: 1  Bytes: 3.66M [3.05MB/s] Redirects: 1  Todo: 0  Errors: 0                                                                                             ]
Guess the number!
Remember, you can update your consent by running this application with the --update-consent flag.
Please input your guess.
12
You guessed: 12
Too small!
The sum of your guess and the secret number is: 108
The product of your guess and the secret number is: 1152
The greatest common divisor of your guess and the secret number is: 12
Please input your guess.
96
You guessed: 96
You win!
Press Enter to exit...
```