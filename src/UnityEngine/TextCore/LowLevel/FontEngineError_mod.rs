#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngineError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontEngineError {
    Atlas_Generation_Cancelled = 100i32,
    Invalid_Character_Code = 17i32,
    Invalid_Face = 35i32,
    Invalid_File = 4i32,
    Invalid_File_Format = 2i32,
    Invalid_File_Path = 1i32,
    Invalid_File_Structure = 3i32,
    Invalid_Glyph_Index = 16i32,
    Invalid_Library = 33i32,
    Invalid_Library_or_Face = 41i32,
    Invalid_Pixel_Size = 23i32,
    Invalid_SharedTextureData = 101i32,
    Invalid_Table = 8i32,
    OpenTypeLayoutLookup_Mismatch = 116i32,
    Success = 0i32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngineError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::LowLevel::FontEngineError
    => "UnityEngine.TextCore.LowLevel"."FontEngineError"
);
