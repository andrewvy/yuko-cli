### yuko-cli

CLI tool to [yuko.app](https://yuko.app).

### Installation

- Clone this repo.
- `cargo install --path .`

Yuko expects a `~/.config/yuko/config.json` to be created, with the format of:

```json
{
  "api": {
    "token": "PERSONAL_API_TOKEN"
  }
}
```

You can get your API token from your accounts page at: [yuko.app/account](https://yuko.app/account).

### Usage

`yuko-cli post "Pubish that yuko-cli project. #todo #yuko"`

`yuko-cli list "#todo #yuko"`
