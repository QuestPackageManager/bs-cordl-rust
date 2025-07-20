#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TextGeneratorUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextGeneratorUtilities";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
impl crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    pub fn AdjustLineOffset(
        startIndex: i32,
        endIndex: i32,
        offset: f32,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("AdjustLineOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AdjustLineOffset", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (startIndex, endIndex, offset, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Approximately(a: f32, b: f32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32, f32), bool, 2usize>("Approximately")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Approximately", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToFloat_ByRefMut1(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
        lastIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        f32,
                        4usize,
                    >("ConvertToFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertToFloat", 4usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (chars, startIndex, length, lastIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToFloat_Il2CppArray_i32_i32_0(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                        ),
                        f32,
                        3usize,
                    >("ConvertToFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertToFloat", 3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (chars, startIndex, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToUTF32(
        highSurrogate: u32,
        lowSurrogate: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u32, u32), u32, 2usize>("ConvertToUTF32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertToUTF32", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (highSurrogate, lowSurrogate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillCharacterVertexBuffers(
        i: i32,
        convertToLinearSpace: bool,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            i32,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("FillCharacterVertexBuffers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FillCharacterVertexBuffers", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (i, convertToLinearSpace, generationSettings, textInfo),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillSpriteVertexBuffers(
        i: i32,
        convertToLinearSpace: bool,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            i32,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("FillSpriteVertexBuffers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FillSpriteVertexBuffers", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (i, convertToLinearSpace, generationSettings, textInfo),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GammaToLinear_Color32_0(
        c: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::Color32),
                        crate::UnityEngine::Color32,
                        1usize,
                    >("GammaToLinear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GammaToLinear", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            method.invoke_unchecked((), (c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GammaToLinear_u8_1(value: u8) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u8), u8, 1usize>("GammaToLinear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GammaToLinear", 1usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeParameters(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
        parameters: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<f32>,
                                >,
                            >,
                        ),
                        i32,
                        4usize,
                    >("GetAttributeParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetAttributeParameters", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (chars, startIndex, length, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkupTagHashCode_Il2CppArray1(
        styleDefinition: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        >,
        readIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u32>,
                            >,
                            i32,
                        ),
                        i32,
                        2usize,
                    >("GetMarkupTagHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetMarkupTagHashCode", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (styleDefinition, readIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkupTagHashCode_TextBackingContainer0(
        styleDefinition: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        readIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextCore::Text::TextBackingContainer, i32),
                        i32,
                        2usize,
                    >("GetMarkupTagHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetMarkupTagHashCode", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (styleDefinition, readIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStyle(
        generationSetting: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextStyle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::TextStyle,
                        >,
                        2usize,
                    >("GetStyle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetStyle", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextStyle,
        > = unsafe { method.invoke_unchecked((), (generationSetting, hashCode))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleHashCode_ByRefMut_i32_ByRefMut0(
        text: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
        index: i32,
        closeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<u32>,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        i32,
                        3usize,
                    >("GetStyleHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetStyleHashCode", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (text, index, closeIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleHashCode_ByRefMut_i32_ByRefMut1(
        text: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::Text::TextBackingContainer,
        >,
        index: i32,
        closeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::TextCore::Text::TextBackingContainer,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        i32,
                        3usize,
                    >("GetStyleHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetStyleHashCode", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (text, index, closeIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF16_Il2CppArray0(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u32>,
                            >,
                            i32,
                        ),
                        u32,
                        2usize,
                    >("GetUTF16")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetUTF16", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (text, i))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF16_TextBackingContainer1(
        text: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextCore::Text::TextBackingContainer, i32),
                        u32,
                        2usize,
                    >("GetUTF16")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetUTF16", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (text, i))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF32_Il2CppArray0(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u32>,
                            >,
                            i32,
                        ),
                        u32,
                        2usize,
                    >("GetUTF32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetUTF32", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (text, i))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF32_TextBackingContainer1(
        text: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextCore::Text::TextBackingContainer, i32),
                        u32,
                        2usize,
                    >("GetUTF32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetUTF32", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (text, i))? };
        Ok(__cordl_ret.into())
    }
    pub fn HexCharsToColor_Il2CppArray_i32_0(
        hexChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        tagCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::Color32,
                        2usize,
                    >("HexCharsToColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HexCharsToColor", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            method.invoke_unchecked((), (hexChars, tagCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HexCharsToColor_i32_1(
        hexChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                        ),
                        crate::UnityEngine::Color32,
                        3usize,
                    >("HexCharsToColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HexCharsToColor", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            method.invoke_unchecked((), (hexChars, startIndex, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HexToInt(hex: char) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(char), u32, 1usize>("HexToInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HexToInt", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (hex))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertClosingStyleTag(
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingElement,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
                                            i32,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("InsertClosingStyleTag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InsertClosingStyleTag", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        charBuffer,
                        writeIndex,
                        textStyleStackDepth,
                        textStyleStacks,
                        generationSettings,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertClosingTextStyle(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextStyle>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextStyle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingElement,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
                                            i32,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("InsertClosingTextStyle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InsertClosingTextStyle", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        style,
                        charBuffer,
                        writeIndex,
                        textStyleStackDepth,
                        textStyleStacks,
                        generationSettings,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertOpeningStyleTag(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextStyle>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextStyle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingElement,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
                                            i32,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("InsertOpeningStyleTag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InsertOpeningStyleTag", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        style,
                        charBuffer,
                        writeIndex,
                        textStyleStackDepth,
                        textStyleStacks,
                        generationSettings,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertOpeningTextStyle(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextStyle>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextStyle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingElement,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
                                            i32,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("InsertOpeningTextStyle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InsertOpeningTextStyle", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        style,
                        charBuffer,
                        writeIndex,
                        textStyleStackDepth,
                        textStyleStacks,
                        generationSettings,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertTextStyleInTextProcessingArray(
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        styleDefinition: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        >,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingElement,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u32>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
                                            i32,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("InsertTextStyleInTextProcessingArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InsertTextStyleInTextProcessingArray", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        charBuffer,
                        writeIndex,
                        styleDefinition,
                        textStyleStackDepth,
                        textStyleStacks,
                        generationSettings,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsBaseGlyph(c: u32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u32), bool, 1usize>("IsBaseGlyph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsBaseGlyph", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsBitmapRendering(
        glyphRenderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode),
                        bool,
                        1usize,
                    >("IsBitmapRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsBitmapRendering", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (glyphRenderMode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCJK(c: u32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u32), bool, 1usize>("IsCJK")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsCJK", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsHangul(c: u32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u32), bool, 1usize>("IsHangul")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsHangul", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUTF16(
        text: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextCore::Text::TextBackingContainer, i32),
                        bool,
                        2usize,
                    >("IsValidUTF16")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsValidUTF16", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (text, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUTF32(
        text: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextCore::Text::TextBackingContainer, i32),
                        bool,
                        2usize,
                    >("IsValidUTF32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsValidUTF32", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (text, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn LegacyAlignmentToNewAlignment(
        anchor: crate::UnityEngine::TextAnchor,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::Text::TextAlignment,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextAnchor),
                        crate::UnityEngine::TextCore::Text::TextAlignment,
                        1usize,
                    >("LegacyAlignmentToNewAlignment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LegacyAlignmentToNewAlignment", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::Text::TextAlignment = unsafe {
            method.invoke_unchecked((), (anchor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LegacyStyleToNewStyle(
        fontStyle: crate::UnityEngine::FontStyle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::Text::FontStyles> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::FontStyle),
                        crate::UnityEngine::TextCore::Text::FontStyles,
                        1usize,
                    >("LegacyStyleToNewStyle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LegacyStyleToNewStyle", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::Text::FontStyles = unsafe {
            method.invoke_unchecked((), (fontStyle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MinAlpha(
        c1: crate::UnityEngine::Color,
        c2: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::Color, crate::UnityEngine::Color),
                        crate::UnityEngine::Color,
                        2usize,
                    >("MinAlpha")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MinAlpha", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), (c1, c2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PackUV(
        x: f32,
        y: f32,
        scale: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (f32, f32, f32),
                        crate::UnityEngine::Vector2,
                        3usize,
                    >("PackUV")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PackUV", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (x, y, scale))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceClosingStyleTag(
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingElement,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
                                            i32,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ReplaceClosingStyleTag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReplaceClosingStyleTag", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        charBuffer,
                        writeIndex,
                        textStyleStackDepth,
                        textStyleStacks,
                        generationSettings,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceOpeningStyleTag_ByRefMut_i32_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut0(
        sourceText: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::Text::TextBackingContainer,
        >,
        srcIndex: i32,
        srcOffset: quest_hook::libil2cpp::ByRefMut<i32>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::TextCore::Text::TextBackingContainer,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingElement,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
                                            i32,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                                >,
                            >,
                        ),
                        bool,
                        8usize,
                    >("ReplaceOpeningStyleTag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReplaceOpeningStyleTag", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        sourceText,
                        srcIndex,
                        srcOffset,
                        charBuffer,
                        writeIndex,
                        textStyleStackDepth,
                        textStyleStacks,
                        generationSettings,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceOpeningStyleTag_ByRefMut_i32_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut1(
        sourceText: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
        srcIndex: i32,
        srcOffset: quest_hook::libil2cpp::ByRefMut<i32>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<u32>,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingElement,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
                                            i32,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                                >,
                            >,
                        ),
                        bool,
                        8usize,
                    >("ReplaceOpeningStyleTag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReplaceOpeningStyleTag", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        sourceText,
                        srcIndex,
                        srcOffset,
                        charBuffer,
                        writeIndex,
                        textStyleStackDepth,
                        textStyleStacks,
                        generationSettings,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResizeInternalArray_ByRefMut0<T>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResizeInternalArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ResizeInternalArray", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResizeInternalArray_i32_1<T>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<T>,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ResizeInternalArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ResizeInternalArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResizeLineExtents(
        _cordl_size: i32,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ResizeLineExtents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ResizeLineExtents", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_cordl_size, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperASCIIFast__cordl_char0(
        c: char,
    ) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(char), char, 1usize>("ToUpperASCIIFast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToUpperASCIIFast", 1usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperASCIIFast_u32_1(c: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u32), u32, 1usize>("ToUpperASCIIFast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToUpperASCIIFast", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperFast(c: char) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(char), char, 1usize>("ToUpperFast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToUpperFast", 1usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
