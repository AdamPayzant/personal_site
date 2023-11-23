/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        aspectRatio: {
          '7/4': '7 / 4',
        },
        spacing: {
          '15%': '15%',
          '20%': '20%',
          '30%': '30%',
          '50%': '50%',
        },
      },
    },
    plugins: [],
  }