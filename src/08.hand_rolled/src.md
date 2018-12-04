# DIY
$web-only$
With this one, we can start from the top. Our entry point, as always, is the `parse` function. The first step here is to check if the first character is a `P`, if it isn't we can stop this is not a `Duration`. Next we want to split the `&str` into parts, the first part would be everything after `P` and before `T`, the second would be everything from `T` to the end of the string.

Once we split it up, we can expect there to be, at most, 2 parts so a manual calls to `next` on the `Split` iterator should be enough. We can feel confident that the first one is going to be the date part because calling `split` on `T3H` would be `""` followed by `"3D"`, so first we test that `next` is `Some` then we double check it isn't `""`, if both are true we can pass the first half to `parse_parts` with the `false` as the second argument.

`parse_parts` takes in one half of the duration and a flag to indicate if `M` should be a month or a minute. It first finds all of the `char_indices` that have one of our unit characters. We are going to need to keep track of our position in this `&str` and this is done with the `start_idx` variable. We can now loop over the indices getting a slice of the input string from the `start_index` to the `idx` of the unit character which we want to parse as an `f32`. Next we want to match on the unit character which should be at the `idx`, using the `time` flag to determine if `M` means minute or month, we create a duration part. We are going to collect all of these parts in a `Vec<DurationPart>` to eventually return it so we push the duration into that `Vec` and finally update the`start_idx` to be the `idx` + 1. This should get us through all of the parsing, next we need to collect these `DurationPart`s into a `Duration`. We do that by passing a mutable reference to a `Duration` along with the each of the `DurationPart`s off to the `update_duration` function. This just matches on the `DurationPart` and updates the provided `Duration` accordingly. We do this for both of our expected iterator items and we are done. There is a check in here to make sure that there is at least 1 unit/value pair.

{{#playpen lib.rs}}

$web-only-end$
$slides-only$
# Are we not men?
# We are demo!
$slides-only-end$
