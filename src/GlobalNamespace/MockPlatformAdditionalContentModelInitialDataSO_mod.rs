#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlatformAdditionalContentModelInitialDataSO {
    __cordl_parent: PersistentScriptableObject,
    pub _levelsEntitlements: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MockPlatformEntitlement,
    >,
    pub _levelPacksEntitlements: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MockPlatformEntitlement,
    >,
    pub _packBetterBuyThanLevel: bool,
}
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlatformAdditionalContentModelInitialDataSO => ""
    ."MockPlatformAdditionalContentModelInitialDataSO"
);
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
impl std::ops::Deref for MockPlatformAdditionalContentModelInitialDataSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
impl std::ops::DerefMut for MockPlatformAdditionalContentModelInitialDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
impl MockPlatformAdditionalContentModelInitialDataSO {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelPacksEntitlements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MockPlatformEntitlement,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MockPlatformEntitlement,
        > = __cordl_object.invoke("get_levelPacksEntitlements", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelsEntitlements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MockPlatformEntitlement,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MockPlatformEntitlement,
        > = __cordl_object.invoke("get_levelsEntitlements", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packBetterBuyThanLevel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_packBetterBuyThanLevel", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
impl quest_hook::libil2cpp::ObjectType
for MockPlatformAdditionalContentModelInitialDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
