# Terminal Renderer

Double buffered terminal renderer for Rust and crossterm

## Why ?

I needed a simple way to render terminal based games. At first I was using [masof](https://github.com/da-x/masof//) by da-x (which this project is heavily inspired by and some code is a little too much similar to) but I wanted to have a more flexible way to render the screen. So I decided to write my own.

## How ?

The simplest way to use this is to create a `Renderer`, feed it events from crossterm, draw `Drawable` objects to it and finally render it to the terminal.

## What this is not ?

This project does not handle input, communication with OS or anything like that. It is only a renderer.

## Features

- [x] Double buffering
- [x] Frames (basically rect with pos and size which can be clipped or not)
- [x] Multiplatform
- [ ] Layers

## Examples

You can find them in the `examples` folder as other rust projects.