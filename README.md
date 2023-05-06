# Learned Leptos by building this

I spent about a weeks worth of time to learn leptos. In order to understand the examples I basicly rewrote a lot of them to my style and created a page to show diffent SPA concepts in a simple page.

## Demo

I took the resulting release build and dropped it on netlify. [leptos-app](https://fascinating-heliotrope-4ad20c.netlify.app/)

## How to run/build

Eseentially you need to run `trunk serve` in the root directory. In order to build the css you can run a simple command. If you want the tailwind cli to watch the folder for changes you can add the `-w` flag at the end of the associated command and run **trunk** on a seperate terminal.

### tldr

```bash
npx tailwindcss -i input.pcss -o output.css
trunk serve
```
