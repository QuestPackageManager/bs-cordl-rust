#[cfg(feature = "MultiplayerBadgeDataPrecisionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeDataPrecisionSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerBadgeDataMinMaxFloatSO,
    >,
}
#[cfg(feature = "MultiplayerBadgeDataPrecisionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerBadgeDataPrecisionSO
    => ""."MultiplayerBadgeDataPrecisionSO"
);
#[cfg(feature = "MultiplayerBadgeDataPrecisionSO")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerBadgeDataPrecisionSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerBadgeDataMinMaxFloatSO,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataPrecisionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerBadgeDataPrecisionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataPrecisionSO")]
impl crate::GlobalNamespace::MultiplayerBadgeDataPrecisionSO {
    pub fn GetValue(
        &mut self,
        result: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerPlayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetValue", (result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerBadgeDataPrecisionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerBadgeDataPrecisionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
