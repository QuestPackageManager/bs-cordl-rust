#[cfg(feature = "UnityEngine+Internal+InputUnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct InputUnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Internal+InputUnsafeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Internal::InputUnsafeUtility =>
    "UnityEngine.Internal"."InputUnsafeUtility"
);
#[cfg(feature = "UnityEngine+Internal+InputUnsafeUtility")]
impl std::ops::Deref for crate::UnityEngine::Internal::InputUnsafeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Internal+InputUnsafeUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Internal::InputUnsafeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Internal+InputUnsafeUtility")]
impl crate::UnityEngine::Internal::InputUnsafeUtility {
    pub fn GetAxis(
        axisName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAxis", (axisName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxisRaw(
        axisName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAxisRaw", (axisName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxisRaw__Unmanaged(
        axisName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        axisNameLen: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAxisRaw__Unmanaged", (axisName, axisNameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxis__Unmanaged(
        axisName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        axisNameLen: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAxis__Unmanaged", (axisName, axisNameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButton(
        buttonName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetButton", (buttonName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonDown(
        buttonName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetButtonDown", (buttonName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonDown__Unmanaged(
        buttonName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buttonNameLen: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetButtonDown__Unmanaged", (buttonName, buttonNameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonUp__Unmanaged(
        buttonName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buttonNameLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetButtonUp__Unmanaged", (buttonName, buttonNameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButton__Unmanaged(
        buttonName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buttonNameLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetButton__Unmanaged", (buttonName, buttonNameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyDownString__Unmanaged(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyDownString__Unmanaged", (name, nameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyString__Unmanaged(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyString__Unmanaged", (name, nameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyUpString__Unmanaged(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyUpString__Unmanaged", (name, nameLen))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Internal+InputUnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Internal::InputUnsafeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
