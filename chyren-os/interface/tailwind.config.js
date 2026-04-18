/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        background: '#0a0a0a', // Deep Space
        primary: '#00f2ff',    // Neon Cyan
        secondary: '#ff00ff',  // Neon Magenta
        text: '#e2e8f0',       // Off-white
      },
    },
  },
  plugins: [],
}
