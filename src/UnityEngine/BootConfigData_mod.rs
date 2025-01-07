#[cfg(feature = "UnityEngine+BootConfigData")]
#[repr(C)]
#[derive(Debug)]
pub struct BootConfigData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+BootConfigData")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::BootConfigData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "BootConfigData";
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
#[cfg(feature = "UnityEngine+BootConfigData")]
impl std::ops::Deref for crate::UnityEngine::BootConfigData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+BootConfigData")]
impl std::ops::DerefMut for crate::UnityEngine::BootConfigData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+BootConfigData")]
impl crate::UnityEngine::BootConfigData {
    pub fn New(
        nativeHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nativeHandle))?;
        Ok(__cordl_object.into())
    }
    pub fn WrapBootConfigData(
        nativeHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::BootConfigData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::BootConfigData> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WrapBootConfigData", (nativeHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        nativeHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nativeHandle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+BootConfigData")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::BootConfigData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
