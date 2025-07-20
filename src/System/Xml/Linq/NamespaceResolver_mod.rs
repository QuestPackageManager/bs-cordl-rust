#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NamespaceResolver {
    pub _scope: i32,
    pub _declaration: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration,
    >,
    pub _rover: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration,
    >,
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Linq::NamespaceResolver {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Linq";
    const CLASS_NAME: &'static str = "NamespaceResolver";
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
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Linq::NamespaceResolver {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Linq::NamespaceResolver {
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
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Linq::NamespaceResolver {
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
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Linq::NamespaceResolver {
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
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Linq::NamespaceResolver {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
impl crate::System::Xml::Linq::NamespaceResolver {
    #[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
    pub type NamespaceDeclaration = crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration;
    pub fn Add(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XNamespace>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Linq::NamespaceResolver as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XNamespace>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Linq::NamespaceResolver as
                    quest_hook::libil2cpp::Type > ::class(), "Add", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (prefix, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddFirst(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XNamespace>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Linq::NamespaceResolver as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XNamespace>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddFirst")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Linq::NamespaceResolver as
                    quest_hook::libil2cpp::Type > ::class(), "AddFirst", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (prefix, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefixOfNamespace(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XNamespace>,
        allowDefaultNamespace: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Linq::NamespaceResolver as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XNamespace>, bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetPrefixOfNamespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Linq::NamespaceResolver as
                    quest_hook::libil2cpp::Type > ::class(), "GetPrefixOfNamespace",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (ns, allowDefaultNamespace))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Linq::NamespaceResolver as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PopScope")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Linq::NamespaceResolver as
                    quest_hook::libil2cpp::Type > ::class(), "PopScope", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PushScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Linq::NamespaceResolver as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PushScope")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Linq::NamespaceResolver as
                    quest_hook::libil2cpp::Type > ::class(), "PushScope", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
#[repr(C)]
#[derive(Debug)]
pub struct NamespaceResolver_NamespaceDeclaration {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ns: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XNamespace>,
    pub scope: i32,
    pub prev: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration,
    >,
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Linq";
    const CLASS_NAME: &'static str = "NamespaceResolver/NamespaceDeclaration";
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
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
impl std::ops::Deref
for crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
impl std::ops::DerefMut
for crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
impl crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
