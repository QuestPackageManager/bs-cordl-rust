#[cfg(feature = "OVRCustomFace")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCustomFace {
    __cordl_parent: OVRFace,
    pub _mappings: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
    >,
    pub retargetingType: crate::GlobalNamespace::OVRCustomFace_RetargetingType,
    pub _allowDuplicateMapping: bool,
}
#[cfg(feature = "OVRCustomFace")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRCustomFace => ""."OVRCustomFace"
);
#[cfg(feature = "OVRCustomFace")]
impl std::ops::Deref for OVRCustomFace {
    type Target = OVRFace;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomFace")]
impl std::ops::DerefMut for OVRCustomFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomFace")]
impl OVRCustomFace {
    #[cfg(feature = "OVRCustomFace+RetargetingType")]
    pub type RetargetingType = crate::GlobalNamespace::OVRCustomFace_RetargetingType;
    pub fn get_RetargetingValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRCustomFace_RetargetingType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRCustomFace_RetargetingType = __cordl_object
            .invoke("get_RetargetingValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCustomBlendShapeNameAndExpressionPairs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        > = __cordl_object.invoke("GetCustomBlendShapeNameAndExpressionPairs", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AllowDuplicateMapping(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllowDuplicateMapping", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Mappings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
        > = __cordl_object.invoke("get_Mappings", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Mappings(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Mappings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RetargetingValue(
        &mut self,
        value: crate::GlobalNamespace::OVRCustomFace_RetargetingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RetargetingValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetFaceExpression(
        &mut self,
        blendShapeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRFaceExpressions_FaceExpression = __cordl_object
            .invoke("GetFaceExpression", (blendShapeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowDuplicateMapping(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowDuplicateMapping", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRCustomFace")]
impl quest_hook::libil2cpp::ObjectType for OVRCustomFace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRCustomFace+RetargetingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRCustomFace_RetargetingType {
    Custom = 1i32,
    OculusFace = 0i32,
}
#[cfg(feature = "OVRCustomFace+RetargetingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRCustomFace_RetargetingType
    => ""."OVRCustomFace/RetargetingType"
);
