#[cfg(feature = "Oculus+Platform+VoipPCMSourceNative")]
#[repr(C)]
#[derive(Debug)]
pub struct VoipPCMSourceNative {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub senderID: u64,
}
#[cfg(feature = "Oculus+Platform+VoipPCMSourceNative")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::VoipPCMSourceNative =>
    "Oculus.Platform"."VoipPCMSourceNative"
);
#[cfg(feature = "Oculus+Platform+VoipPCMSourceNative")]
impl std::ops::Deref for crate::Oculus::Platform::VoipPCMSourceNative {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+VoipPCMSourceNative")]
impl std::ops::DerefMut for crate::Oculus::Platform::VoipPCMSourceNative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+VoipPCMSourceNative")]
impl crate::Oculus::Platform::VoipPCMSourceNative {
    pub fn GetPCM(
        &mut self,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPCM", (dest, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PeekSizeElements(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("PeekSizeElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSenderID(
        &mut self,
        senderID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSenderID", (senderID))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Oculus+Platform+VoipPCMSourceNative")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::VoipPCMSourceNative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+VoipPCMSourceNative")]
impl AsRef<crate::Oculus::Platform::IVoipPCMSource>
for crate::Oculus::Platform::VoipPCMSourceNative {
    fn as_ref(&self) -> &crate::Oculus::Platform::IVoipPCMSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Oculus+Platform+VoipPCMSourceNative")]
impl AsMut<crate::Oculus::Platform::IVoipPCMSource>
for crate::Oculus::Platform::VoipPCMSourceNative {
    fn as_mut(&mut self) -> &mut crate::Oculus::Platform::IVoipPCMSource {
        unsafe { std::mem::transmute(self) }
    }
}
