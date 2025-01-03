#[cfg(feature = "System+Xml+SecureStringHasher")]
#[repr(C)]
#[derive(Debug)]
pub struct SecureStringHasher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hashCodeRandomizer: i32,
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::SecureStringHasher => "System.Xml"
    ."SecureStringHasher"
);
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl std::ops::Deref for crate::System::Xml::SecureStringHasher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl std::ops::DerefMut for crate::System::Xml::SecureStringHasher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl crate::System::Xml::SecureStringHasher {
    #[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
    pub type HashCodeOfStringDelegate = crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate;
    pub fn Equals(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCodeDelegate() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCodeDelegate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCodeOfString(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sLen: i32,
        additionalEntropy: i64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCodeOfString", (key, sLen, additionalEntropy))?;
        Ok(__cordl_ret.into())
    }
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::SecureStringHasher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::Xml::SecureStringHasher {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::Xml::SecureStringHasher {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct SecureStringHasher_HashCodeOfStringDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::SecureStringHasher_HashCodeOfStringDelegate => "System.Xml"
    ."SecureStringHasher/HashCodeOfStringDelegate"
);
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
impl std::ops::Deref
for crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
impl std::ops::DerefMut
for crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
impl crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate {
    pub fn Invoke(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sLen: i32,
        additionalEntropy: i64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Invoke", (s, sLen, additionalEntropy))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
