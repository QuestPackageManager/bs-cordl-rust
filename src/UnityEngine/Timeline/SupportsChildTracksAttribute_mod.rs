#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SupportsChildTracksAttribute {
    __cordl_parent: crate::System::Attribute,
    pub childType: *mut crate::System::Type,
    pub levels: i32,
}
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::SupportsChildTracksAttribute => "UnityEngine.Timeline"
    ."SupportsChildTracksAttribute"
);
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
impl std::ops::Deref for crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
impl crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    pub fn New(
        childType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        levels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (childType, levels))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        childType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        levels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (childType, levels))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+SupportsChildTracksAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::SupportsChildTracksAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
