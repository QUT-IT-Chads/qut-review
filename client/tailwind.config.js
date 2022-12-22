/** @type {import('tailwindcss').Config} */
module.exports = {
    theme: {
        colors: {
            'default-gray': '#4b5563',
            'light-gray': '#5b6778',
            'light-blue': '#3287fd',
            'dark-blue': '#245aac',
            yellow: '#ffdd29'
        }
    },
    content: [
        './app/**/*.{js,ts,jsx,tsx}',
        './pages/**/*.{js,ts,jsx,tsx}',
        './components/**/*.{js,ts,jsx,tsx}'
    ],
    mode: 'jit'
};
