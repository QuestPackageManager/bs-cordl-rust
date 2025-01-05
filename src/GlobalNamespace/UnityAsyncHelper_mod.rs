#[cfg(feature = "UnityAsyncHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityAsyncHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityAsyncHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UnityAsyncHelper => ""
    ."UnityAsyncHelper"
);
#[cfg(feature = "UnityAsyncHelper")]
impl std::ops::Deref for crate::GlobalNamespace::UnityAsyncHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn InvokeSafe_A1<A>(
        asyncTask: quest_hook::libil2cpp::Gc<
            A,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
        firstParameter: A,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeSafe", (asyncTask, firstParameter))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSafe_A_B2<A, B>(
        asyncTask: quest_hook::libil2cpp::Gc<
            A,
            B,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
        firstParameter: A,
        secondParameter: B,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeSafe", (asyncTask, firstParameter, secondParameter))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSafe_A_B_C3<A, B, C>(
        asyncTask: quest_hook::libil2cpp::Gc<
            A,
            B,
            C,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
        firstParameter: A,
        secondParameter: B,
        thirdParameter: C,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InvokeSafe",
                (asyncTask, firstParameter, secondParameter, thirdParameter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSafe_A_B_C_D4<A, B, C, D>(
        asyncTask: quest_hook::libil2cpp::Gc<
            A,
            B,
            C,
            D,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
        firstParameter: A,
        secondParameter: B,
        thirdParameter: C,
        fourthParameter: D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        D: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InvokeSafe",
                (
                    asyncTask,
                    firstParameter,
                    secondParameter,
                    thirdParameter,
                    fourthParameter,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSafe_Gc0(
        asyncTask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeSafe", (asyncTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitUntilAsync_Gc_Gc0(
        coroutineStarter: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        predicate: quest_hook::libil2cpp::Gc<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitUntilAsync", (coroutineStarter, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitUntilAsync_Gc_Gc1(
        coroutineStarter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICoroutineStarter,
        >,
        predicate: quest_hook::libil2cpp::Gc<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitUntilAsync", (coroutineStarter, predicate))?;
        Ok(__cordl_ret.into())
    }
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub __1__state: i32,
    pub __2__current: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub __4__this: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
#[repr(C)]
#[derive(Debug)]
pub struct __c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub __1__state: i32,
    pub __2__current: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub __4__this: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d
    => ""
    ."UnityAsyncHelper/<>c__DisplayClass6_0/<<WaitUntilAsync>g__WaitUntilPredicateTrue|0>d"
);
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl std::ops::Deref
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
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
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
