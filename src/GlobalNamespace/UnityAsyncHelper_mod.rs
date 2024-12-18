#[cfg(feature = "UnityAsyncHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityAsyncHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityAsyncHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UnityAsyncHelper => ""
    ."UnityAsyncHelper"
);
#[cfg(feature = "UnityAsyncHelper")]
impl std::ops::Deref for crate::GlobalNamespace::UnityAsyncHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityAsyncHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnityAsyncHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityAsyncHelper")]
impl crate::GlobalNamespace::UnityAsyncHelper {
    #[cfg(feature = "UnityAsyncHelper+_InvokeSafe_d__0")]
    pub type _InvokeSafe_d__0 = crate::GlobalNamespace::UnityAsyncHelper__InvokeSafe_d__0;
    #[cfg(feature = "UnityAsyncHelper+_InvokeSafe_d__1_1")]
    pub type _InvokeSafe_d__1_1<A: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::UnityAsyncHelper__InvokeSafe_d__1_1<
        A,
    >;
    #[cfg(feature = "UnityAsyncHelper+_InvokeSafe_d__2_2")]
    pub type _InvokeSafe_d__2_2<
        A: quest_hook::libil2cpp::Type,
        B: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::UnityAsyncHelper__InvokeSafe_d__2_2<A, B>;
    #[cfg(feature = "UnityAsyncHelper+_InvokeSafe_d__3_3")]
    pub type _InvokeSafe_d__3_3<
        A: quest_hook::libil2cpp::Type,
        B: quest_hook::libil2cpp::Type,
        C: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::UnityAsyncHelper__InvokeSafe_d__3_3<A, B, C>;
    #[cfg(feature = "UnityAsyncHelper+_InvokeSafe_d__4_4")]
    pub type _InvokeSafe_d__4_4<
        A: quest_hook::libil2cpp::Type,
        B: quest_hook::libil2cpp::Type,
        C: quest_hook::libil2cpp::Type,
        D: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::UnityAsyncHelper__InvokeSafe_d__4_4<A, B, C, D>;
    #[cfg(feature = "UnityAsyncHelper+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::GlobalNamespace::UnityAsyncHelper___c__DisplayClass5_0;
}
#[cfg(feature = "UnityAsyncHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UnityAsyncHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
#[repr(C)]
#[derive(Debug)]
pub struct __c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub __1__state: i32,
    pub __2__current: *mut quest_hook::libil2cpp::Il2CppObject,
    pub __4__this: *mut crate::GlobalNamespace::UnityAsyncHelper___c__DisplayClass5_0,
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d
    => ""
    ."UnityAsyncHelper/<>c__DisplayClass5_0/<<WaitUntilAsync>g__WaitUntilPredicateTrue|0>d"
);
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl std::ops::Deref
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        __1__state: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (__1__state))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_Generic_IEnumerator_System_Object__get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "System.Collections.Generic.IEnumerator<System.Object>.get_Current",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IEnumerator.Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        __1__state: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (__1__state))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<
    crate::System::Collections::Generic::IEnumerator_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<
    crate::System::Collections::Generic::IEnumerator_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
