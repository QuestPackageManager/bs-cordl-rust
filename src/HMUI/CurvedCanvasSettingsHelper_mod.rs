#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CurvedCanvasSettingsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cachedCanvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    pub _cachedCanvasIsRootCanvas: bool,
    pub _curvedCanvasSettings: quest_hook::libil2cpp::Gc<
        crate::HMUI::CurvedCanvasSettings,
    >,
    pub _hasCachedData: bool,
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::CurvedCanvasSettingsHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "CurvedCanvasSettingsHelper";
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
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl std::ops::Deref for crate::HMUI::CurvedCanvasSettingsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl std::ops::DerefMut for crate::HMUI::CurvedCanvasSettingsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl crate::HMUI::CurvedCanvasSettingsHelper {
    pub fn GetCurvedCanvasSettings(
        &mut self,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings> = __cordl_object
            .invoke("GetCurvedCanvasSettings", (canvas))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurvedCanvasSettingsForCanvas(
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedCanvasSettings> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurvedCanvasSettingsForCanvas", (canvas))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::CurvedCanvasSettingsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
