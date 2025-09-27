import { Global, css } from '@emotion/react';

export default function GlobalStyle() {
  return (
    <Global
      styles={css`
        :root {
          --button-bg-color: hsl(289 55.9% 38.4%);
          --button-color: hsl(289 7.9% 88.4%);

          --shadow-color: 289deg 76% 5%;
          --shadow-elevation-low:
            0.3px 0.3px 0.5px hsl(var(--shadow-color) / 0.23),
            0.5px 0.5px 0.9px -0.8px hsl(var(--shadow-color) / 0.31),
            1.2px 1.2px 2.1px -1.6px hsl(var(--shadow-color) / 0.39);
          --shadow-elevation-medium:
            0.3px 0.3px 0.5px hsl(var(--shadow-color) / 0.25),
            1.1px 1.1px 1.9px -0.5px hsl(var(--shadow-color) / 0.31),
            2.5px 2.6px 4.5px -1.1px hsl(var(--shadow-color) / 0.37),
            5.8px 5.8px 10.3px -1.6px hsl(var(--shadow-color) / 0.44);
          --shadow-elevation-high:
            0.3px 0.3px 0.5px hsl(var(--shadow-color) / 0.23),
            2.2px 2.2px 3.9px -0.2px hsl(var(--shadow-color) / 0.26),
            3.9px 4px 7px -0.5px hsl(var(--shadow-color) / 0.29),
            6.2px 6.2px 11px -0.7px hsl(var(--shadow-color) / 0.32),
            9.3px 9.4px 16.6px -0.9px hsl(var(--shadow-color) / 0.35),
            13.9px 14px 24.7px -1.2px hsl(var(--shadow-color) / 0.37),
            20.4px 20.6px 36.3px -1.4px hsl(var(--shadow-color) / 0.4),
            29.4px 29.6px 52.3px -1.6px hsl(var(--shadow-color) / 0.43);
        }

        html, body {
          margin: 0;
          padding: 0;
        }

        body {
          background-color: hsl(289 3.2% 9.2%);
          color: hsl(289 7.9% 88.4%);
        }

      `}
    />
  );
}
