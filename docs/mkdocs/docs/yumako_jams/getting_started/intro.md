

# Radio Item

A radio item represent a musicbrainz recording with some associated data. Currently, a radio item contains:
- The musicbrainz recording's data
- Listens of that recording
- The score of that item

## Score

The score of a radio item is a value that can be manipulated to sort / filter the items. It doesn't have a strict definition, as the score depends on the radio creator's vision. 

As an exemple,you can set the score to the number of listens of the radio item, then sort by the biggest score to have a radio of your top listened tracks, or even set it as the recording duration, etc...

# Module

A module allows to interact with one or multple radio items. Whether it's to create items, modify their data, or remove them.

# Stream

The stream represent the incoming flow of radio items. It's easier to see it as a conveyor belt that funnel radio items one by one into a series of modules. The output of a stream is the output of the radio

# Inputs

## Module inputs

Module inputs allow 