# Block Assembly Syntax Docs
## Sprites
Sprites are declared with the sprite name surrounded by brackets. For example, to declare a sprite named `Cat`, you would use `[Cat]`.

They also have many properties.
|Name|What it does|How to use in `basm`|
|-|-|-|
|isStage|Determines whether the sprite is the stage|Use `.isStage` keyword, then use a boolean literal (`true`/`false`)|
|name|The name of the sprite|Use `.name`, then the name as a string literal|
|variables|Declares the sprite specific variables|For regular monitors, use `.var`, whether the monitor is visible as a `bool`, the UID as a `string`, the name as a `string`, the type (either, `string`, `double`, `int`, or `bool`, cannot be anything else), and the variable value.<br><br>For example, `.var true "V001" "Score" double 0` would create a variable called `Score` with the UID `V001`. It would be of `double` type and have the value `0`. The monitor would be visible.<br><br>By default, it is a normal monitor. To change to the big monitor type, use `.bigmonitor` then the variable UID. To change to the slider type, use `.slidermonitor`, then the variable UID, the minimum value, and the maximum value.<br><br>To change to global scope instead of sprite scope, use `.varglobalscope` then the variable UID. To change to cloud scope instead of sprite scope, use `.varcloudscope` then the variable UID.|
|lists|Declares the sprite specific lists|For regular lists, use `.list`, whether the monitor is visible as a `bool`, the UID as a `string`, the name as a `string`, the `string` values of the list separated by spaces, then the `end` keyword.<br><br>For example, `.list false "L001" "Logs" "Logging in" "Logged in" "Opened app" "Closed app" end` would create a list called `Logs` with the UID `L001`. The values would be `Logging in`, `Logged in`, `Opened app`, and `Closed app` in that order. The list would not be visible.<br><br>To change to global scope instead of sprite scope, use `.listglobalscope` then the list UID|
|broadcasts|Declares broadcasts|To declare a broadcast, use `.broadcast`, the broadcast UID, and the broadcast name. It throws a warning if not in the stage sprite. There is no sprite scope.|
|currentCostume|The current costume of a sprite.|Just use `.currentCostume` and the number of the current costume. It is zero-indexed.|
|costumes|Declares costumes|For SVGs, use `.costumeVector`, then the path to the image file as a `string`, the costume name as a `string`, the x position of the center of rotation, and the y position of the center of rotation. <br><br>For PNGs/JPEGs/other bitmap images, <big>UNFINISHED</big>|
|sounds|Declares sounds|<big>UNFINISHED</big>|
|volume|Declares the volume of the sounds|Use `.volume`, thenn a number.|





## Blocks
A block in BASM is declared with the `block` keyword at the start and the `end` keyword at the end

Inside the block, 8 fields must be provided:

### `uid`
Each block has a unique identifier (UID), which is given as a `string`. A block's UID shouldn't be the same as any other block's UID.

### `opcode`
Each block also has an `opcode`. Every opcode corresponds to a Scratch block. For example, `motion_movesteps` corresponds to the `Move () steps` block in the Motion category. The opcode is also written as a `string`.

### `parent`
The `parent` property points to the block stacked above the current block, or parent block. This property is given as a `string` containing the UID of the parent block. If there is no parent block, `null` is used. If the block is inside a C block, the UID of the C block is given.

### `next`
The `next` property serves the same purpose as the `parent` property, but for the block stacked below the current block.

### `in`
Inputs are the white text/number boxes inside blocks. They are each declared separately with the `in` keyword. Each input declaration includes, in this order,
1. The `in` keyword
2. The input's key (its name) as a string literal
3. The input's type
4. The value assigned to the input as a string literal or number literal

For example, to move 10 steps with the `motion_movesteps` block, you'd use:
```basm=
in "STEPS" double 10
```

### `field`
Fields are similar to inputs, but they are picked from a menu of options. They are declared with the `field` keyword. Each input declaration includes, in this order,
1. The `field` keyword
2. The field's key (its name) as a string literal
3. The value assigned to the field as a string literal

For example, to make a sprite draggable with the `sensing_setdragmode` block, you'd use:
```basm=
field DRAG_MODE "draggable"
```

**Note:** Most fields are used in shadow blocks (explained below), and the blocks you take from the block palette are instead using inputs that are `block_ptr`s to those shadow blocks. But, you can tell a block using a normal field and a shadow block are. If the field is the same color as the block and is rectangular, it is a normal field. If the field is slightly darker than the block and is round, it uses a shadow block.

### `shadow`
Every block you can take from the block palette in the editor is not a shadow block, so you'd usually use `shadow false`. But, sometimes, blocks have round fields. That means there is a shadow block in use. The actual block is not a shadow block, but it has a `block_ptr` input which points to a shadow block, which has the actual field. Shadow blocks would use `shadow true` inside them.

### `topLevel`
There are blocks called "hat blocks", which never have parent blocks and start up scripts. You would use `topLevel true` for these, because they are at the top level (nothing can be stacked above them). Everything else is `topLevel false`.

todo: read the rest of "sounds":[],"volume":100,"layerOrder":0,"tempo":60,"videoTransparency":50,"videoState":"on","textToSpeechLanguage":null}],"monitors":[],"extensions":[],"meta":{"semver":"3.0.0","vm":"12.0.2-hotfix","agent":"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36"}}

todo: check scratch wiki page for sb3