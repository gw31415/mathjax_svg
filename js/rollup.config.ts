import commonjs from "@rollup/plugin-commonjs";
import nodeResolve from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";
import type { RollupOptions } from "rollup";

export default {
  input: "src/index.ts",
  output: {
    file: "out/index.mjs",
    format: "esm",
  },
  plugins: [
    typescript({
      tsconfig: "tsconfig.json"
    }),
    nodeResolve(),
    commonjs(),
    terser(),

  ]
} satisfies RollupOptions;
