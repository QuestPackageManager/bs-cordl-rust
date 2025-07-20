#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
#[repr(C)]
#[derive(Debug)]
pub struct Axis {
    __cordl_parent: crate::MS::Internal::Xml::XPath::AstNode,
    pub _axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
    pub _input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    pub _prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _nodeType: crate::System::Xml::XPath::XPathNodeType,
    pub abbrAxis: bool,
    pub _urn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
unsafe impl quest_hook::libil2cpp::Type for crate::MS::Internal::Xml::XPath::Axis {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "Axis";
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
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::Axis {
    type Target = crate::MS::Internal::Xml::XPath::AstNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::Axis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
impl crate::MS::Internal::Xml::XPath::Axis {
    #[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
    pub type AxisType = crate::MS::Internal::Xml::XPath::Axis_AxisType;
    pub fn New_Axis_AxisType_AstNode1(
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axisType, input))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_XPathNodeType0(
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nodetype: crate::System::Xml::XPath::XPathNodeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axisType, input, prefix, name, nodetype))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Axis_AxisType_AstNode1(
        &mut self,
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::MS::Internal::Xml::XPath::Axis_AxisType,
                    quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (axisType, input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_XPathNodeType0(
        &mut self,
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nodetype: crate::System::Xml::XPath::XPathNodeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::MS::Internal::Xml::XPath::Axis_AxisType,
                    quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Xml::XPath::XPathNodeType,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (axisType, input, prefix, name, nodetype))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AbbrAxis(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_AbbrAxis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_AbbrAxis", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Input(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
                0usize,
            >("get_Input")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_Input", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Name")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_Name", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathNodeType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Xml::XPath::XPathNodeType,
                0usize,
            >("get_NodeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_NodeType", 0usize
                )
            });
        let __cordl_ret: crate::System::Xml::XPath::XPathNodeType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Prefix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_Prefix", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathResultType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Xml::XPath::XPathResultType,
                0usize,
            >("get_ReturnType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_ReturnType", 0usize
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::MS::Internal::Xml::XPath::AstNode_AstType,
                0usize,
            >("get_Type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_Type", 0usize
                )
            });
        let __cordl_ret: crate::MS::Internal::Xml::XPath::AstNode_AstType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeOfAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::MS::Internal::Xml::XPath::Axis_AxisType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::MS::Internal::Xml::XPath::Axis_AxisType,
                0usize,
            >("get_TypeOfAxis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_TypeOfAxis", 0usize
                )
            });
        let __cordl_ret: crate::MS::Internal::Xml::XPath::Axis_AxisType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Urn(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Urn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "get_Urn", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Input(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Input")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "set_Input", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Urn(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Urn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MS::Internal::Xml::XPath::Axis as quest_hook::libil2cpp::Type
                    > ::class(), "set_Urn", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::Axis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis_AxisType {
    #[default]
    Ancestor = 0i32,
    AncestorOrSelf = 1i32,
    Attribute = 2i32,
    Child = 3i32,
    Descendant = 4i32,
    DescendantOrSelf = 5i32,
    Following = 6i32,
    FollowingSibling = 7i32,
    Namespace = 8i32,
    None = 13i32,
    Parent = 9i32,
    Preceding = 10i32,
    PrecedingSibling = 11i32,
    _cordl_Self = 12i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::MS::Internal::Xml::XPath::Axis_AxisType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "Axis/AxisType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::MS::Internal::Xml::XPath::Axis_AxisType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::MS::Internal::Xml::XPath::Axis_AxisType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::MS::Internal::Xml::XPath::Axis_AxisType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::MS::Internal::Xml::XPath::Axis_AxisType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
