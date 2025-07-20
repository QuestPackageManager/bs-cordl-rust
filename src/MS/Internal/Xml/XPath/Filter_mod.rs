#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
#[repr(C)]
#[derive(Debug)]
pub struct Filter {
    __cordl_parent: crate::MS::Internal::Xml::XPath::AstNode,
    pub _input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    pub _condition: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
unsafe impl quest_hook::libil2cpp::Type for crate::MS::Internal::Xml::XPath::Filter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "Filter";
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
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::Filter {
    type Target = crate::MS::Internal::Xml::XPath::AstNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::Filter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
impl crate::MS::Internal::Xml::XPath::Filter {
    pub fn New(
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        condition: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, condition))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        condition: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Filter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
                    quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Filter as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input, condition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathResultType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Filter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Xml::XPath::XPathResultType,
                0usize,
            >("get_ReturnType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Filter as
                    quest_hook::libil2cpp::Type > ::class(), "get_ReturnType", 0usize
                )
            });
        let __cordl_ret: crate::System::Xml::XPath::XPathResultType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::MS::Internal::Xml::XPath::AstNode_AstType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Filter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::MS::Internal::Xml::XPath::AstNode_AstType,
                0usize,
            >("get_Type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Filter as
                    quest_hook::libil2cpp::Type > ::class(), "get_Type", 0usize
                )
            });
        let __cordl_ret: crate::MS::Internal::Xml::XPath::AstNode_AstType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::Filter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
