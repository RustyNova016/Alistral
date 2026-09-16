# Your second radio

Let's continue by making a radio for your top listened recordings.

Exercise: In the module documentation, try finding what modules you may want. Hint:
- You need to get the radio items with the most listens.

Our first module will be `listen_count_scorer`. What it does is set the score of the item to the count of all time listens of your recording

The second module is the `sort_module`. It sorts the radio listens depending on their scores. 

Let's add them both to the stack:

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
        },

        {
            "step_type": "listen_count_scorer",
            "id": "listen_count_scorer",
        },

        {
            "step_type": "sort_module",
            "id": "sort_module",
            "inputs": {
                "direction": "Desc"
            }
        }
    ],
    "inputs": {}
}
```

A few notes:
- `listen_count_scorer` has no inputs. There's no need for them
- `sort_module` sorts in a descending order of scores by default. So it doesn't need inputs either. This is just for ease of reading and that...
- `sort_module` can also sort in an ascending order if you prefer