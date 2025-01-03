#[cfg(feature = "Oculus+Platform+Models+NetSyncVoipAttenuationValueList")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSyncVoipAttenuationValueList {
    __cordl_parent: crate::Oculus::Platform::Models::DeserializableList_1<
        *mut crate::Oculus::Platform::Models::NetSyncVoipAttenuationValue,
    >,
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncVoipAttenuationValueList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::Models::NetSyncVoipAttenuationValueList =>
    "Oculus.Platform.Models"."NetSyncVoipAttenuationValueList"
);
#[cfg(feature = "Oculus+Platform+Models+NetSyncVoipAttenuationValueList")]
impl std::ops::Deref
for crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList {
    type Target = crate::Oculus::Platform::Models::DeserializableList_1<
        *mut crate::Oculus::Platform::Models::NetSyncVoipAttenuationValue,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncVoipAttenuationValueList")]
impl std::ops::DerefMut
for crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncVoipAttenuationValueList")]
impl crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList {
    pub fn New(
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncVoipAttenuationValueList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
