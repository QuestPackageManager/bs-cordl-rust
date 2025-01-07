#[cfg(feature = "SonyPublishingHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyPublishingHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SonyPublishingHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SonyPublishingHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SonyPublishingHelper";
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
#[cfg(feature = "SonyPublishingHelper")]
impl std::ops::Deref for crate::GlobalNamespace::SonyPublishingHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyPublishingHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyPublishingHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyPublishingHelper")]
impl crate::GlobalNamespace::SonyPublishingHelper {
    pub fn GetContentIdFromNpTitleId(
        serviceIdPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        npTitleId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        productLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetContentIdFromNpTitleId",
                (serviceIdPrefix, npTitleId, productLabel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContentIdFromTitleId(
        serviceIdPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        titleId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        productLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetContentIdFromTitleId",
                (serviceIdPrefix, titleId, productLabel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContentId_Il2CppString_Il2CppString1(
        serviceId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        productLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetContentId", (serviceId, productLabel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContentId_SonyPublisherSKUSettingsSO0(
        sonyPublisherSKUSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyPublisherSKUSettingsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetContentId", (sonyPublisherSKUSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNpTitleId(
        titleId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNpTitleId", (titleId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServiceId(
        sonyPublisherSKUSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyPublisherSKUSettingsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetServiceId", (sonyPublisherSKUSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServiceIdFromNpTitleId(
        serviceIdPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        npTitleId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetServiceIdFromNpTitleId", (serviceIdPrefix, npTitleId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServiceIdFromTitleId(
        serviceIdPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        titleId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetServiceIdFromTitleId", (serviceIdPrefix, titleId))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyPublishingHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SonyPublishingHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
