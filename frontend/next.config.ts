import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Empty turbopack config to indicate Turbopack is intentionally used
  // Node.js polyfills are not needed for browser builds
  turbopack: {},
  webpack: (config) => {
    config.resolve.fallback = {
      ...config.resolve.fallback,
      fs: false,
      net: false,
      tls: false,
    };
    return config;
  },
};

export default nextConfig;
