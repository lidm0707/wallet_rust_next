/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        './app/**/*.{js,ts,jsx,tsx}', // For Next.js app directory
        './pages/**/*.{js,ts,jsx,tsx}', // For traditional Next.js pages
        './components/**/*.{js,ts,jsx,tsx}', // For components directory
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('tailwindcss-animate'), // Add this plugin
    ],
};
