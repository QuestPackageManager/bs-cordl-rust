#[cfg(feature = "NullAllowedIf")]
#[repr(C)]
#[derive(Debug)]
pub struct NullAllowedIf {
    __cordl_parent: crate::GlobalNamespace::NullAllowed,
    pub propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _valueToCompare: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _comparisonOperation: crate::GlobalNamespace::ComparisonOperation,
}
#[cfg(feature = "NullAllowedIf")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NullAllowedIf => ""
    ."NullAllowedIf"
);
#[cfg(feature = "NullAllowedIf")]
impl std::ops::Deref for crate::GlobalNamespace::NullAllowedIf {
    type Target = crate::GlobalNamespace::NullAllowed;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowedIf")]
impl std::ops::DerefMut for crate::GlobalNamespace::NullAllowedIf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowedIf")]
impl crate::GlobalNamespace::NullAllowedIf {
    pub fn IsNullAllowedFor(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNullAllowedFor", (value, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ComparisonOperation_Il2CppObject_NullAllowed_Context1(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        comparisonOperation: crate::GlobalNamespace::ComparisonOperation,
        valueToCompare: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (propertyName, comparisonOperation, valueToCompare, context),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject_NullAllowed_Context0(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        equalsTo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName, equalsTo, context))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ComparisonOperation_Il2CppObject_NullAllowed_Context1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        comparisonOperation: crate::GlobalNamespace::ComparisonOperation,
        valueToCompare: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_NullAllowed_Context0(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        equalsTo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyName, equalsTo, context))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NullAllowedIf")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NullAllowedIf {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
