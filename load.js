import http from "k6/http";
import { check } from "k6";

/**
 * ⚡️ TURBO PERFORMANCE COMPARISON ⚡️
 * - Optimized for maximum Requests Per Second (RPS).
 * - Discards response bodies to save client CPU/RAM.
 * - Randomizes assets to prevent browser-specific optimization.
 */

export const options = {
  discardResponseBodies: true, // 🔥 CRITICAL for max RPS
  scenarios: {
    turbo_load: {
      executor: "constant-vus",
      vus: 500,
      duration: "1m",
    },
  },
  thresholds: {
    http_req_failed: ["rate<0.05"],
  },
};

const BASE_URL = __ENV.URL || "http://127.0.0.1:8080";

export function setup() {
  console.log(`\n🚀 STARTING TURBO LOAD TEST`);
  console.log(`📍 Target URL: ${BASE_URL}`);
  console.log(`⚠️  Mode: Multi-Asset Random (Max RPS)\n`);
}

const ASSETS = [
  "/index.html",
  "/assets/index-Cp8FtQGN.js",
  "/assets/index-CcxdxiUR.css",
  "/tanstack-circle-logo.png",
];

export default function () {
  // Randomly pick one asset per iteration to maximize RPS
  const path = ASSETS[Math.floor(Math.random() * ASSETS.length)];
  
  const params = {
    headers: {
      "Accept-Encoding": "br, gzip", // Request compression
    },
  };

  const res = http.get(`${BASE_URL}${path}`, params);
  
  // Minimal check to keep CPU overhead low, but verify compression
  check(res, { 
    "status is 200": (r) => r.status === 200,
    "is compressed": (r) => r.headers["Content-Encoding"] === "br" || r.headers["Content-Encoding"] === "gzip" || r.headers["content-encoding"] === "br" || r.headers["content-encoding"] === "gzip",
  });
}
