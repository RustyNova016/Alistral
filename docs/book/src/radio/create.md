# Create a radio playlist

To create a radio playlist, you can use `alistral radio create`. This takes in the arguments as follows:

## Radio name

Which radio to use

## (Optional) -o, -output

The music provider to send the playlist to.

Possible values: `listenbrainz` (default), `youtube`

## (Optional) -u, -username

Your username to use for the radio. By default, uses the one stored in the config file

## (Optional) -t, -token

Your ListenBrainz token. By default, uses the one stored in the config file

## Radio Arguments

Those are the parameters for the radio. Those can be found by using `alistral radio info <RADIO_NAME>`

You can set the value of a variable by adding an argument like so: `variable_name = "Some Value"`. Multiple variables are separated by a `;`, which can be escaped like so: `\;`

Values are using their JSON representation. So, for example:
- Integer: `listens = 3`
- String: `name = "John Factorio"`
- List: `genres = ["Industrial", "Electro"]`
- Objects: `fancy_stuff = {"fancy_value": "Hello"}`

Due to limitations with shells automatically removing quotes in commands, it's recommened to wrap the arguments in a `'`. For example:

`alistral radio create overdue_count 'listen_range="Last90Days"; minimum_listens = 5'`

### Special variables

Those variables are automatically provided and don't need to be entered:
- timeouts
- bumps
- username (uses the --username flag's value)

Those variables aren't part of the radio generation, and won't show up in the radio's info, but are still used:
- `count: integer` (Default: `50`): The **minimum** number of tracks the playlist should have
- `duration: Duration String` (Default: `"0 seconds"`): The **minimum** duration the playlist should have