# Hello, Rust

Tested ok on fedora linux

```
kushal@kusfedoravm:~$ cd ~/bin; cat update_helloworld.sh; ./update_helloworld.sh
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
Latest release: 202408082021
Download URL: https://github.com/kusl/hellorust20240711/releases/download/202408082021/helloworld
helloworld           100% [==============================================================================================================================================================>]    3.69M    7.68MB/s
                          [Files: 1  Bytes: 3.69M [3.66MB/s] Redirects: 1  Todo: 0  Errors: 0                                                                                             ]
Guess the number!
Remember, you can update your consent by running this application with the --update-consent flag.
Please input your guess.
12
You guessed: 12
Too small!
The sum of your guess and the secret number is: 107
The product of your guess and the secret number is: 1140
The greatest common divisor of your guess and the secret number is: 1
Please input your guess.
95
You guessed: 95
You win!
Game Statistics:
Attempts: [12, 95]
Secret Number: 95
Guesses: [12, 95]
All Games History:
Game 1: Attempts: [12, 95], Secret Number: 95, Guesses: [12, 95]
Press Enter to exit...

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
Latest release: 202408082021
Download URL: https://github.com/kusl/hellorust20240711/releases/download/202408082021/helloworld
helloworld           100% [==============================================================================================================================================================>]    3.69M   15.85MB/s
                          [Files: 1  Bytes: 3.69M [7.73MB/s] Redirects: 1  Todo: 0  Errors: 0                                                                                             ]
Guess the number!
Remember, you can update your consent by running this application with the --update-consent flag.
Please input your guess.
34
You guessed: 34
Too small!
The sum of your guess and the secret number is: 124
The product of your guess and the secret number is: 3060
The greatest common divisor of your guess and the secret number is: 2
Please input your guess.
90
You guessed: 90
You win!
Game Statistics:
Attempts: [34, 90]
Secret Number: 90
Guesses: [34, 90]
All Games History:
Game 1: Attempts: [12, 95], Secret Number: 95, Guesses: [12, 95]
Game 2: Attempts: [34, 90], Secret Number: 90, Guesses: [34, 90]
Press Enter to exit...

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
Latest release: 202408082021
Download URL: https://github.com/kusl/hellorust20240711/releases/download/202408082021/helloworld
helloworld           100% [==============================================================================================================================================================>]    3.69M   31.53MB/s
                          [Files: 1  Bytes: 3.69M [7.85MB/s] Redirects: 1  Todo: 0  Errors: 0                                                                                             ]
Guess the number!
Remember, you can update your consent by running this application with the --update-consent flag.
Please input your guess.
24
You guessed: 24
Too small!
The sum of your guess and the secret number is: 111
The product of your guess and the secret number is: 2088
The greatest common divisor of your guess and the secret number is: 3
Please input your guess.
89
You guessed: 89
Too big!
The sum of your guess and the secret number is: 176
The product of your guess and the secret number is: 7743
The greatest common divisor of your guess and the secret number is: 1
Please input your guess.
88
You guessed: 88
Too big!
The sum of your guess and the secret number is: 175
The product of your guess and the secret number is: 7656
The greatest common divisor of your guess and the secret number is: 1
Please input your guess.
87
You guessed: 87
You win!
Game Statistics:
Attempts: [24, 89, 88, 87]
Secret Number: 87
Guesses: [24, 89, 88, 87]
All Games History:
Game 1: Attempts: [12, 95], Secret Number: 95, Guesses: [12, 95]
Game 2: Attempts: [34, 90], Secret Number: 90, Guesses: [34, 90]
Game 3: Attempts: [24, 89, 88, 87], Secret Number: 87, Guesses: [24, 89, 88, 87]
Press Enter to exit...

kushal@kusfedoravm:~/bin$
```

![rust rover screenshot](hello-rust.png)
