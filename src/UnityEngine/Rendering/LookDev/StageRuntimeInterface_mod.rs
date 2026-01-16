#[cfg(feature = "cordl_class_UnityEngine+Rendering+LookDev+StageRuntimeInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct StageRuntimeInterface {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AddGameObject: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<bool, quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>>,
    >,
    pub m_GetCamera: quest_hook::libil2cpp::Gc<
        crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>>,
    >,
    pub m_GetSunLight: quest_hook::libil2cpp::Gc<
        crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>>,
    >,
    pub SRPData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LookDev+StageRuntimeInterface")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::LookDev::StageRuntimeInterface
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.LookDev";
    const CLASS_NAME: &'static str = "StageRuntimeInterface";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+LookDev+StageRuntimeInterface")]
impl std::ops::Deref for crate::UnityEngine::Rendering::LookDev::StageRuntimeInterface {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LookDev+StageRuntimeInterface")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::LookDev::StageRuntimeInterface {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LookDev+StageRuntimeInterface")]
impl crate::UnityEngine::Rendering::LookDev::StageRuntimeInterface {
    pub fn AddGameObject(
        &mut self,
        persistent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        1usize,
                    >("AddGameObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddGameObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (persistent))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        AddGameObject: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<bool, quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>>,
        >,
        GetCamera: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>>,
        >,
        GetSunLight: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (AddGameObject, GetCamera, GetSunLight))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        AddGameObject: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<bool, quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>>,
        >,
        GetCamera: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>>,
        >,
        GetSunLight: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<
                                bool,
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (AddGameObject, GetCamera, GetSunLight))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_camera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        0usize,
                    >("get_camera")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_camera", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sunLight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                        0usize,
                    >("get_sunLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_sunLight", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LookDev+StageRuntimeInterface")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::LookDev::StageRuntimeInterface
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
