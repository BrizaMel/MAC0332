/** @type {import('next').NextConfig} */

module.exports = {
  reactStrictMode: true,
  env: {
    MOCKED: process.env.MOCKED,
    ENDPOINT_URL: process.env.ENDPOINT_URL,
  }
};
