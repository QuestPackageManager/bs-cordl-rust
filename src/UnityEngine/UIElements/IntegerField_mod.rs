#[cfg(feature = "UnityEngine+UIElements+IntegerField")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegerField {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueField_1<i32>,
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IntegerField =>
    "UnityEngine.UIElements"."IntegerField"
);
#[cfg(feature = "UnityEngine+UIElements+IntegerField")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IntegerField {
    type Target = crate::UnityEngine::UIElements::TextValueField_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IntegerField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField")]
impl crate::UnityEngine::UIElements::IntegerField {
    #[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::IntegerField_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::IntegerField_UxmlTraits;
    #[cfg(feature = "UnityEngine+UIElements+IntegerField+IntegerInput")]
    pub type IntegerInput = crate::UnityEngine::UIElements::IntegerField_IntegerInput;
    pub fn ApplyInputDeviceDelta(
        &mut self,
        delta: crate::UnityEngine::Vector3,
        speed: crate::UnityEngine::UIElements::DeltaSpeed,
        startValue: i32,
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
    pub fn StringToValue(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("StringToValue", (str))?;
        Ok(__cordl_ret)
    }
    pub fn ValueToString(
        &mut self,
        v: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ValueToString", (v))?;
        Ok(__cordl_ret)
    }
    pub fn get_integerInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IntegerField_IntegerInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IntegerField_IntegerInput = __cordl_object
            .invoke("get_integerInput", ())?;
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_i32_1(
        label: *mut crate::System::String,
        maxLength: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, maxLength))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::IntegerField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+IntegerInput")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegerField_IntegerInput {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueField_1_TextValueInput<i32>,
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+IntegerInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IntegerField_IntegerInput => "UnityEngine.UIElements"
    ."IntegerField/IntegerInput"
);
#[cfg(feature = "UnityEngine+UIElements+IntegerField+IntegerInput")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IntegerField_IntegerInput {
    type Target = crate::UnityEngine::UIElements::TextValueField_1_TextValueInput<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+IntegerInput")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IntegerField_IntegerInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+IntegerInput")]
impl crate::UnityEngine::UIElements::IntegerField_IntegerInput {
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
    pub fn ValueToString(
        &mut self,
        v: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ValueToString", (v))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyInputDeviceDelta(
        &mut self,
        delta: crate::UnityEngine::Vector3,
        speed: crate::UnityEngine::UIElements::DeltaSpeed,
        startValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInputDeviceDelta", (delta, speed, startValue))?;
        Ok(__cordl_ret)
    }
    pub fn get_parentIntegerField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IntegerField,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IntegerField = __cordl_object
            .invoke("get_parentIntegerField", ())?;
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
    pub fn StringToValue(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("StringToValue", (str))?;
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
#[cfg(feature = "UnityEngine+UIElements+IntegerField+IntegerInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IntegerField_IntegerInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegerField_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::IntegerField,
        *mut crate::UnityEngine::UIElements::IntegerField_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IntegerField_UxmlFactory => "UnityEngine.UIElements"
    ."IntegerField/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IntegerField_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::IntegerField,
        *mut crate::UnityEngine::UIElements::IntegerField_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IntegerField_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlFactory")]
impl crate::UnityEngine::UIElements::IntegerField_UxmlFactory {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IntegerField_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegerField_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueFieldTraits_2<
        i32,
        *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IntegerField_UxmlTraits
    => "UnityEngine.UIElements"."IntegerField/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IntegerField_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::TextValueFieldTraits_2<
        i32,
        *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IntegerField_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlTraits")]
impl crate::UnityEngine::UIElements::IntegerField_UxmlTraits {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IntegerField+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IntegerField_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
