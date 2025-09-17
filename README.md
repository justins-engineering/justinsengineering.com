# justinsengineering.com

## Development

### Requirements
- [Bun](https://bun.com/get)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/)

### Tailwind CSS
1. Run the following command in the root of the project:
```bash
bun install
```
2. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css --watch
```

### Serving The App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --ssg
```
