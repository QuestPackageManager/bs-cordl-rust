#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemEnumerable_1<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _enumerator: quest_hook::libil2cpp::Gc<
        crate::System::IO::Enumeration::FileSystemEnumerable_1_DelegateEnumerator<
            TResult,
        >,
    >,
    pub _transform: quest_hook::libil2cpp::Gc<
        crate::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform<TResult>,
    >,
    pub _options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    pub _directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _ShouldIncludePredicate_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult>,
    >,
    pub _ShouldRecursePredicate_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult>,
    >,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Enumeration::FileSystemEnumerable_1
    < TResult > => "System.IO.Enumeration"."FileSystemEnumerable`1" < TResult >
);
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult> {
    #[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+DelegateEnumerator")]
    pub type DelegateEnumerator = crate::System::IO::Enumeration::FileSystemEnumerable_1_DelegateEnumerator<
        TResult,
    >;
    #[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindPredicate")]
    pub type FindPredicate = crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<
        TResult,
    >;
    #[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindTransform")]
    pub type FindTransform = crate::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform<
        TResult,
    >;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TResult>,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        transform: quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform<TResult>,
        >,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (directory, transform, options))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        transform: quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform<TResult>,
        >,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (directory, transform, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldIncludePredicate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult>,
        > = __cordl_object.invoke("get_ShouldIncludePredicate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldRecursePredicate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult>,
        > = __cordl_object.invoke("get_ShouldRecursePredicate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ShouldIncludePredicate(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ShouldIncludePredicate", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TResult>>
for crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TResult> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TResult>>
for crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TResult> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
impl<TResult: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1")]
impl<TResult: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+DelegateEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemEnumerable_1_DelegateEnumerator<
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::IO::Enumeration::FileSystemEnumerator_1<TResult>,
    pub _enumerable: quest_hook::libil2cpp::Gc<
        crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult>,
    >,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+DelegateEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Enumeration::FileSystemEnumerable_1_DelegateEnumerator < TResult > =>
    "System.IO.Enumeration"."FileSystemEnumerable`1/DelegateEnumerator" < TResult >
);
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+DelegateEnumerator")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::IO::Enumeration::FileSystemEnumerable_1_DelegateEnumerator<TResult> {
    type Target = crate::System::IO::Enumeration::FileSystemEnumerator_1<TResult>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+DelegateEnumerator")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::IO::Enumeration::FileSystemEnumerable_1_DelegateEnumerator<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+DelegateEnumerator")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::IO::Enumeration::FileSystemEnumerable_1_DelegateEnumerator<TResult> {
    pub fn New(
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (enumerable))?;
        Ok(__cordl_object.into())
    }
    pub fn ShouldIncludeEntry(
        &mut self,
        entry: quest_hook::libil2cpp::ByRefMut<
            crate::System::IO::Enumeration::FileSystemEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldIncludeEntry", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldRecurseIntoEntry(
        &mut self,
        entry: quest_hook::libil2cpp::ByRefMut<
            crate::System::IO::Enumeration::FileSystemEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldRecurseIntoEntry", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransformEntry(
        &mut self,
        entry: quest_hook::libil2cpp::ByRefMut<
            crate::System::IO::Enumeration::FileSystemEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TResult = __cordl_object.invoke("TransformEntry", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::IO::Enumeration::FileSystemEnumerable_1<TResult>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (enumerable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+DelegateEnumerator")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::IO::Enumeration::FileSystemEnumerable_1_DelegateEnumerator<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindPredicate")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemEnumerable_1_FindPredicate<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindPredicate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate < TResult > =>
    "System.IO.Enumeration"."FileSystemEnumerable`1/FindPredicate" < TResult >
);
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindPredicate")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindPredicate")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindPredicate")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult> {
    pub fn Invoke(
        &mut self,
        entry: quest_hook::libil2cpp::ByRefMut<
            crate::System::IO::Enumeration::FileSystemEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindPredicate")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::IO::Enumeration::FileSystemEnumerable_1_FindPredicate<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemEnumerable_1_FindTransform<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform < TResult > =>
    "System.IO.Enumeration"."FileSystemEnumerable`1/FindTransform" < TResult >
);
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindTransform")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform<TResult> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindTransform")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindTransform")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform<TResult> {
    pub fn Invoke(
        &mut self,
        entry: quest_hook::libil2cpp::ByRefMut<
            crate::System::IO::Enumeration::FileSystemEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TResult = __cordl_object.invoke("Invoke", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerable_1+FindTransform")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::IO::Enumeration::FileSystemEnumerable_1_FindTransform<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
