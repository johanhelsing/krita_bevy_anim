# Krita to Bevy animation workflow

Goal: 1-click in Krita, see animation in game within seconds (milliseconds?)

I tried to find a nice way to export krita animations to something Bevy would
accept, but couldn't find any, this is what I've got so far.

## Status

Work in progress!

The workflow... works, but there are a few manual steps left

- [ ] make bevy_krita_anim able to watch itself, so we don't need `watchexec`
- [ ] support for `bevy_trickfilm` animation manifest format?
- [ ] support for multi-row atlases

## Prerequisites

- Krita 5+
- cargo

## Preparation

Install the atlas baking tool from this repository:

```shell
cargo install --git https://github.com/johanhelsing/krita_bevy_anim
```

Install `watchexec-cli` so we can trigger baking automatically after rendering:

```shell
cargo install watchexec-cli
```

Create an *empty* render folder

```shell
mkdir -p /path/to/render
```

Watch for renders appearing here:

```shell
cd /path/to/render
watchexec -e png "krita_bevy_anim . --rm --output /path/to/game/assets/your_anim"
```

This will watch for new images appearing in the render folder and 

Let this run in the background while you work in Krita

## Workflow

### 1. Make an animation

Make an animation with Krita, using animation tools with dope sheet, onion
skinning etc.

### 2. Render the animation

Hit `File` -> `Render Animation`

- Choose "Image sequence"
- Set the location to the empty render folder you created earlier
- Leave "base name" empty (required, for now at least)
- Choose "Only render unique frames"

Don't worry, Krita will remember these settings. Next time you'll only need to
hit render and ok.

Our watcher script will detect the render

This will create a couple of files in your assets folder:

- `your_anim.png`: All the sprites in one texture
- `your_anim.titan`: Manifest file containing details about how the atlas is
  partitioned.
- `your_anim.flippy`: A file containing frame timing

## Using the assets in your game

### 1. Add `bevy_titan` plugin

Add `bevy_titan` to your game's dependencies and add its
`SpriteSheetLoaderPlugin`. This is a tiny plugin that adds support for the
`.titan` sprite sheet manifest, we generated earlier, which we're using to
communicate grid size and dimensions to Bevy.

### 2. Load the `.titan` file

```rust
let anim_atlas: Handle<TextureAtlas> = asset_server.load("your_anim.titan");
```

TODO: explain how to use timings file.

### 3. Enable asset hot reloading

Read bevy docs for how.

You should now be good to go, for subsequent updates, simply leave your game
running, then:

1. render the animation
2. run the baking command

And you should see your updated animation.

## The solutions I didn't like

https://github.com/Falano/kritaSpritesheetManager only supports fixed
intervals, meaning your sprite sheets will have to run at a homogenous speed,
meaning you will can't time animations as you want, or you'll have to choose a
really high speed and have lots of wasted identical frames.

Another issue with it, is that it seems to bake the onion skinning trails into
the actual sprites, so you'll have a lot of green and red in your game unless
you remember to turn that off before exporting.

