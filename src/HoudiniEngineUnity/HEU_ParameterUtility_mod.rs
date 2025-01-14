#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ParameterUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_ParameterUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ParameterUtility";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ParameterUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ParameterUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterUtility")]
impl crate::HoudiniEngineUnity::HEU_ParameterUtility {
    pub fn FindTextureParamByNameOrTag(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_ParmInfo>,
        >,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useTextureParmName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::HoudiniEngineUnity::HAPI_ParmInfo,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                i32,
                5usize,
            >("FindTextureParamByNameOrTag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindTextureParamByNameOrTag", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (session, nodeID, parameters, parameterName, useTextureParmName),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetChoice(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        outValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                3usize,
            >("GetChoice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetChoice", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, outValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetColor(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        getValue: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
                ),
                bool,
                3usize,
            >("GetColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetColor", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, getValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFloat(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        outValue: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                3usize,
            >("GetFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFloat", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, outValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFloats(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        outValues: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<f32>,
                        >,
                    >,
                ),
                bool,
                3usize,
            >("GetFloats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFloats", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, outValues))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInputNode(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        obj: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    >,
                ),
                bool,
                4usize,
            >("GetInputNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInputNode", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, index, obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInt(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        outValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                3usize,
            >("GetInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInt", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, outValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterColor3Value(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_ParmInfo>,
        >,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultValue: crate::UnityEngine::Color,
        outputColor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::HoudiniEngineUnity::HAPI_ParmInfo,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::Color,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
                ),
                bool,
                6usize,
            >("GetParameterColor3Value")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetParameterColor3Value", 6usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        session,
                        nodeID,
                        parameters,
                        parameterName,
                        defaultValue,
                        outputColor,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterFloatValue(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_ParmInfo>,
        >,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultValue: f32,
        returnValue: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::HoudiniEngineUnity::HAPI_ParmInfo,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    f32,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                6usize,
            >("GetParameterFloatValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetParameterFloatValue", 6usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        session,
                        nodeID,
                        parameters,
                        parameterName,
                        defaultValue,
                        returnValue,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterIndexFromName(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_ParmInfo>,
        >,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::HoudiniEngineUnity::HAPI_ParmInfo,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                i32,
                3usize,
            >("GetParameterIndexFromName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetParameterIndexFromName", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (session, parameters, parameterName))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterIndexFromNameOrTag(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_ParmInfo>,
        >,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::HoudiniEngineUnity::HAPI_ParmInfo,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                i32,
                4usize,
            >("GetParameterIndexFromNameOrTag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetParameterIndexFromNameOrTag", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (session, nodeID, parameters, parameterName))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        outValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                3usize,
            >("GetString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetString", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, outValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetToggle(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        outValue: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                3usize,
            >("GetToggle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetToggle", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, outValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetChoice(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setValue: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                ),
                bool,
                3usize,
            >("SetChoice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetChoice", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, setValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetColor(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setValue: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::Color,
                ),
                bool,
                3usize,
            >("SetColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetColor", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, setValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFloat(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setValue: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    f32,
                ),
                bool,
                3usize,
            >("SetFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetFloat", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, setValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFloats(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                ),
                bool,
                3usize,
            >("SetFloats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetFloats", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, setValues))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInputNode(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    i32,
                ),
                bool,
                4usize,
            >("SetInputNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetInputNode", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInt(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setValue: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                ),
                bool,
                3usize,
            >("SetInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetInt", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, setValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetString(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                bool,
                3usize,
            >("SetString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetString", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, setValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetToggle(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setValue: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                bool,
                3usize,
            >("SetToggle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetToggle", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (asset, paramName, setValue))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_ParameterUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
