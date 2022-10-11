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

CNVM is simple command line tool to switch and install your node versions. Managing multiple versions of Node.js can be annoying, everything is easier with CNVM just execute one command and everything is done!

## ⚡️ Quick start

- This section will be filled later right before the first release.

## ⚙️ Commands & Options

| Option | Description                                              | Type   | Default | Required? |
|--------|----------------------------------------------------------|--------|---------|-----------|
| `-v`   | Specify version to work with. | `String` | `false` | Yes        |
| `-c`   | Specify path to cnvm folder (this folder does contain all installed versions of node). | `String` | `false` | No        |

### `install`

Install a new version of node.js that is not currently on your system.

```bash
cnvm install -v <version>
```

### `switch`

Switch to a different Node version, **the version must be installed** before switching to it.

### `uninstall`

Uninstall specific Node version from your system, a switch will be automatically used to switch to latest stable version.

```bash
cnvm uninstall -v <version>
```

## 🔨 Contributing

If you want to contribute, please open an issue first and dicuss the changes you want to make in detail. Thanks!

## ⚠️ License

`CNVM` is free and open-source software licensed under the [GNU General Public License 2.0](https://github.com/Atsukoro1/cnvm/blob/main/LICENSE).