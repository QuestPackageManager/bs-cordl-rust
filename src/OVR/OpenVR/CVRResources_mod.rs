#[cfg(feature = "OVR+OpenVR+CVRResources")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRResources {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRResources,
}
#[cfg(feature = "OVR+OpenVR+CVRResources")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRResources => "OVR.OpenVR"
    ."CVRResources"
);
#[cfg(feature = "OVR+OpenVR+CVRResources")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRResources {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRResources")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRResources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRResources")]
impl crate::OVR::OpenVR::CVRResources {
    pub fn GetResourceFullPath(
        &mut self,
        pchResourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchResourceTypeDirectory: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchPathBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetResourceFullPath",
                (pchResourceName, pchResourceTypeDirectory, pchPathBuffer, unBufferLen),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSharedResource(
        &mut self,
        pchResourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("LoadSharedResource", (pchResourceName, pchBuffer, unBufferLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pInterface))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRResources")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRResources {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
