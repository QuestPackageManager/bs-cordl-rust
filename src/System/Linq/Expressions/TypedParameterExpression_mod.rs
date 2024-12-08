#[cfg(feature = "System+Linq+Expressions+TypedParameterExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct TypedParameterExpression {
    __cordl_parent: crate::System::Linq::Expressions::ParameterExpression,
    pub _Type_k__BackingField: *mut crate::System::Type,
}
#[cfg(feature = "System+Linq+Expressions+TypedParameterExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::TypedParameterExpression => "System.Linq.Expressions"
    ."TypedParameterExpression"
);
#[cfg(feature = "System+Linq+Expressions+TypedParameterExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::TypedParameterExpression {
    type Target = crate::System::Linq::Expressions::ParameterExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+TypedParameterExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::TypedParameterExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+TypedParameterExpression")]
impl crate::System::Linq::Expressions::TypedParameterExpression {
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, name))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+TypedParameterExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::TypedParameterExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
