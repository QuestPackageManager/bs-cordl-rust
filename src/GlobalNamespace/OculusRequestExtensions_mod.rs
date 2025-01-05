#[cfg(feature = "OculusRequestExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusRequestExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OculusRequestExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusRequestExtensions => ""
    ."OculusRequestExtensions"
);
#[cfg(feature = "OculusRequestExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OculusRequestExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusRequestExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusRequestExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusRequestExtensions")]
impl crate::GlobalNamespace::OculusRequestExtensions {
    pub fn ToPlatformException(
        error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
        errorType: crate::GlobalNamespace::PlatformException_ErrorType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToPlatformException", (error, errorType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusRequestExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusRequestExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
