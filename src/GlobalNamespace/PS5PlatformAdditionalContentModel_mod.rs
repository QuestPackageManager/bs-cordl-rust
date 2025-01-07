#[cfg(feature = "PS5PlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5PlatformAdditionalContentModel {
    __cordl_parent: crate::GlobalNamespace::SonyPlatformAdditionalContentModel,
}
#[cfg(feature = "PS5PlatformAdditionalContentModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PS5PlatformAdditionalContentModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PS5PlatformAdditionalContentModel";
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
#[cfg(feature = "PS5PlatformAdditionalContentModel")]
impl std::ops::Deref for crate::GlobalNamespace::PS5PlatformAdditionalContentModel {
    type Target = crate::GlobalNamespace::SonyPlatformAdditionalContentModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5PlatformAdditionalContentModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS5PlatformAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5PlatformAdditionalContentModel")]
impl crate::GlobalNamespace::PS5PlatformAdditionalContentModel {
    pub fn New(
        sonyCommerceHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper,
        >,
        sonyLevelProductCollectionModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (sonyCommerceHelper, sonyLevelProductCollectionModel),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        sonyCommerceHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper,
        >,
        sonyLevelProductCollectionModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sonyCommerceHelper, sonyLevelProductCollectionModel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PS5PlatformAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS5PlatformAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
