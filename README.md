# Krita to Bevy animation workflow

Goal: 1-click in Krita, see animation in game within seconds

I tried to find a nice way to export krita animations to something Bevy would
accept, but couldn't find any, this is what I've got so far.

## Status

Work in progress!

The workflow... works, but there are a few manual steps left

- [ ] make krita python plugin to render sprites and run atlas baker through
  shortcut
- [ ] add support for `bevy_trickfilm` animation format?
- [ ] add support for multi-row atlases

## Preparation

### 1. Install krita flipbook baking tool

```shell
cargo install --path .
```

### 2. Add `bevy_titan`

Add `bevy_titan` to your game's dependencies and add its
`SpriteSheetLoaderPlugin`. This is a tiny plugin that adds support for `.titan`
sprite sheet manifests, which we're using to communicate grid size and
dimensions to Bevy.

### 3. Enable asset hot reloading

Read bevy docs for how.

## Workflow

### 1. Make an animation

Make an animation with Krita, using animation tools with dope sheet, onion
skinning etc.

### 2. Render the animation

Hit `File` -> `Render Animation`

- Choose "Image sequence"
- Set the location to an empty folder
- Leave "base name" empty (required, for now at least)
- Choose "Only render unique frames"

Don't worry, Krita will remember these settings. Next time you'll only need to
hit render and ok.

### 3. Bake textures and timing info

TODO: automate this step in krita plugin

Run the tool you installed earlier:

```shell
bevy_krita_anim path/to/render_folder --output path/to/game/assets/your_anim
```

This will create a couple of files:

- `your_anim.png`: All the sprites in one texture
- `your_anim.titan`: Manifest file containing details about how the atlas is
  partitioned.
- `your_anim.flippy`: A file containing frame timing information

## The solutions I didn't like

https://github.com/Falano/kritaSpritesheetManager only supports fixed
intervals, meaning your sprite sheets will have to run at a homogenous speed,
meaning you will can't time animations as you want, or you'll have to choose a
really high speed and have lots of wasted identical frames.

Another issue with it, is that it seems to bake the onion skinning trails into
the actual sprites, so you'll have a lot of green and red in your game unless
you remember to turn that off before exporting.

