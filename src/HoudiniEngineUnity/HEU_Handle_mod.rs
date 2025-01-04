#[cfg(feature = "HoudiniEngineUnity+HEU_Handle")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Handle {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _handleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _handleType: crate::HoudiniEngineUnity::HEU_Handle_HEU_HandleType,
    pub _handleIndex: i32,
    pub _handleParamTranslateBinding: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_HandleParamBinding,
    >,
    pub _handleParamRotateBinding: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_HandleParamBinding,
    >,
    pub _handleParamScaleBinding: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_HandleParamBinding,
    >,
    pub _handlePosition: crate::UnityEngine::Vector3,
    pub _handleRotation: crate::UnityEngine::Quaternion,
    pub _handleScale: crate::UnityEngine::Vector3,
    pub _rstOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
    pub _xyzOrder: crate::HoudiniEngineUnity::HAPI_XYZOrder,
    pub _convertedTransformEuler: crate::HoudiniEngineUnity::HAPI_TransformEuler,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Handle =>
    "HoudiniEngineUnity"."HEU_Handle"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Handle {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_Handle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle")]
impl crate::HoudiniEngineUnity::HEU_Handle {
    #[cfg(feature = "HoudiniEngineUnity+HEU_Handle+HEU_HandleType")]
    pub type HEU_HandleType = crate::HoudiniEngineUnity::HEU_Handle_HEU_HandleType;
    pub fn CleanUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTransform(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        parameters: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Parameters>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTransform", (session, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRotateBinding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HandleParamBinding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HandleParamBinding,
        > = __cordl_object.invoke("GetRotateBinding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScaleBinding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HandleParamBinding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HandleParamBinding,
        > = __cordl_object.invoke("GetScaleBinding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTranslateBinding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HandleParamBinding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HandleParamBinding,
        > = __cordl_object.invoke("GetTranslateBinding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUpdatedPosition(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        inPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetUpdatedPosition", (asset, inPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUpdatedRotation(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        inRotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetUpdatedRotation", (asset, inRotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasRotateHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasRotateHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasScaleHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasScaleHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasTranslateHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTranslateHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Handle>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRotateHandleDisabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsRotateHandleDisabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsScaleHandleDisabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsScaleHandleDisabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSpecialRSTOrder(
        rstOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSpecialRSTOrder", (rstOrder))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTranslateHandleDisabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTranslateHandleDisabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetupHandle(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetID: i32,
        handleIndex: i32,
        handleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        handleType: crate::HoudiniEngineUnity::HEU_Handle_HEU_HandleType,
        handleInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_HandleInfo,
        >,
        parameters: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Parameters>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetupHandle",
                (
                    session,
                    assetID,
                    handleIndex,
                    handleName,
                    handleType,
                    handleInfo,
                    parameters,
                ),
            )?;
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
    pub fn get_ConvertedTransformEuler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_TransformEuler> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_TransformEuler = __cordl_object
            .invoke("get_ConvertedTransformEuler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandleName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_HandleName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandlePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_HandlePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandleRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_HandleRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandleScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_HandleScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandleType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_Handle_HEU_HandleType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_Handle_HEU_HandleType = __cordl_object
            .invoke("get_HandleType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RSTOrder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_RSTOrder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_RSTOrder = __cordl_object
            .invoke("get_RSTOrder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XYZOrder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_XYZOrder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_XYZOrder = __cordl_object
            .invoke("get_XYZOrder", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_Handle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<*mut crate::HoudiniEngineUnity::HEU_Handle>,
> for crate::HoudiniEngineUnity::HEU_Handle {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_Handle,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<*mut crate::HoudiniEngineUnity::HEU_Handle>,
> for crate::HoudiniEngineUnity::HEU_Handle {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_Handle,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle+HEU_HandleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_Handle_HEU_HandleType {
    #[default]
    UNSUPPORTED = 1i32,
    XFORM = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Handle+HEU_HandleType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Handle_HEU_HandleType =>
    "HoudiniEngineUnity"."HEU_Handle/HEU_HandleType"
);
