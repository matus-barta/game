# Chunk

## data

locate by `id`

```json
{
    id: int,
    terrain_id: string, //sha265 of the asset
    world_objects: optional WorldObject[], //array of world objects
    placables: optional Placable[] //array of placables
}
```

## map size

Chunk id should be determined by position in the world. But this limits the world size, at least limits have to be determined.

Lets make size of the chunk `1km^2 = 1000m x 1000m`, and limit the us to ~~256~~ `32chunks x 32chunks = 1024 chunks` , this is 1024km^2 (265 might be ok if we are going for WoW size).

| GAME  | size in km^2 |
| ----- | ------------ |
| ETS 2 | 11680        |
| TDU 2 | 1600         |
| WoW   | 207          |
| SoT   | 85           |
| GTA V | 81           |
| RDR2  | 75           |

## Chunk id

- linear array of 1024 elements
- 32 by 32

So,

```text
[0][1][2][3][4][5]...[31]
.
.
.
.
.
[991]..............[1023]
```

with some _math_ we can find our position and also neighboring chunks.
