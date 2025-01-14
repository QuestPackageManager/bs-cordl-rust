#[cfg(feature = "MultiplayerBadgeDataGoodCutsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeDataGoodCutsSO {
    __cordl_parent: crate::GlobalNamespace::MultiplayerBadgeDataMinMaxIntSO,
}
#[cfg(feature = "MultiplayerBadgeDataGoodCutsSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerBadgeDataGoodCutsSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerBadgeDataGoodCutsSO";
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
#[cfg(feature = "MultiplayerBadgeDataGoodCutsSO")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerBadgeDataGoodCutsSO {
    type Target = crate::GlobalNamespace::MultiplayerBadgeDataMinMaxIntSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataGoodCutsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerBadgeDataGoodCutsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataGoodCutsSO")]
impl crate::GlobalNamespace::MultiplayerBadgeDataGoodCutsSO {
    pub fn GetValue(
        &mut self,
        result: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerPlayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >),
                i32,
                1usize,
            >("GetValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetValue", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (result)) };
        Ok(__cordl_ret.into())
    }
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerBadgeDataGoodCutsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerBadgeDataGoodCutsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
