# Generating radios

You can start generating radios using the `yumako_jams run` command:

```bash
alistral yumako_jams run <RADIO_NAME> [INPUTS]
```

# Arguments

## Radio Name

The name of the radio file to use

## inputs

The inputs of the radio. Those are tweakable settings to generate the radio. The format is a [json5](https://json5.org/) string (JSON with less restrictions). The top level braces are optional:

`listen_range: "Last90Days", duration: "25 hours"`

To know the possible inputs of a readio, you can use the `info` command:

```bash
alistral yumako_jams info <RADIO_NAME>
```

### Special inputs

Those variables are automatically provided and don't need to be entered:

- timeouts
  
- bumps
  
- username (uses the --username flag's value)

## --target

The interzic service to send the playlist to. 

## --instance

The instance of the interzic service 

## --username (Optional)

Your listenbrainz username

## --token (Optional)

The token of your listenbrainz account

