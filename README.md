# justinsengineering.com

## Requirements
- [Ruby](https://github.com/justins-engineering/vzw-nidd-front-end/blob/main/.ruby-version#L1)
- [Bun](https://bun.sh/)

## Setup
1. ```sh
   gem install bundler
   ```
2. ```sh
   bundle install
   ```
3. ```sh
   bun install
   ```

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```
