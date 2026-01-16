#[cfg(feature = "cordl_class_HMUI+CurvedCanvasSettingsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CurvedCanvasSettingsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cachedCanvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    pub _cachedCanvasIsRootCanvas: bool,
    pub _curvedCanvasSettings: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings>,
    pub _hasCachedData: bool,
}
#[cfg(feature = "cordl_class_HMUI+CurvedCanvasSettingsHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::CurvedCanvasSettingsHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "CurvedCanvasSettingsHelper";
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
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl std::ops::Deref for crate::HMUI::CurvedCanvasSettingsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl std::ops::DerefMut for crate::HMUI::CurvedCanvasSettingsHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl crate::HMUI::CurvedCanvasSettingsHelper {
    pub fn GetCurvedCanvasSettings(
        &mut self,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>),
                        quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings>,
                        1usize,
                    >("GetCurvedCanvasSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCurvedCanvasSettings", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings> =
            unsafe { cordl_method_info.invoke_unchecked(self, (canvas))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCurvedCanvasSettingsForCanvas(
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>),
                        quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings>,
                        1usize,
                    >("GetCurvedCanvasSettingsForCanvas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCurvedCanvasSettingsForCanvas", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings> =
            unsafe { cordl_method_info.invoke_unchecked((), (canvas))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_HMUI+CurvedCanvasSettingsHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::CurvedCanvasSettingsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
