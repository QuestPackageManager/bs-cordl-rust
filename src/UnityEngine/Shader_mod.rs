#[cfg(feature = "UnityEngine+Shader")]
#[repr(C)]
#[derive(Debug)]
pub struct Shader {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Shader")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Shader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Shader";
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
#[cfg(feature = "UnityEngine+Shader")]
impl std::ops::Deref for crate::UnityEngine::Shader {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Shader")]
impl std::ops::DerefMut for crate::UnityEngine::Shader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Shader")]
impl crate::UnityEngine::Shader {
    pub fn CheckPropertyIndex(
        s: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CheckPropertyIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "CheckPropertyIndex", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (s, propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeywordFast(
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Rendering::GlobalKeyword),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableKeywordFast")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "DisableKeywordFast", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeywordFast_Injected(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Rendering::GlobalKeyword,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableKeywordFast_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "DisableKeywordFast_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_ByRefMut1(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Rendering::GlobalKeyword,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableKeyword")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "DisableKeyword", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_Il2CppString0(
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableKeyword")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "DisableKeyword", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeywordFast(
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Rendering::GlobalKeyword),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EnableKeywordFast")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "EnableKeywordFast", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeywordFast_Injected(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Rendering::GlobalKeyword,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EnableKeywordFast_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "EnableKeywordFast_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_ByRefMut1(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Rendering::GlobalKeyword,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EnableKeyword")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "EnableKeyword", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_Il2CppString0(
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EnableKeyword")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "EnableKeyword", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalFloatArray(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<f32>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ExtractGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "ExtractGlobalFloatArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalFloatArrayImpl(
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<f32>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ExtractGlobalFloatArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "ExtractGlobalFloatArrayImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalMatrixArray(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Matrix4x4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ExtractGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "ExtractGlobalMatrixArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalMatrixArrayImpl(
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ExtractGlobalMatrixArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "ExtractGlobalMatrixArrayImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalVectorArray(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Vector4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ExtractGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "ExtractGlobalVectorArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalVectorArrayImpl(
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Vector4,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ExtractGlobalVectorArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "ExtractGlobalVectorArrayImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Find(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                1usize,
            >("Find")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "Find", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindBuiltin(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                1usize,
            >("FindBuiltin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "FindBuiltin", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindPassTagValue_ShaderTagId0(
        &mut self,
        passIndex: i32,
        tagName: crate::UnityEngine::Rendering::ShaderTagId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ShaderTagId> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, crate::UnityEngine::Rendering::ShaderTagId),
                crate::UnityEngine::Rendering::ShaderTagId,
                2usize,
            >("FindPassTagValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "FindPassTagValue", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderTagId = unsafe {
            method.invoke_unchecked(self, (passIndex, tagName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindPassTagValue_i32_ShaderTagId1(
        &mut self,
        subshaderIndex: i32,
        passIndex: i32,
        tagName: crate::UnityEngine::Rendering::ShaderTagId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ShaderTagId> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::Rendering::ShaderTagId),
                crate::UnityEngine::Rendering::ShaderTagId,
                3usize,
            >("FindPassTagValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "FindPassTagValue", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderTagId = unsafe {
            method.invoke_unchecked(self, (subshaderIndex, passIndex, tagName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindPropertyIndex(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("FindPropertyIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "FindPropertyIndex", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (propertyName))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindSubshaderTagValue(
        &mut self,
        subshaderIndex: i32,
        tagName: crate::UnityEngine::Rendering::ShaderTagId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ShaderTagId> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, crate::UnityEngine::Rendering::ShaderTagId),
                crate::UnityEngine::Rendering::ShaderTagId,
                2usize,
            >("FindSubshaderTagValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "FindSubshaderTagValue", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderTagId = unsafe {
            method.invoke_unchecked(self, (subshaderIndex, tagName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindTextureStack(
        &mut self,
        propertyIndex: i32,
        stackName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        layerIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                3usize,
            >("FindTextureStack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "FindTextureStack", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (propertyIndex, stackName, layerIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindTextureStackImpl(
        s: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIdx: i32,
        stackName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        layerIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                4usize,
            >("FindTextureStackImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "FindTextureStackImpl", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, propertyIdx, stackName, layerIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllGlobalKeywords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::UnityEngine::Rendering::GlobalKeyword,
                    >,
                >,
                0usize,
            >("GetAllGlobalKeywords")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetAllGlobalKeywords", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDependency(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                1usize,
            >("GetDependency")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetDependency", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = unsafe {
            method.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnabledGlobalKeywords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::UnityEngine::Rendering::GlobalKeyword,
                    >,
                >,
                0usize,
            >("GetEnabledGlobalKeywords")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetEnabledGlobalKeywords", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalColor_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::UnityEngine::Color,
                1usize,
            >("GetGlobalColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalColor", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalColor_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::UnityEngine::Color,
                1usize,
            >("GetGlobalColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalColor", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), (nameID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArrayCountImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("GetGlobalFloatArrayCountImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloatArrayCountImpl", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArrayImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                1usize,
            >("GetGlobalFloatArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloatArrayImpl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArray_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                1usize,
            >("GetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloatArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArray_Il2CppString_List_1_2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<f32>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloatArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArray_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                1usize,
            >("GetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloatArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = unsafe { method.invoke_unchecked((), (nameID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArray_i32_List_1_3(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<f32>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloatArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatImpl(name: i32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), f32, 1usize>("GetGlobalFloatImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloatImpl", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloat_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                f32,
                1usize,
            >("GetGlobalFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloat", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloat_i32_1(nameID: i32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), f32, 1usize>("GetGlobalFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalFloat", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (nameID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalIntImpl(name: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("GetGlobalIntImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalIntImpl", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalInt_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("GetGlobalInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalInt", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalInt_i32_1(nameID: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("GetGlobalInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalInt", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (nameID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalInteger_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("GetGlobalInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalInteger", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalInteger_i32_1(nameID: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("GetGlobalInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalInteger", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (nameID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArrayCountImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("GetGlobalMatrixArrayCountImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrixArrayCountImpl", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArrayImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                >,
                1usize,
            >("GetGlobalMatrixArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrixArrayImpl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArray_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                >,
                1usize,
            >("GetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrixArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArray_Il2CppString_List_1_2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Matrix4x4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrixArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArray_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                >,
                1usize,
            >("GetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrixArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = unsafe { method.invoke_unchecked((), (nameID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArray_i32_List_1_3(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Matrix4x4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrixArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::UnityEngine::Matrix4x4,
                1usize,
            >("GetGlobalMatrixImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrixImpl", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixImpl_Injected(
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetGlobalMatrixImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrixImpl_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrix_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::UnityEngine::Matrix4x4,
                1usize,
            >("GetGlobalMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrix", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrix_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::UnityEngine::Matrix4x4,
                1usize,
            >("GetGlobalMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalMatrix", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked((), (nameID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalTextureImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                1usize,
            >("GetGlobalTextureImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalTextureImpl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalTexture_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                1usize,
            >("GetGlobalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalTexture", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalTexture_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                1usize,
            >("GetGlobalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalTexture", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            method.invoke_unchecked((), (nameID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArrayCountImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("GetGlobalVectorArrayCountImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVectorArrayCountImpl", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArrayImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                >,
                1usize,
            >("GetGlobalVectorArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVectorArrayImpl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArray_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                >,
                1usize,
            >("GetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVectorArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArray_Il2CppString_List_1_2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Vector4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVectorArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArray_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                >,
                1usize,
            >("GetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVectorArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = unsafe { method.invoke_unchecked((), (nameID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArray_i32_List_1_3(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Vector4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVectorArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::UnityEngine::Vector4,
                1usize,
            >("GetGlobalVectorImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVectorImpl", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorImpl_Injected(
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetGlobalVectorImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVectorImpl_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVector_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::UnityEngine::Vector4,
                1usize,
            >("GetGlobalVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVector", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVector_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::UnityEngine::Vector4,
                1usize,
            >("GetGlobalVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetGlobalVector", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked((), (nameID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPassCountInSubshader(
        &mut self,
        subshaderIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetPassCountInSubshader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPassCountInSubshader", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (subshaderIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyAttributes_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                2usize,
            >("GetPropertyAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyAttributes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked((), (shader, propertyIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyAttributes_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                1usize,
            >("GetPropertyAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyAttributes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked(self, (propertyIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetPropertyCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultFloatValue(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), f32, 1usize>("GetPropertyDefaultFloatValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDefaultFloatValue", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked(self, (propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultIntValue_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                i32,
                2usize,
            >("GetPropertyDefaultIntValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDefaultIntValue", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (shader, propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultIntValue_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetPropertyDefaultIntValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDefaultIntValue", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultValue(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                crate::UnityEngine::Vector4,
                2usize,
            >("GetPropertyDefaultValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDefaultValue", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked((), (shader, propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultValue_Injected(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetPropertyDefaultValue_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDefaultValue_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (shader, propertyIndex, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultVectorValue(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                crate::UnityEngine::Vector4,
                1usize,
            >("GetPropertyDefaultVectorValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDefaultVectorValue", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked(self, (propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDescription_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetPropertyDescription")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDescription", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (shader, propertyIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDescription_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetPropertyDescription")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDescription", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (propertyIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyFlags_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderPropertyFlags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                crate::UnityEngine::Rendering::ShaderPropertyFlags,
                2usize,
            >("GetPropertyFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyFlags", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderPropertyFlags = unsafe {
            method.invoke_unchecked((), (shader, propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyFlags_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderPropertyFlags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                crate::UnityEngine::Rendering::ShaderPropertyFlags,
                1usize,
            >("GetPropertyFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyFlags", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderPropertyFlags = unsafe {
            method.invoke_unchecked(self, (propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyNameId_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                i32,
                2usize,
            >("GetPropertyNameId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyNameId", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (shader, propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyNameId_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetPropertyNameId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyNameId", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyName_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetPropertyName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyName", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (shader, propertyIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyName_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetPropertyName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (propertyIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyRangeLimits(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                crate::UnityEngine::Vector2,
                1usize,
            >("GetPropertyRangeLimits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyRangeLimits", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, (propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyTextureDefaultName_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetPropertyTextureDefaultName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyTextureDefaultName", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (shader, propertyIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyTextureDefaultName_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetPropertyTextureDefaultName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyTextureDefaultName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (propertyIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyTextureDimension_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::TextureDimension> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                crate::UnityEngine::Rendering::TextureDimension,
                2usize,
            >("GetPropertyTextureDimension")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyTextureDimension", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::TextureDimension = unsafe {
            method.invoke_unchecked((), (shader, propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyTextureDimension_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::TextureDimension> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                crate::UnityEngine::Rendering::TextureDimension,
                1usize,
            >("GetPropertyTextureDimension")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyTextureDimension", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::TextureDimension = unsafe {
            method.invoke_unchecked(self, (propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyType_Shader_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderPropertyType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>, i32),
                crate::UnityEngine::Rendering::ShaderPropertyType,
                2usize,
            >("GetPropertyType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyType", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderPropertyType = unsafe {
            method.invoke_unchecked((), (shader, propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyType_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderPropertyType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                crate::UnityEngine::Rendering::ShaderPropertyType,
                1usize,
            >("GetPropertyType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyType", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderPropertyType = unsafe {
            method.invoke_unchecked(self, (propertyIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IDToTag(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("IDToTag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "IDToTag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_FindPassTagValue(
        &mut self,
        passIndex: i32,
        tagName: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("Internal_FindPassTagValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "Internal_FindPassTagValue", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (passIndex, tagName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_FindPassTagValueInSubShader(
        &mut self,
        subShaderIndex: i32,
        passIndex: i32,
        tagName: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                i32,
                3usize,
            >("Internal_FindPassTagValueInSubShader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "Internal_FindPassTagValueInSubShader", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (subShaderIndex, passIndex, tagName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_FindSubshaderTagValue(
        &mut self,
        subShaderIndex: i32,
        tagName: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("Internal_FindSubshaderTagValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "Internal_FindSubshaderTagValue", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (subShaderIndex, tagName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsKeywordEnabledFast(
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Rendering::GlobalKeyword),
                bool,
                1usize,
            >("IsKeywordEnabledFast")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "IsKeywordEnabledFast", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsKeywordEnabledFast_Injected(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Rendering::GlobalKeyword,
                >),
                bool,
                1usize,
            >("IsKeywordEnabledFast_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "IsKeywordEnabledFast_Injected", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsKeywordEnabled_ByRefMut1(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Rendering::GlobalKeyword,
                >),
                bool,
                1usize,
            >("IsKeywordEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "IsKeywordEnabled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsKeywordEnabled_Il2CppString0(
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsKeywordEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "IsKeywordEnabled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PropertyToID(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("PropertyToID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "PropertyToID", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBufferImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalBufferImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalBufferImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_Il2CppString_ComputeBuffer0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalBuffer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_Il2CppString_GraphicsBuffer2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalBuffer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_i32_ComputeBuffer1(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalBuffer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_i32_GraphicsBuffer3(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalBuffer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::Color,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalColor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_i32_1(
        nameID: i32,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::UnityEngine::Color),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalColor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBufferImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetGlobalConstantBufferImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalConstantBufferImpl", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_Il2CppString_ComputeBuffer0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetGlobalConstantBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalConstantBuffer", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_Il2CppString_GraphicsBuffer2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetGlobalConstantBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalConstantBuffer", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_i32_ComputeBuffer1(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetGlobalConstantBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalConstantBuffer", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_i32_GraphicsBuffer3(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetGlobalConstantBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalConstantBuffer", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantGraphicsBufferImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetGlobalConstantGraphicsBufferImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalConstantGraphicsBufferImpl", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArrayImpl(
        name: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalFloatArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloatArrayImpl", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Il2CppString_Il2CppArray3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloatArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Il2CppString_List_1_1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<f32>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloatArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_Il2CppArray4(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloatArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_Il2CppArray_i32_0(
        name: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloatArray", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_List_1_2(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<f32>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloatArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatImpl(
        name: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalFloatImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloatImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloat", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_i32_1(
        nameID: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalFloat", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalGraphicsBufferImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalGraphicsBufferImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalGraphicsBufferImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalIntImpl(
        name: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalIntImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalIntImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalInt", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_i32_1(
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalInt", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalInteger", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_i32_1(
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalInteger", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArrayImpl(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalMatrixArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrixArrayImpl", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Il2CppString_Il2CppArray3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrixArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Il2CppString_List_1_1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Matrix4x4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrixArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_Il2CppArray4(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrixArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_Il2CppArray_i32_0(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrixArray", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_List_1_2(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Matrix4x4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalMatrixArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrixArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixImpl(
        name: i32,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::UnityEngine::Matrix4x4),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalMatrixImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrixImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixImpl_Injected(
        name: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalMatrixImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrixImpl_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::Matrix4x4,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrix", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_i32_1(
        nameID: i32,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::UnityEngine::Matrix4x4),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalMatrix", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalRenderTextureImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    crate::UnityEngine::Rendering::RenderTextureSubElement,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalRenderTextureImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalRenderTextureImpl", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value, element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTextureImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalTextureImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalTextureImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Il2CppString_RenderTexture_RenderTextureSubElement2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    crate::UnityEngine::Rendering::RenderTextureSubElement,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalTexture", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value, element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Il2CppString_Texture0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalTexture", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_RenderTexture_RenderTextureSubElement3(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    crate::UnityEngine::Rendering::RenderTextureSubElement,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalTexture", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value, element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_Texture1(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalTexture", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArrayImpl(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalVectorArrayImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVectorArrayImpl", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Il2CppString_Il2CppArray3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVectorArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Il2CppString_List_1_1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Vector4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVectorArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_Il2CppArray4(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVectorArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_Il2CppArray_i32_0(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVectorArray", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, values, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_List_1_2(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Vector4,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVectorArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVectorArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorImpl(
        name: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::UnityEngine::Vector4),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVectorImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVectorImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorImpl_Injected(
        name: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVectorImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVectorImpl_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::Vector4,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVector", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_i32_1(
        nameID: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::UnityEngine::Vector4),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalVector", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (nameID, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::GlobalKeyword,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetKeyword")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetKeyword", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeywordFast(
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Rendering::GlobalKeyword, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetKeywordFast")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetKeywordFast", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeywordFast_Injected(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::GlobalKeyword,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetKeywordFast_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "SetKeywordFast_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (keyword, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TagToID(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("TagToID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "TagToID", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn WarmupAllShaders() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("WarmupAllShaders")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "WarmupAllShaders", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_disableBatching(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::DisableBatchingType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::DisableBatchingType,
                0usize,
            >("get_disableBatching")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_disableBatching", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::DisableBatchingType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_enabledGlobalKeywords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::UnityEngine::Rendering::GlobalKeyword,
                    >,
                >,
                0usize,
            >("get_enabledGlobalKeywords")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_enabledGlobalKeywords", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_globalKeywords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::UnityEngine::Rendering::GlobalKeyword,
                    >,
                >,
                0usize,
            >("get_globalKeywords")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_globalKeywords", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_globalMaximumLOD() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_globalMaximumLOD")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_globalMaximumLOD", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_globalRenderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_globalRenderPipeline")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_globalRenderPipeline", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_globalShaderHardwareTier() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderHardwareTier,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Rendering::ShaderHardwareTier,
                0usize,
            >("get_globalShaderHardwareTier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_globalShaderHardwareTier", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderHardwareTier = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_isSupported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_keywordSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::LocalKeywordSpace,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Rendering::LocalKeywordSpace,
                0usize,
            >("get_keywordSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_keywordSpace", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::LocalKeywordSpace = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_keywordSpace_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeywordSpace,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Rendering::LocalKeywordSpace,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_keywordSpace_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_keywordSpace_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_maximumChunksOverride() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_maximumChunksOverride")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_maximumChunksOverride", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maximumLOD(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_maximumLOD")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_maximumLOD", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_passCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_passCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_passCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_renderQueue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_renderQueue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_renderQueue", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_subshaderCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_subshaderCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "get_subshaderCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_globalMaximumLOD(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_globalMaximumLOD")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "set_globalMaximumLOD", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_globalRenderPipeline(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_globalRenderPipeline")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "set_globalRenderPipeline", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_globalShaderHardwareTier(
        value: crate::UnityEngine::Rendering::ShaderHardwareTier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Rendering::ShaderHardwareTier),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_globalShaderHardwareTier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "set_globalShaderHardwareTier", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_maximumChunksOverride(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_maximumChunksOverride")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "set_maximumChunksOverride", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_maximumLOD(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Shader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_maximumLOD")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Shader as quest_hook::libil2cpp::Type >
                    ::class(), "set_maximumLOD", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Shader")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Shader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
