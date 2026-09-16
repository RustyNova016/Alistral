# Your first radio

## The radio file's skeleton

The base of a radio looks like this

```json
{
    "name": "My Radio",
    "yumako_version": "0.1.0",
    "stack": [],
    "inputs": {}
}
```

`name` is the name of the radio

`yumako_version` is the version of yumako_jam this is built for. This allows warning about outdated radios.

`stack` is the list of modules that are in the radio

`inputs` handles variables. You can safely ignore it for now.


## Writting the radio

The first thing a radio need, is radio items. What's the point of a radio if it's empty? 

To get radio items, we first need a seeder module. It "seeds" the radio by making radio items. A great starting module is the `listen_seeder` module. It looks at an user's listens, and turn all their listened recordings into radio items.

```json
{
    "name": "My Radio",
    "stack": [        
        {
            "id": "listen_seeder",
            "step_type": "listen_seeder",
            "inputs": {
                "user": "RustyNova"
            }
        }
    ],
    "inputs": {}
}
```

Let's go over the layer's JSON.

`id`: Each layer must have their own unique name. Choose one that makes sense and describe what you are trying to accomplish.

`step_type`: The type of module to use. Here we want a `listen_seeder`

`inputs`: Most modules require a bunch of info to work. Those are **inputs**. Here, our listen seeder want to know what user to pull the listen's from. You can put your LB username instead

And that's it! That's a working radio. This will give you a radio of all the recording you ever listened to... Not the most useful, but it works.