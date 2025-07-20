#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlatformAdditionalContentModelInitialDataSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _levelsEntitlements: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockPlatformEntitlement>,
        >,
    >,
    pub _levelPacksEntitlements: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockPlatformEntitlement>,
        >,
    >,
    pub _packBetterBuyThanLevel: bool,
}
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockPlatformAdditionalContentModelInitialDataSO";
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
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
impl std::ops::Deref
for crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
impl crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_levelPacksEntitlements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MockPlatformEntitlement,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MockPlatformEntitlement,
                        >,
                    >,
                >,
                0usize,
            >("get_levelPacksEntitlements")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_levelPacksEntitlements", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MockPlatformEntitlement,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_levelsEntitlements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MockPlatformEntitlement,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MockPlatformEntitlement,
                        >,
                    >,
                >,
                0usize,
            >("get_levelsEntitlements")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO as
                    quest_hook::libil2cpp::Type > ::class(), "get_levelsEntitlements",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MockPlatformEntitlement,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_packBetterBuyThanLevel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_packBetterBuyThanLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_packBetterBuyThanLevel", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModelInitialDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
