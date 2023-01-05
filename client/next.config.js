/** @type {import('next').NextConfig} */
module.exports = {
    reactStrictMode: true,
    eslint: {
        dirs: ['app', 'components', 'helper', 'types', 'pages']
    },
    experimental: {
        appDir: true
    }
};
