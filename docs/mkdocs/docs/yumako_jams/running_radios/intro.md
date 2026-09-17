# Generating radios

You can start generating radios using the `yumako_jams run` command:

```bash
alistral yumako_jams run <RADIO_NAME> [INPUTS]
```

The inputs parameter is a a JSON5 string of the radio parameters, with the top level braces optional:

`listen_range: "Last90Days", duration: "25 hours"`

To learn more about the inputs of a radio, you can use the `info` command:

```bash
alistral yumako_jams info <RADIO_NAME>
```