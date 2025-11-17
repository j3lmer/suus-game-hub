# 1. Architectural Overview

## Core components

1. **Game State**

   * Current scene ID
   * Flags (e.g., “eyes_opened”, “has_glasses”)
   * Inventory
   * Lose/win state
   * Render options (ASCII art visibility, hidden-until-unlocked text)

2. **Scene Definitions**
   For each scene, define:

   * `id`
   * `ascii_art` (multiple sizes if needed)
   * `enter_text`
   * `actions` (command → effect)
   * `transitions` (e.g., “go to bathroom” → scene `bathroom`)
   * `constraints` (e.g., action requires glasses)
   * `on_enter` events
   * `dynamic_text` (optional, reacts to state)

3. **Command Parser**

   * Tokenizer: normalize case, remove punctuation
   * Verb–noun handler: “open eyes”, “put on glasses”, “go bathroom”
   * Fuzzy matching for autocomplete
   * Command resolver → specific action object

4. **Engine**

   * Accept command → parse → check availability → update state → produce:

     * text output
     * updated scene or description
     * lose/win detection

5. **UI Layer**

   * Scrollable output log
   * Input line with autocomplete suggestions
   * Task/goal panel
   * “Visual preview” ASCII frame
   * Inventory view (icons, ASCII, or simple list)

---

# 2. Scene Structure Example (JSON-style)

```json
{
  "scene_id": "bedroom_in_bed",
  "ascii_art": {
    "small": "....",
    "medium": "...."
  },
  "enter_text": "Het is zondag... Maar wacht eens even, alles is zwart!",
  "actions": {
    "open eyes": {
      "requires": [],
      "effects": [
        {"type": "flag_set", "flag": "eyes_open"},
        {"type": "reveal_ascii"}
      ],
      "result_text": "Ah, dat is beter..."
    },
    "put on glasses": {
      "requires": ["eyes_open"],
      "effects": [
        {"type": "flag_set", "flag": "glasses_on"}
      ],
      "result_text": "Zo hey, wat is alles scherp ineens!"
    },
    "doomscroll": {
      "requires": ["eyes_open"],
      "effects": [
        {"type": "trigger_lose", "reason": "instagram trap"}
      ],
      "result_text": "Je opent instagram..."
    },
    "get out of bed": {
      "requires": ["eyes_open"],
      "effects": [
        {"type": "change_scene", "scene": "bedroom_towards_closet"}
      ],
      "result_text": "Je stapt uit bed..."
    }
  }
}
```

This gives you a highly predictable pattern for all scenes.

---

# 3. How to Implement

## Step 1: Content separation

Put all story content in a folder like:

```
/story
  scenes.json
  items.json
  global_flags.json
```

or use markdown with front-matter for each scene:

```
scene_id: bedroom_in_bed
actions:
  - name: open eyes
    result: …
```

Keeping content out of the engine lets you expand the story easily.

---

## Step 2: Create a minimal engine loop

Pseudo-logic:

```
state = load_default_state()
while state.running:
    render_ui(state)
    cmd = get_user_input()
    action = parser.resolve(cmd, state.current_scene)
    if action invalid:
         show "Dat werkt niet."
    else:
         apply_effects(action.effects, state)
         push_to_log(action.result_text)
```

---

## Step 3: Autocomplete

Use a simple method first:

* Gather all actions for current scene + global verbs (“inventory”, “help”, “look”).
* When user types, show fuzzy matches (Levenshtein or simple “startsWith”).
* If only one match → auto-complete on TAB.

Later:

* Add dynamic suggestions based on items in room.
* Add contextual suggestions like “inspect [item]”.

---

## Step 4: ASCII Art Handling

You have two options:

### Manual sizing

Prepare ASCII at fixed widths:

* small: 30 chars
* medium: 60 chars
* full: 100 chars

UI picks based on panel width.

### Programmatic scaling

Use a monospaced “scaler” that compresses/expands ASCII by sampling characters. Quality is worse but easier.

Recommendation: Use fixed sizes. Artistic control matters in this genre.

---

## Step 5: Inventory Logic

At minimum:

```
inventory = ["telefoon"]
```

Add:

* item metadata (weight optional)
* ASCII representation for small preview
* optional “use” actions that trigger global effects or scene-specific overrides.

---

# 4. Implementation Strategy

## MVP first

1. Hardcode two scenes (your Scene 1 and 2).
2. Implement:

   * state
   * simple parser (exact match)
   * text log
   * ASCII display
3. Add inventory logic
4. Add transitions
5. Add lose screen

When everything works in minimal form, expand:

## Phase 2: Autocomplete & fuzzy commands

* Add command normalization
* Add synonyms (“opstaan” → “get out of bed”)

## Phase 3: Scene authoring workflow

* Move scenes into story files
* Create a loader

## Phase 4: QoL Features

* Save/load
* Animated typewriter text
* Achievement popups (e.g., “Stinky Sunday”)

---

# 5. Recommended directory structure

```
/engine
  parser.js
  state.js
  engine.js

/story
  scenes/
    bedroom_in_bed.json
    bedroom_closet.json
    bathroom.json
  items.json

/ui
  components/
    asciiPanel.js
    logPanel.js
    inventoryPanel.js
    inputBar.js

/assets
  ascii/
    bedroom_bed_small.txt
    bedroom_bed_medium.txt
```

---

# 6. Your existing draft integrated into this structure

* Scene 1: “bedroom_in_bed”
* Scene 2: “bedroom_towards_closet”
* Lose scenes: “doomscroll_lose”, “gilmore_girls_lose”
* Next: bathroom, toilet, hallway, living room, spaceship entry, alien kitten realm, rescue scene.

Place each scene’s `enter_text`, `actions`, and ASCII in separate scene files.

---

If you want, I can produce:

* an exact ready-to-use scene JSON schema
* a minimal runnable engine skeleton (any language)
* a first batch of scenes rewritten into the structured format
* an autocomplete algorithm design
