# What YAM Should Copy

This is the compressed takeaway from the external inspiration set.

## Runtime shape

- keep the app loop explicit
- keep terminal lifecycle explicit
- keep timing, input, and redraw behavior visible
- keep debug output separate from scene output

## Composition model

- treat terminal output as a composited pipeline
- keep layout and width handling first-class
- make overlay layering explicit
- compose from fields, masks, and buffers instead of isolated writes

## Conversion model

- treat Chafa-like rendering as preprocessing plus compression
- tune source quality, thresholding, palette, and symbol density before blaming final render code
- make glyph density a deliberate choice

## Workflow model

- use examples and tiny runnable presets as the main onboarding path
- keep reusable presets and recipes around
- make reproducible scene states easy to launch
- keep layout, conversion, and growth changes independently testable

## Tooling model

- keep thin wrappers/adapters around shared cores when needed
- make debug and development workflows explicit
- provide discoverable commands for common presets and introspection
- add sandbox or preview surfaces when exploration cost is high

## Layout model

- prefer explicit, composable layout primitives
- treat Unicode width and terminal cell sizing as real constraints
- keep placement resolution-independent where possible

## What this means for YAM

YAM should stay focused on:

- scene composition
- mask/field-driven layout
- reproducible recipes
- debug/introspection tooling
- small, explicit runtime surfaces

YAM should not drift toward:

- a generic terminal toolkit
- a monolithic renderer
- isolated text writes without a shared composition model
