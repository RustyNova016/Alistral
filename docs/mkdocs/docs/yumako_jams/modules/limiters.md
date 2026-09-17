# Limiters

Limiters are a type of filter that stops radio items after a certain condition is met. This allows making cutoffs on radios to not make it last a whole month

## Length limiter

Stop the radio after it reaches a certain ammount of radio items, or a duration

### Inputs

- `mode: "All" | "Any"` (Default: `"All"`): Choose whether one condition must be met (`"Any"`) or all (`"All"`)
- `duration: String` (Default: `"2 hours"`): The maximum duration written using the [human time format](https://docs.rs/humantime/latest/humantime/fn.parse_duration.html)
- `count: Int` (Default: `50`): The maximum amount of radio items to let through


### Example

```json
{
    "step_type": "length_limiter",
    "id": "length_limiter",
    "inputs": {
        "mode": "Any",
        "duration": "10 hours"
    }
}
```