#[cfg(feature = "Unity+Burst+BurstCompiler")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+BurstCompiler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompiler => "Unity.Burst"
    ."BurstCompiler"
);
#[cfg(feature = "Unity+Burst+BurstCompiler")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler")]
impl crate::Unity::Burst::BurstCompiler {
    #[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
    pub type BurstCompilerHelper = crate::Unity::Burst::BurstCompiler_BurstCompilerHelper;
    #[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
    pub type CommandBuilder = crate::Unity::Burst::BurstCompiler_CommandBuilder;
    #[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
    pub type FakeDelegate = crate::Unity::Burst::BurstCompiler_FakeDelegate;
    #[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
    pub type StaticTypeReinitAttribute = crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute;
    #[cfg(feature = "Unity+Burst+BurstCompiler+__c")]
    pub type __c = crate::Unity::Burst::BurstCompiler___c;
}
#[cfg(feature = "Unity+Burst+BurstCompiler")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::BurstCompiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate =>
    "Unity.Burst"."BurstCompiler/BurstCompilerHelper/IsBurstEnabledDelegate"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
impl std::ops::Deref
for crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
impl std::ops::DerefMut
for crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
impl crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler_BurstCompilerHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompiler_BurstCompilerHelper
    => "Unity.Burst"."BurstCompiler/BurstCompilerHelper"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler_BurstCompilerHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompiler_BurstCompilerHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
impl crate::Unity::Burst::BurstCompiler_BurstCompilerHelper {
    #[cfg(
        feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate"
    )]
    pub type IsBurstEnabledDelegate = crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate;
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompiler_BurstCompilerHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler_CommandBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _builder: *mut crate::System::Text::StringBuilder,
    pub _hasArgs: bool,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompiler_CommandBuilder =>
    "Unity.Burst"."BurstCompiler/CommandBuilder"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler_CommandBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompiler_CommandBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
impl crate::Unity::Burst::BurstCompiler_CommandBuilder {
    pub fn And(
        &mut self,
        sep: char,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Unity::Burst::BurstCompiler_CommandBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Unity::Burst::BurstCompiler_CommandBuilder = __cordl_object
            .invoke("And", (sep))?;
        Ok(__cordl_ret)
    }
    pub fn Begin(
        &mut self,
        cmd: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Unity::Burst::BurstCompiler_CommandBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Unity::Burst::BurstCompiler_CommandBuilder = __cordl_object
            .invoke("Begin", (cmd))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SendToCompiler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("SendToCompiler", ())?;
        Ok(__cordl_ret)
    }
    pub fn With_Il2CppString0(
        &mut self,
        arg: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Unity::Burst::BurstCompiler_CommandBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Unity::Burst::BurstCompiler_CommandBuilder = __cordl_object
            .invoke("With", (arg))?;
        Ok(__cordl_ret)
    }
    pub fn With_IntPtr1(
        &mut self,
        arg: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Unity::Burst::BurstCompiler_CommandBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Unity::Burst::BurstCompiler_CommandBuilder = __cordl_object
            .invoke("With", (arg))?;
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
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompiler_CommandBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler_FakeDelegate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Method_k__BackingField: *mut crate::System::Reflection::MethodInfo,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompiler_FakeDelegate =>
    "Unity.Burst"."BurstCompiler/FakeDelegate"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler_FakeDelegate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompiler_FakeDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
impl crate::Unity::Burst::BurstCompiler_FakeDelegate {
    pub fn New(
        method: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        method: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method))?;
        Ok(__cordl_ret)
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("get_Method", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompiler_FakeDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler_StaticTypeReinitAttribute {
    __cordl_parent: crate::System::Attribute,
    pub reinitType: *mut crate::System::Type,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute => "Unity.Burst"
    ."BurstCompiler/StaticTypeReinitAttribute"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
impl std::ops::DerefMut
for crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
impl crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute {
    pub fn New(
        toReinit: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (toReinit))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        toReinit: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (toReinit))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
