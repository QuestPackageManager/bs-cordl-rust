#[cfg(feature = "OffsetDirectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OffsetDirectionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "OffsetDirectionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OffsetDirectionExtensions => ""
    ."OffsetDirectionExtensions"
);
#[cfg(feature = "OffsetDirectionExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OffsetDirectionExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OffsetDirectionExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OffsetDirectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OffsetDirectionExtensions")]
impl crate::GlobalNamespace::OffsetDirectionExtensions {
    pub fn OppositeDirection(
        offsetDirection: crate::GlobalNamespace::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OffsetDirection> {
        let __cordl_ret: crate::GlobalNamespace::OffsetDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OppositeDirection", (offsetDirection))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OffsetDirectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OffsetDirectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
