#[cfg(feature = "OVR+OpenVR+CVRChaperone")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRChaperone {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVRChaperone,
}
#[cfg(feature = "OVR+OpenVR+CVRChaperone")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRChaperone => "OVR.OpenVR"
    ."CVRChaperone"
);
#[cfg(feature = "OVR+OpenVR+CVRChaperone")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRChaperone {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRChaperone")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRChaperone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRChaperone")]
impl crate::OVR::OpenVR::CVRChaperone {
    pub fn ReloadInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSceneColor(
        &mut self,
        color: crate::OVR::OpenVR::HmdColor_t,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSceneColor", (color))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetBoundsColor(
        &mut self,
        pOutputColorArray: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdColor_t,
        >,
        nNumOutputColors: i32,
        flCollisionBoundsFadeDistance: f32,
        pOutputCameraColor: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdColor_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetBoundsColor",
                (
                    pOutputColorArray,
                    nNumOutputColors,
                    flCollisionBoundsFadeDistance,
                    pOutputCameraColor,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetCalibrationState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ChaperoneCalibrationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ChaperoneCalibrationState = __cordl_object
            .invoke("GetCalibrationState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayAreaSize(
        &mut self,
        pSizeX: quest_hook::libil2cpp::ByRefMut<f32>,
        pSizeZ: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetPlayAreaSize", (pSizeX, pSizeZ))?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayAreaRect(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdQuad_t>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetPlayAreaRect", (rect))?;
        Ok(__cordl_ret)
    }
    pub fn ForceBoundsVisible(
        &mut self,
        bForce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceBoundsVisible", (bForce))?;
        Ok(__cordl_ret)
    }
    pub fn AreBoundsVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreBoundsVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRChaperone")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRChaperone {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
