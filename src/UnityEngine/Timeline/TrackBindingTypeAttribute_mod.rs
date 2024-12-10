#[cfg(feature = "UnityEngine+Timeline+TrackBindingTypeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackBindingTypeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _cordl_type: *mut crate::System::Type,
    pub flags: crate::UnityEngine::Timeline::TrackBindingFlags,
}
#[cfg(feature = "UnityEngine+Timeline+TrackBindingTypeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TrackBindingTypeAttribute
    => "UnityEngine.Timeline"."TrackBindingTypeAttribute"
);
#[cfg(feature = "UnityEngine+Timeline+TrackBindingTypeAttribute")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackBindingTypeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackBindingTypeAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TrackBindingTypeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackBindingTypeAttribute")]
impl crate::UnityEngine::Timeline::TrackBindingTypeAttribute {
    pub fn New_TrackBindingFlags1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        flags: crate::UnityEngine::Timeline::TrackBindingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_TrackBindingFlags1(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        flags: crate::UnityEngine::Timeline::TrackBindingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackBindingTypeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TrackBindingTypeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
