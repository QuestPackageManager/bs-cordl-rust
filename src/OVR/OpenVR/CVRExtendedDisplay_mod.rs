#[cfg(feature = "OVR+OpenVR+CVRExtendedDisplay")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRExtendedDisplay {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRExtendedDisplay,
}
#[cfg(feature = "OVR+OpenVR+CVRExtendedDisplay")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRExtendedDisplay => "OVR.OpenVR"
    ."CVRExtendedDisplay"
);
#[cfg(feature = "OVR+OpenVR+CVRExtendedDisplay")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRExtendedDisplay {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRExtendedDisplay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRExtendedDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRExtendedDisplay")]
impl crate::OVR::OpenVR::CVRExtendedDisplay {
    pub fn GetDXGIOutputInfo(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        pnAdapterOutputIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDXGIOutputInfo", (pnAdapterIndex, pnAdapterOutputIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeOutputViewport(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pnX: quest_hook::libil2cpp::ByRefMut<u32>,
        pnY: quest_hook::libil2cpp::ByRefMut<u32>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetEyeOutputViewport", (eEye, pnX, pnY, pnWidth, pnHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindowBounds(
        &mut self,
        pnX: quest_hook::libil2cpp::ByRefMut<i32>,
        pnY: quest_hook::libil2cpp::ByRefMut<i32>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetWindowBounds", (pnX, pnY, pnWidth, pnHeight))?;
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
#[cfg(feature = "OVR+OpenVR+CVRExtendedDisplay")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRExtendedDisplay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
