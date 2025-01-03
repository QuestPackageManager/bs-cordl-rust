#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRRenderModels {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRRenderModels,
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRRenderModels => "OVR.OpenVR"
    ."CVRRenderModels"
);
#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRRenderModels {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRRenderModels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
impl crate::OVR::OpenVR::CVRRenderModels {
    #[cfg(feature = "OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
    pub type GetComponentStateUnion = crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion;
    #[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
    pub type _GetComponentStatePacked = crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked;
    pub fn FreeRenderModel(
        &mut self,
        pRenderModel: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeRenderModel", (pRenderModel))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeTexture(
        &mut self,
        pTexture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeTexture", (pTexture))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeTextureD3D11(
        &mut self,
        pD3D11Texture2D: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeTextureD3D11", (pD3D11Texture2D))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentButtonMask(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object
            .invoke("GetComponentButtonMask", (pchRenderModelName, pchComponentName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentCount(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetComponentCount", (pchRenderModelName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentName(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        unComponentIndex: u32,
        pchComponentName: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unComponentNameLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetComponentName",
                (
                    pchRenderModelName,
                    unComponentIndex,
                    pchComponentName,
                    unComponentNameLen,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentRenderModelName(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentRenderModelName: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unComponentRenderModelNameLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetComponentRenderModelName",
                (
                    pchRenderModelName,
                    pchComponentName,
                    pchComponentRenderModelName,
                    unComponentRenderModelNameLen,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentState(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t,
        >,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetComponentState",
                (
                    pchRenderModelName,
                    pchComponentName,
                    pControllerState,
                    pState,
                    pComponentState,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentStateForDevicePath(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        devicePath: u64,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetComponentStateForDevicePath",
                (
                    pchRenderModelName,
                    pchComponentName,
                    devicePath,
                    pState,
                    pComponentState,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetRenderModelCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelErrorNameFromEnum(
        &mut self,
        error: crate::OVR::OpenVR::EVRRenderModelError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetRenderModelErrorNameFromEnum", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelName(
        &mut self,
        unRenderModelIndex: u32,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unRenderModelNameLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetRenderModelName",
                (unRenderModelIndex, pchRenderModelName, unRenderModelNameLen),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelOriginalPath(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchOriginalPath: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unOriginalPathLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRRenderModelError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetRenderModelOriginalPath",
                (pchRenderModelName, pchOriginalPath, unOriginalPathLen, peError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelThumbnailURL(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchThumbnailURL: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unThumbnailURLLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRRenderModelError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetRenderModelThumbnailURL",
                (pchRenderModelName, pchThumbnailURL, unThumbnailURLLen, peError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadIntoTextureD3D11_Async(
        &mut self,
        textureId: i32,
        pDstTexture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRRenderModelError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRRenderModelError = __cordl_object
            .invoke("LoadIntoTextureD3D11_Async", (textureId, pDstTexture))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRenderModel_Async(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        ppRenderModel: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRRenderModelError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRRenderModelError = __cordl_object
            .invoke("LoadRenderModel_Async", (pchRenderModelName, ppRenderModel))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTextureD3D11_Async(
        &mut self,
        textureId: i32,
        pD3D11Device: crate::System::IntPtr,
        ppD3D11Texture2D: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRRenderModelError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRRenderModelError = __cordl_object
            .invoke(
                "LoadTextureD3D11_Async",
                (textureId, pD3D11Device, ppD3D11Texture2D),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTexture_Async(
        &mut self,
        textureId: i32,
        ppTexture: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRRenderModelError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRRenderModelError = __cordl_object
            .invoke("LoadTexture_Async", (textureId, ppTexture))?;
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
    pub fn RenderModelHasComponent(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RenderModelHasComponent", (pchRenderModelName, pchComponentName))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRRenderModels {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CVRRenderModels_GetComponentStateUnion {
    padding: [u8; 8usize],
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion => "OVR.OpenVR"
    ."CVRRenderModels/GetComponentStateUnion"
);
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
impl crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion {}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRRenderModels__GetComponentStatePacked {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked => "OVR.OpenVR"
    ."CVRRenderModels/_GetComponentStatePacked"
);
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
impl crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked {
    pub fn BeginInvoke(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pchRenderModelName,
                    pchComponentName,
                    pControllerState,
                    pState,
                    pComponentState,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pControllerState, pState, pComponentState, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Invoke",
                (
                    pchRenderModelName,
                    pchComponentName,
                    pControllerState,
                    pState,
                    pComponentState,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
