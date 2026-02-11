# justinsengineering.com

## Development

### Requirements

- [Bun](https://bun.com/get)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/)

### Tailwind CSS

1. Run the following command in the root of the project:
```sh
bun install
```
2. Run the following command in the root of the project to start the Tailwind CSS compiler:

```sh
bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css --watch
```

### Serving The App for Development

Run the following command in the root of your project to start developing with the default platform:

```sh
dx serve --ssg
```

## Building/Testing

### Bundling

1. Run the following command in the root of the project to compile and minify the Tailwind CSS:

```sh
bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css --minify
```

2. Run the following command in the root of your project to bundle the assets:

```sh
dx build --web --ssg --release --debug-symbols=false
```

### Serving The App

Run the following command in the root of your project to bundle the assets:

```sh
bunx wrangler dev
```
