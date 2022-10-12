<div align="center">

<div>
    <img src="https://i.postimg.cc/cJ1zgBsM/logo.png" align="center">
</div>

<div>
    <img src="https://img.shields.io/github/commit-activity/m/Atsukoro1/cnvm?style=for-the-badge">
    <img src="https://img.shields.io/github/repo-size/Atsukoro1/cnvm?style=for-the-badge">
    <img src="https://img.shields.io/static/v1?label=OS&message=Linux%20|%20Windows&color=orange&style=for-the-badge">
</div>

Manage your **node.js versions**, easier and faster!

<br>
</div>

## 📝 Description

CNVM is the best command line tool for switching and installing node versions! It makes managing multiple versions of Node.js a breeze! Just execute one command and everything is done!

## ⚡️ Quick start

### 🔨 Installing on Linux

1. Download the prebuilt binary [here](https://github.com/Atsukoro1/cnvm/releases)

2. Add the node binaries directory to the path using the *.bashrc* file (if you use bash) or *.zshrc* if you use zsh. This folders should be located in your home directory

```bash
# If you haven't change anything this dir will be ~/.nodejs/bin
export PATH="/home/<yourname>/.nodejs/bin:$PATH"
```

3. Move the binary to the /bin directory

```bash
# Assuming the binary downloaded to "Download" directory
mv ~/Downloads/binary ~/bin/cnvm
```

### 🔨 Installing on Windows

1. Download the prebuilt executable [here](https://github.com/Atsukoro1/cnvm/releases)

2. Move the downloaded executable to any folder you want

3. Now add C:\users\<your_name>\.node.js (assuming you will not change the path during the node installation process) and 
the directory you moved the executable to, to the PATH. In case you don't know how to add folders to PATH on windows, use [this guide](https://stackoverflow.com/questions/44272416/how-to-add-a-folder-to-path-environment-variable-in-windows-10-with-screensho).

## ⚙️ Commands & Options

| Option | Description                                              | Type   | Default | Required? |
|--------|----------------------------------------------------------|--------|---------|-----------|
| `-v`   | Specify version to work with. | `String` | Nothing | Yes        |
| `-c`   | Specify path to cnvm folder (this folder does contain all installed versions of node). | `String` | `false` | No        |

### `install`

Install a new version of node.js that is not currently on your system.

```bash
cnvm install -v <version>
```

### `switch`

Switch to a different Node version, **the version must be installed** before switching to it.

```bash
cnvm switch -v <version>
```

### `uninstall`

Uninstall specific Node version from your system, a switch will be automatically used to switch to latest stable version.

```bash
cnvm uninstall -v <version>
```

## 🔨 Contributing

If you want to contribute, please open an issue first and dicuss the changes you want to make in detail. Thanks!

## ⚠️ License

`CNVM` is free and open-source software licensed under the [GNU General Public License 2.0](https://github.com/Atsukoro1/cnvm/blob/main/LICENSE).