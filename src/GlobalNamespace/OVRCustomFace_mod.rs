#[cfg(feature = "OVRCustomFace")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCustomFace {
    __cordl_parent: crate::GlobalNamespace::OVRFace,
    pub _mappings: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
        >,
    >,
    pub retargetingType: crate::GlobalNamespace::OVRCustomFace_RetargetingType,
    pub _allowDuplicateMapping: bool,
}
#[cfg(feature = "OVRCustomFace")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRCustomFace {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRCustomFace";
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
#[cfg(feature = "OVRCustomFace")]
impl std::ops::Deref for crate::GlobalNamespace::OVRCustomFace {
    type Target = crate::GlobalNamespace::OVRFace;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomFace")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRCustomFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomFace")]
impl crate::GlobalNamespace::OVRCustomFace {
    #[cfg(feature = "OVRCustomFace+RetargetingType")]
    pub type RetargetingType = crate::GlobalNamespace::OVRCustomFace_RetargetingType;
    pub fn GetCustomBlendShapeNameAndExpressionPairs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                >,
            >,
        > = __cordl_object.invoke("GetCustomBlendShapeNameAndExpressionPairs", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn get_AllowDuplicateMapping(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowDuplicateMapping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mappings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        > = __cordl_object.invoke("get_Mappings", ())?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_Mappings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Mappings", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRCustomFace")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRCustomFace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRCustomFace+RetargetingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRCustomFace_RetargetingType {
    #[default]
    Custom = 1i32,
    OculusFace = 0i32,
}
#[cfg(feature = "OVRCustomFace+RetargetingType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRCustomFace_RetargetingType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRCustomFace/RetargetingType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRCustomFace+RetargetingType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRCustomFace_RetargetingType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRCustomFace+RetargetingType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRCustomFace_RetargetingType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRCustomFace+RetargetingType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRCustomFace_RetargetingType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRCustomFace+RetargetingType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRCustomFace_RetargetingType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
