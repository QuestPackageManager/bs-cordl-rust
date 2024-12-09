#[cfg(feature = "System+Dynamic+BindingRestrictions")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::BindingRestrictions =>
    "System.Dynamic"."BindingRestrictions"
);
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
impl std::ops::Deref for crate::System::Dynamic::BindingRestrictions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
impl std::ops::DerefMut for crate::System::Dynamic::BindingRestrictions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
impl crate::System::Dynamic::BindingRestrictions {
    #[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
    pub type BindingRestrictionsProxy = crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+CustomRestriction")]
    pub type CustomRestriction = crate::GlobalNamespace::BindingRestrictions_CustomRestriction;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+InstanceRestriction")]
    pub type InstanceRestriction = crate::GlobalNamespace::BindingRestrictions_InstanceRestriction;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
    pub type MergedRestriction = crate::GlobalNamespace::BindingRestrictions_MergedRestriction;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
    pub type TestBuilder = crate::System::Dynamic::BindingRestrictions_TestBuilder;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+TypeRestriction")]
    pub type TypeRestriction = crate::GlobalNamespace::BindingRestrictions_TypeRestriction;
    pub fn GetExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("GetExpression", ())?;
        Ok(__cordl_ret)
    }
    pub fn Merge(
        &mut self,
        restrictions: *mut crate::System::Dynamic::BindingRestrictions,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Dynamic::BindingRestrictions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::BindingRestrictions = __cordl_object
            .invoke("Merge", (restrictions))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("ToExpression", ())?;
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
}
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::BindingRestrictions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions_BindingRestrictionsProxy {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy => "System.Dynamic"
    ."BindingRestrictions/BindingRestrictionsProxy"
);
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
impl std::ops::Deref
for crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
impl std::ops::DerefMut
for crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
impl crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions_TestBuilder {
    __cordl_parent: crate::System::Object,
    pub _unique: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::Dynamic::BindingRestrictions,
    >,
    pub _tests: *mut crate::System::Collections::Generic::Stack_1<
        crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode,
    >,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::BindingRestrictions_TestBuilder
    => "System.Dynamic"."BindingRestrictions/TestBuilder"
);
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
impl std::ops::Deref for crate::System::Dynamic::BindingRestrictions_TestBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
impl std::ops::DerefMut for crate::System::Dynamic::BindingRestrictions_TestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
impl crate::System::Dynamic::BindingRestrictions_TestBuilder {
    #[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
    pub type AndNode = crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode;
    pub fn Append(
        &mut self,
        restrictions: *mut crate::System::Dynamic::BindingRestrictions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Append", (restrictions))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Push(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (node, depth))?;
        Ok(__cordl_ret)
    }
    pub fn ToExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("ToExpression", ())?;
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
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::BindingRestrictions_TestBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TestBuilder_BindingRestrictions_AndNode {
    pub Depth: i32,
    pub Node: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Dynamic::TestBuilder_BindingRestrictions_AndNode => "System.Dynamic"
    ."BindingRestrictions/TestBuilder/AndNode"
);
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
impl crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {}
