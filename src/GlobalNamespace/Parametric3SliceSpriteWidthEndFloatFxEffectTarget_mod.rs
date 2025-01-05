#[cfg(feature = "Parametric3SliceSpriteWidthEndFloatFxEffectTarget")]
#[repr(C)]
#[derive(Debug)]
pub struct Parametric3SliceSpriteWidthEndFloatFxEffectTarget {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FloatFxGroupEffectTarget,
    >,
    pub _parametric3SliceSpriteController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Parametric3SliceSpriteController,
    >,
    pub _valueBounds: crate::UnityEngine::Vector2,
    pub _valueMultiplier: f32,
}
#[cfg(feature = "Parametric3SliceSpriteWidthEndFloatFxEffectTarget")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::Parametric3SliceSpriteWidthEndFloatFxEffectTarget => ""
    ."Parametric3SliceSpriteWidthEndFloatFxEffectTarget"
);
#[cfg(feature = "Parametric3SliceSpriteWidthEndFloatFxEffectTarget")]
impl std::ops::Deref
for crate::GlobalNamespace::Parametric3SliceSpriteWidthEndFloatFxEffectTarget {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FloatFxGroupEffectTarget,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Parametric3SliceSpriteWidthEndFloatFxEffectTarget")]
impl std::ops::DerefMut
for crate::GlobalNamespace::Parametric3SliceSpriteWidthEndFloatFxEffectTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Parametric3SliceSpriteWidthEndFloatFxEffectTarget")]
impl crate::GlobalNamespace::Parametric3SliceSpriteWidthEndFloatFxEffectTarget {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (groupId, elementId, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerValue", (groupId, elementId, value))?;
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
#[cfg(feature = "Parametric3SliceSpriteWidthEndFloatFxEffectTarget")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::Parametric3SliceSpriteWidthEndFloatFxEffectTarget {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
