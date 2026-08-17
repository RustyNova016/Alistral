# Alistral for noobs

This tutorial should give you everything you need to start using Alistral, even if you didn't touch the terminal once in your life!

## The terminal

Alistral is a command line application, this means you need a terminal to run it. Just open your global search (windows/super key) and type "terminal" and run it.

You should have a black and scary window in front of you, but don't fear. The terminal is your friend! To run alistral, you need to tell the terminal where it is. Either CTRL+C the alistral downloaded file and paste in your terminal, or drag and drop the file in. You should now see the path to the file written on it. Further on, we assume that this part is always present

Next, we need to say what we want to do. For exemple, the daily report can be accessed by tapping `daily`. 

However, Alistral need more info about where to get those infos, and what to display. 
Add the date in [YYYY-MM-DD format](https://xkcd.com/634/), ex: `2026-08-14`, then your listenbrainz username, ex: `RustyNova`, then enter.

That's it! You have alistral running! It will take a while to get all the data, but you can stop it and come back without loosing progress.

## Make it less painful

While you can add your username to commands that needs it, it's easier to save it. Replace the dailty command (`daily 2026-08-14 RustyNova`) with `config default-user`, and add your username. Press enter, and now it is saved as the default username!

You can rerun `daily 2026-08-14` without the name, and it should work. And even better, `daily` gives you today's report!

## Stop it. Get some --help

Now run `daily --help`. This will show you how to create the command.

`Usage: alistral daily [OPTIONS] [DATE] [USERNAME]`

Ignore `[OPTIONS]` for a sec. You can see that we have our two inputs here! Date and Username. You can see them explained below as `Arguments`. Arguments in `[]` are optional. You can ignore them if not needed. Argument in `<>` must be set.

The options are described below. They used to accept inputs, but the order doesnt matter! You just need to write the name of the option, and then what's asked: `--username RustyNova`. Sometimes there are single letter options. Those works the same, but they are faster to type: `-u RustyNova`.

## The manual

You can find a list of all the commands and their descriptions [here](./CommandLineHelp.md). Those are all the `--help` texts in one document, easy to read.

As fun exercises, try looking up a recording, or show your top artists by listen duration

Answers: 
- `lookup recording https://listenbrainz.org/track/b8429ebe-6aa4-4b62-be3f-7c145fac2be8`
- `stats tops artist --sort-by listen-duration`

# I am so lost...

Still struggling? Feel free to send a message in the [forum thread](https://community.metabrainz.org/t/alistral-power-tools-for-listenbrainz/726412), a github discussion, or ask in the listenbrainz channel of the Metabrainz IRC/Matrix/Discord (DMs are fine, but I ignore short messages as it's 99% scammers. Treat your DM as it was an email!)

Your feedback will help improve this document!