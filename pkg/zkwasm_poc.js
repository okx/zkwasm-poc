import * as wasm from "./zkwasm_poc_bg.wasm";
import { __wbg_set_wasm } from "./zkwasm_poc_bg.js";
__wbg_set_wasm(wasm);
export * from "./zkwasm_poc_bg.js";
