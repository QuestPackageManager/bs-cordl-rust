#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ManagedStreamHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ManagedStreamHelpers =>
    "UnityEngine"."ManagedStreamHelpers"
);
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
impl std::ops::Deref for crate::UnityEngine::ManagedStreamHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::ManagedStreamHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
impl crate::UnityEngine::ManagedStreamHelpers {
    pub fn ManagedStreamLength(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        returnValueAddress: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ManagedStreamLength", (stream, returnValueAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn ManagedStreamRead(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        returnValueAddress: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ManagedStreamRead",
                (buffer, offset, count, stream, returnValueAddress),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ManagedStreamSeek(
        offset: i64,
        origin: u32,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        returnValueAddress: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ManagedStreamSeek", (offset, origin, stream, returnValueAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateLoadFromStream(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateLoadFromStream", (stream))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ManagedStreamHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
