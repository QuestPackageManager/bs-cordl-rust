#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsignedLongField {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueField_1<u64>,
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UnsignedLongField =>
    "UnityEngine.UIElements"."UnsignedLongField"
);
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UnsignedLongField {
    type Target = crate::UnityEngine::UIElements::TextValueField_1<u64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UnsignedLongField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField")]
impl crate::UnityEngine::UIElements::UnsignedLongField {
    #[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::UnsignedLongField_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UnsignedLongInput")]
    pub type UnsignedLongInput = crate::UnityEngine::UIElements::UnsignedLongField_UnsignedLongInput;
    #[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::UnsignedLongField_UxmlTraits;
    pub fn get_unsignedLongInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::UnsignedLongField_UnsignedLongInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::UnsignedLongField_UnsignedLongInput = __cordl_object
            .invoke("get_unsignedLongInput", ())?;
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
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("StringToValue", (str))?;
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
    pub fn ApplyInputDeviceDelta(
        &mut self,
        delta: crate::UnityEngine::Vector3,
        speed: crate::UnityEngine::UIElements::DeltaSpeed,
        startValue: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInputDeviceDelta", (delta, speed, startValue))?;
        Ok(__cordl_ret)
    }
    pub fn ValueToString(
        &mut self,
        v: u64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ValueToString", (v))?;
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
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UnsignedLongField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UnsignedLongInput")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsignedLongField_UnsignedLongInput {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueField_1_TextValueInput<u64>,
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UnsignedLongInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UnsignedLongField_UnsignedLongInput =>
    "UnityEngine.UIElements"."UnsignedLongField/UnsignedLongInput"
);
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UnsignedLongInput")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UnsignedLongField_UnsignedLongInput {
    type Target = crate::UnityEngine::UIElements::TextValueField_1_TextValueInput<u64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UnsignedLongInput")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UnsignedLongField_UnsignedLongInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UnsignedLongInput")]
impl crate::UnityEngine::UIElements::UnsignedLongField_UnsignedLongInput {
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
    pub fn ApplyInputDeviceDelta(
        &mut self,
        delta: crate::UnityEngine::Vector3,
        speed: crate::UnityEngine::UIElements::DeltaSpeed,
        startValue: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInputDeviceDelta", (delta, speed, startValue))?;
        Ok(__cordl_ret)
    }
    pub fn ClampToMinMaxULongValue(
        &mut self,
        niceDelta: i64,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object
            .invoke("ClampToMinMaxULongValue", (niceDelta, value))?;
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
    pub fn ValueToString(
        &mut self,
        v: u64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ValueToString", (v))?;
        Ok(__cordl_ret)
    }
    pub fn StringToValue(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("StringToValue", (str))?;
        Ok(__cordl_ret)
    }
    pub fn get_parentUnsignedLongField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::UnsignedLongField,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::UnsignedLongField = __cordl_object
            .invoke("get_parentUnsignedLongField", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UnsignedLongInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UnsignedLongField_UnsignedLongInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsignedLongField_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::UnsignedLongField,
        *mut crate::UnityEngine::UIElements::UnsignedLongField_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UnsignedLongField_UxmlFactory => "UnityEngine.UIElements"
    ."UnsignedLongField/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UnsignedLongField_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::UnsignedLongField,
        *mut crate::UnityEngine::UIElements::UnsignedLongField_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlFactory")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UnsignedLongField_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlFactory")]
impl crate::UnityEngine::UIElements::UnsignedLongField_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UnsignedLongField_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsignedLongField_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::TextValueFieldTraits_2<
        u64,
        *mut crate::UnityEngine::UIElements::UxmlUnsignedLongAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UnsignedLongField_UxmlTraits => "UnityEngine.UIElements"
    ."UnsignedLongField/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UnsignedLongField_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::TextValueFieldTraits_2<
        u64,
        *mut crate::UnityEngine::UIElements::UxmlUnsignedLongAttributeDescription,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlTraits")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UnsignedLongField_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlTraits")]
impl crate::UnityEngine::UIElements::UnsignedLongField_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+UnsignedLongField+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UnsignedLongField_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
