# Paths
Here are all the paths used by alistral. In those paths, `Alice` is your system username. Most of the paths are based on system standards, although Windows and MacOS are lacking. If a standard path is wrong / forget to read a variable, please submit an issue

## Alistral Cache
Temporary files for alistral. You can safely delete this directory, although it may result in having to fetch data for hours. You can overwrite it by setting the `ALISTRAL_CACHE_PATH` environement variable

If compiled with `debug_assertions` (Aka `cargo build` and not `cargo build --release`), an extra `debug` folder is added at the end of the path. This is to prevent development database migrations to impact the released builds

### Windows
`C:\Users\Alice\AppData\Roaming\Local`

### MacOS
`/Users/Alice/Library/Caches/alistral`

### Linux
When running as a systemd service, it will take the first path in the `CacheDirectory` variable, if provided. Note that it doesn't add the `alistral` directory. `ALISTRAL_CACHE_PATH` still overwrite it

If not, it will take `$XDG_CACHE_HOME /alistral` or `$HOME/.cache/alistral` (`/home/alice/.cache/alistral`)

## Alistral Configuration
The configuration file of alistral. You can overwrite it by setting the `ALISTRAL_CONFIG_PATH` environement variable

### Windows
`C:\Users\Alice\AppData\Roaming\alistral`

### MacOS
`/Users/Alice/Library/Application Support/alistral`

### Linux
When running as a systemd service, it will take the first path in the `ConfigurationDirectory` variable, if provided. Note that it doesn't add the `alistral` directory. `ALISTRAL_CONFIG_PATH` still overwrite it

If not, it will take `$XDG_CONFIG_HOME/alistral` or `$HOME/.config/alistral` (`/home/alice/.config/alistral`)

## Alistral Data
The long term data of alistral. You can overwrite it by setting the `ALISTRAL_DATA_PATH` environement variable

### Windows
`C:\Users\Alice\AppData\Roaming\alistral`

### MacOS
`/Users/Alice/Library/Application Support/alistral`

### Linux
When running as a systemd service, it will take the first path in the `StateDirectory` variable, if provided. Note that it doesn't add the `alistral` directory. `ALISTRAL_DATA_PATH` still overwrite it

If not, it will take `$XDG_DATA_HOME/alistral` or `$HOME/.local/share/alistral` (`/home/alice/.local/share/alistral`)

## Alistral Logs
The logs of alistral. You can overwrite it by setting the `ALISTRAL_LOGS_PATH` environement variable

### Windows
`C:\Users\Alice\AppData\Roaming\alistral\logs`

### MacOS
`/Users/Alice/Library/Application Support/alistral/logs`

### Linux
When running as a systemd service, it will take the first path in the `LogsDirectory` variable, if provided. Note that it doesn't add the `alistral` directory. `ALISTRAL_LOGS_PATH` still overwrite it

If not, it will take `$XDG_DATA_HOME/alistral/logs` or `$HOME/.local/share/alistral/logs` (`/home/alice/.config/alistral/logs`). 

## Interzic Data
The long term data of interzic. You can overwrite it by setting the `INTERZIC_DATA_PATH` environement variable

### Windows
`C:\Users\Alice\AppData\Roaming\interzic`

### MacOS
`/Users/Alice/Library/Application Support/interzic`

### Linux
When running as a systemd service, it will take the first path in the `StateDirectory` variable, if provided. Note that it doesn't add the `interzic` directory. `INTERZIC_DATA_PATH` still overwrite it. It's preferable that you use `INTERZIC_DATA_PATH` as the alistral and interzic data directory would be shared. Add the `INTERZIC_DATA_PATH` environement variable, and also add the path as a second argument for `StateDirectory`

If not, it will take `$XDG_DATA_HOME/interzic` or `$HOME/.local/share/interzic` (`/home/alice/.local/share/interzic`)

## Yumako jams Configuration
The configuration for yumako jams. You can overwrite it by setting the `YUMAKO_JAMS_CONFIG_PATH` environement variable

### Windows
`C:\Users\Alice\AppData\Roaming\yumako_jams`

### MacOS
`/Users/Alice/Library/Application Support/yumako_jams`

### Linux
When running as a systemd service, it will take the first path in the `ConfigurationDirectory` variable, if provided. Note that it doesn't add the `yumako_jams` directory. `YUMAKO_JAMS_CONFIG_PATH` still overwrite it. It's preferable that you use `YUMAKO_JAMS_CONFIG_PATH` as the alistral and yumako jams config directory would be shared. Add the `YUMAKO_JAMS_CONFIG_PATH` environement variable, and also add the path as a second argument for `ConfigurationDirectory`

If not, it will take `$XDG_CONFIG_HOME/yumako_jams` or `$HOME/.config/yumako_jams` (`/home/alice/.config/yumako_jams`)