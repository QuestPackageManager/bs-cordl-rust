#[cfg(feature = "System+Linq+Expressions+ByRefParameterExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct ByRefParameterExpression {
    __cordl_parent: crate::System::Linq::Expressions::TypedParameterExpression,
}
#[cfg(feature = "System+Linq+Expressions+ByRefParameterExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::ByRefParameterExpression => "System.Linq.Expressions"
    ."ByRefParameterExpression"
);
#[cfg(feature = "System+Linq+Expressions+ByRefParameterExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::ByRefParameterExpression {
    type Target = crate::System::Linq::Expressions::TypedParameterExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ByRefParameterExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ByRefParameterExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ByRefParameterExpression")]
impl crate::System::Linq::Expressions::ByRefParameterExpression {
    pub fn GetIsByRef(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetIsByRef", ())?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "System+Linq+Expressions+ByRefParameterExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ByRefParameterExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}