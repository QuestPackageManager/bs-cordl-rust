#[cfg(feature = "UnityEngine+UIElements+FloatField")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatField {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueField_1<f32>,
}
#[cfg(feature = "UnityEngine+UIElements+FloatField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FloatField =>
    "UnityEngine.UIElements"."FloatField"
);
#[cfg(feature = "UnityEngine+UIElements+FloatField")]
impl std::ops::Deref for crate::UnityEngine::UIElements::FloatField {
    type Target = crate::UnityEngine::UIElements::TextValueField_1<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::FloatField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField")]
impl crate::UnityEngine::UIElements::FloatField {
    #[cfg(feature = "UnityEngine+UIElements+FloatField+FloatInput")]
    pub type FloatInput = crate::UnityEngine::UIElements::FloatField_FloatInput;
    #[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::FloatField_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::FloatField_UxmlTraits;
    pub fn ApplyInputDeviceDelta(
        &mut self,
        delta: crate::UnityEngine::Vector3,
        speed: crate::UnityEngine::UIElements::DeltaSpeed,
        startValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInputDeviceDelta", (delta, speed, startValue))?;
        Ok(__cordl_ret)
    }
    pub fn CanTryParse(
        &mut self,
        textString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanTryParse", (textString))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_i32_1(
        label: *mut crate::System::String,
        maxLength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, maxLength))?;
        Ok(__cordl_object)
    }
    pub fn StringToValue(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("StringToValue", (str))?;
        Ok(__cordl_ret)
    }
    pub fn ValueToString(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ValueToString", (v))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_i32_1(
        &mut self,
        label: *mut crate::System::String,
        maxLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label, maxLength))?;
        Ok(__cordl_ret)
    }
    pub fn get_floatInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::FloatField_FloatInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::FloatField_FloatInput = __cordl_object
            .invoke("get_floatInput", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::FloatField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+FloatInput")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatField_FloatInput {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueField_1_TextValueInput<f32>,
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+FloatInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FloatField_FloatInput
    => "UnityEngine.UIElements"."FloatField/FloatInput"
);
#[cfg(feature = "UnityEngine+UIElements+FloatField+FloatInput")]
impl std::ops::Deref for crate::UnityEngine::UIElements::FloatField_FloatInput {
    type Target = crate::UnityEngine::UIElements::TextValueField_1_TextValueInput<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+FloatInput")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::FloatField_FloatInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+FloatInput")]
impl crate::UnityEngine::UIElements::FloatField_FloatInput {
    pub fn ApplyInputDeviceDelta(
        &mut self,
        delta: crate::UnityEngine::Vector3,
        speed: crate::UnityEngine::UIElements::DeltaSpeed,
        startValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInputDeviceDelta", (delta, speed, startValue))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn StringToValue(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("StringToValue", (str))?;
        Ok(__cordl_ret)
    }
    pub fn ValueToString(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ValueToString", (v))?;
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
    pub fn get_allowedCharacters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_allowedCharacters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_parentFloatField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::FloatField> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::FloatField = __cordl_object
            .invoke("get_parentFloatField", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+FloatInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::FloatField_FloatInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatField_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::FloatField,
        *mut crate::UnityEngine::UIElements::FloatField_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FloatField_UxmlFactory
    => "UnityEngine.UIElements"."FloatField/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::FloatField_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::FloatField,
        *mut crate::UnityEngine::UIElements::FloatField_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::FloatField_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlFactory")]
impl crate::UnityEngine::UIElements::FloatField_UxmlFactory {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::FloatField_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatField_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueFieldTraits_2<
        f32,
        *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FloatField_UxmlTraits
    => "UnityEngine.UIElements"."FloatField/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::FloatField_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::TextValueFieldTraits_2<
        f32,
        *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::FloatField_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlTraits")]
impl crate::UnityEngine::UIElements::FloatField_UxmlTraits {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UIElements+FloatField+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::FloatField_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
