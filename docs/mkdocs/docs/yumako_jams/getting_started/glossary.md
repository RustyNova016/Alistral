# Glossary

Making radios require some understanding of some concepts. It might be hard to derstand everything in this page at first, but keep this tab open and come back to it anytime you need to refresh on what a word need 

## Radio Item

A radio item represent a musicbrainz recording with some associated data. Currently, a radio item contains:
- The musicbrainz recording's data
- Listens of that recording
- The score of that item

Those are the futures tracks that will appear in your playlist

#### Score

The score of a radio item is a value that can be manipulated to sort / filter the items. It doesn't have a strict definition, as the score depends on the radio creator's vision. 

As an exemple, you can set the score to the number of listens of the radio item, then sort by the biggest score to have a radio of your top listened tracks, or even set it as the recording duration, etc...

# Module

A module allows to manipulate those radio items. The common types of modules are:
- Seeders: Provides radio items to the radio.
- Filters: Remove radio items based on a condition.
- Mappers: Turn a listen into other(s)
- Scorers: Add/Remove from the score of a radio item

## Layer

A layer is an instance of a module in a radio. If a module is the blueprint of an oven, a layer is an actual physical oven. 

## Stream

The stream represent the incoming flow of radio items. It's easier to see it as a conveyor belt that funnel radio items one by one into a series of modules. The output of a stream is the output of the radio