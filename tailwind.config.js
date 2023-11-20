/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        aspectRatio: {
          '7/4': '7 / 4',
        }
      },
    },
    plugins: [],
  }