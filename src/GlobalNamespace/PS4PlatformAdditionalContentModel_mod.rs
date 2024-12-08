#[cfg(feature = "PS4PlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PS4PlatformAdditionalContentModel {
    __cordl_parent: SonyPlatformAdditionalContentModel,
}
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PS4PlatformAdditionalContentModel => ""
    ."PS4PlatformAdditionalContentModel"
);
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
impl std::ops::Deref for PS4PlatformAdditionalContentModel {
    type Target = SonyPlatformAdditionalContentModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
impl std::ops::DerefMut for PS4PlatformAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
impl PS4PlatformAdditionalContentModel {
    pub fn _ctor(
        &mut self,
        sonyCommerceHelper: *mut ISonyCommerceHelper,
        sonyLevelProductCollectionModel: *mut SonyLevelProductCollectionModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sonyCommerceHelper, sonyLevelProductCollectionModel))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sonyCommerceHelper: *mut ISonyCommerceHelper,
        sonyLevelProductCollectionModel: *mut SonyLevelProductCollectionModel,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (sonyCommerceHelper, sonyLevelProductCollectionModel),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType for PS4PlatformAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
