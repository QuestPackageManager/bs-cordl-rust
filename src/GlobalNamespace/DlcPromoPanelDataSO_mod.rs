#[cfg(feature = "DlcPromoPanelDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct DlcPromoPanelDataSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _cutOffTest: i32,
    pub _minNumberOfNotOwnedPacks: i32,
    pub _defaultPromoInfoId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _dlcPromoPanelType: crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType,
    pub _customDlcPromoBanner: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PromoBannerInfoSO,
    >,
}
#[cfg(feature = "DlcPromoPanelDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DlcPromoPanelDataSO => ""
    ."DlcPromoPanelDataSO"
);
#[cfg(feature = "DlcPromoPanelDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::DlcPromoPanelDataSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DlcPromoPanelDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::DlcPromoPanelDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DlcPromoPanelDataSO")]
impl crate::GlobalNamespace::DlcPromoPanelDataSO {
    #[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
    pub type DlcPromoPanelType = crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType;
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
    pub fn get_customDlcPromoBanner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PromoBannerInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PromoBannerInfoSO,
        > = __cordl_object.invoke("get_customDlcPromoBanner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutOffTest(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cutOffTest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPromoInfoId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_defaultPromoInfoId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dlcPromoPanelType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType = __cordl_object
            .invoke("get_dlcPromoPanelType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minNumberOfNotOwnedPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_minNumberOfNotOwnedPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultPromoInfoId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultPromoInfoId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DlcPromoPanelDataSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DlcPromoPanelDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DlcPromoPanelDataSO_DlcPromoPanelType {
    Pack = 0i32,
    Store = 1i32,
}
#[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType => ""
    ."DlcPromoPanelDataSO/DlcPromoPanelType"
);
