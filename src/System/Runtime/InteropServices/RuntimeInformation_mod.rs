#[cfg(feature = "System+Runtime+InteropServices+RuntimeInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeInformation {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+InteropServices+RuntimeInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::RuntimeInformation =>
    "System.Runtime.InteropServices"."RuntimeInformation"
);
#[cfg(feature = "System+Runtime+InteropServices+RuntimeInformation")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::RuntimeInformation {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+RuntimeInformation")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::RuntimeInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+RuntimeInformation")]
impl crate::System::Runtime::InteropServices::RuntimeInformation {
    pub fn GetOSName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetOSName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRuntimeArchitecture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRuntimeArchitecture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOSPlatform(
        osPlatform: crate::System::Runtime::InteropServices::OSPlatform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOSPlatform", (osPlatform))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+RuntimeInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::RuntimeInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
