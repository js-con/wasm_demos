/*
 * @Author: Lin Bowen
 * @Date: 2021-10-12 21:04:24
 * @LastEditTime: 2021-10-12 22:55:55
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \Projects\wasm_demos\wasm-rust-md5-demo\wasm\src\lib.rs
 */

use wasm_bindgen::prelude::*;
use md5;

#[wasm_bindgen(js_name = RustMD5)]
pub fn hasher(data:&str) -> String{
    let digest = md5::compute(data);
    let res = format!("{:x}",digest);
    res
}