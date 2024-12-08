#[cfg(feature = "NullAllowedIf")]
#[repr(C)]
#[derive(Debug)]
pub struct NullAllowedIf {
    __cordl_parent: NullAllowed,
    pub propertyName: *mut crate::System::String,
    pub _valueToCompare: *mut crate::System::Object,
    pub _comparisonOperation: ComparisonOperation,
}
#[cfg(feature = "NullAllowedIf")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NullAllowedIf => ""."NullAllowedIf"
);
#[cfg(feature = "NullAllowedIf")]
impl std::ops::Deref for NullAllowedIf {
    type Target = NullAllowed;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowedIf")]
impl std::ops::DerefMut for NullAllowedIf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowedIf")]
impl NullAllowedIf {
    pub fn IsNullAllowedFor(
        &mut self,
        value: *mut crate::System::Object,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNullAllowedFor", (value, context))?;
        Ok(__cordl_ret)
    }
    pub fn New_ComparisonOperation_Object_NullAllowed_Context1(
        propertyName: *mut crate::System::String,
        comparisonOperation: ComparisonOperation,
        valueToCompare: *mut crate::System::Object,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (propertyName, comparisonOperation, valueToCompare, context),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Object_NullAllowed_Context0(
        propertyName: *mut crate::System::String,
        equalsTo: *mut crate::System::Object,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName, equalsTo, context))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ComparisonOperation_Object_NullAllowed_Context1(
        &mut self,
        propertyName: *mut crate::System::String,
        comparisonOperation: ComparisonOperation,
        valueToCompare: *mut crate::System::Object,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (propertyName, comparisonOperation, valueToCompare, context),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_NullAllowed_Context0(
        &mut self,
        propertyName: *mut crate::System::String,
        equalsTo: *mut crate::System::Object,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyName, equalsTo, context))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NullAllowedIf")]
impl quest_hook::libil2cpp::ObjectType for NullAllowedIf {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
