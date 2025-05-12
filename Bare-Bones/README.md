# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

{% if is_tailwind -%}
### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```
{%- endif %}

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

{% if is_fullstack -%}
```bash
dx serve --platform {{default_platform}}
```
{%- else -%}
```bash
dx serve
```
{%- endif %}

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

{% if is_mobile -%}
To serve on mobile, you need to explicitly set your target device, `android` or `ios`:
```bash
dx serve --platform android
```
{%- endif %}
