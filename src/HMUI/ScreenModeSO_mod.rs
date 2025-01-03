#[cfg(feature = "HMUI+ScreenModeSO")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenModeSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _data: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenModeData>,
}
#[cfg(feature = "HMUI+ScreenModeSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ScreenModeSO => "HMUI"."ScreenModeSO"
);
#[cfg(feature = "HMUI+ScreenModeSO")]
impl std::ops::Deref for crate::HMUI::ScreenModeSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScreenModeSO")]
impl std::ops::DerefMut for crate::HMUI::ScreenModeSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScreenModeSO")]
impl crate::HMUI::ScreenModeSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ScreenModeData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenModeData> = __cordl_object
            .invoke("get_data", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ScreenModeSO")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ScreenModeSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
