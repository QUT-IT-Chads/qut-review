/** @type {import('next').NextConfig} */
module.exports = {
    reactStrictMode: true,
    eslint: {
        dirs: ['app', 'pages', 'components', 'types']
    },
    experimental: {
        appDir: true
    }
};
