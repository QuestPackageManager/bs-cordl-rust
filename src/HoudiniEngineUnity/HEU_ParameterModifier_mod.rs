#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ParameterModifier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _parameterIndex: i32,
    pub _action: crate::HoudiniEngineUnity::HEU_ParameterModifier_ModifierAction,
    pub _instanceIndex: i32,
    pub _modifierValue: i32,
    pub _floatValue: f32,
    pub _intValue: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ParameterModifier =>
    "HoudiniEngineUnity"."HEU_ParameterModifier"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ParameterModifier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ParameterModifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier")]
impl crate::HoudiniEngineUnity::HEU_ParameterModifier {
    #[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier+ModifierAction")]
    pub type ModifierAction = crate::HoudiniEngineUnity::HEU_ParameterModifier_ModifierAction;
    pub fn GetNewModifier(
        action: crate::HoudiniEngineUnity::HEU_ParameterModifier_ModifierAction,
        parameterIndex: i32,
        instanceIndex: i32,
        modifierValue: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ParameterModifier>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ParameterModifier,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetNewModifier",
                (action, parameterIndex, instanceIndex, modifierValue),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ParameterModifier,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_ParameterModifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_ParameterModifier,
    >,
> for crate::HoudiniEngineUnity::HEU_ParameterModifier {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_ParameterModifier,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_ParameterModifier,
    >,
> for crate::HoudiniEngineUnity::HEU_ParameterModifier {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_ParameterModifier,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier+ModifierAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_ParameterModifier_ModifierAction {
    #[default]
    MULTIPARM_CLEAR = 2i32,
    MULTIPARM_INSERT = 0i32,
    MULTIPARM_REMOVE = 1i32,
    SET_FLOAT = 3i32,
    SET_INT = 4i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ParameterModifier+ModifierAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_ParameterModifier_ModifierAction => "HoudiniEngineUnity"
    ."HEU_ParameterModifier/ModifierAction"
);
