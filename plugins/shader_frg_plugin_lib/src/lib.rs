/*
MIT License

Copyright (c) 2023 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use bevy::{asset::load_internal_asset, prelude::*, reflect::TypeUuid};

pub const HANDLE_SHADER_CHUNK_RANDOM: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7837514426031940724);
pub const HANDLE_SHADER_CHUNK_NOISE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 6830514420931920724);

pub struct ShaderLibPlugin;

impl Plugin for ShaderLibPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE_SHADER_CHUNK_RANDOM,
            "random.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            HANDLE_SHADER_CHUNK_NOISE,
            "noise.wgsl",
            Shader::from_wgsl
        );
    }
}
