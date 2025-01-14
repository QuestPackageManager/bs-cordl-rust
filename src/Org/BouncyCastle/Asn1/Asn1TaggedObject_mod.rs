#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1TaggedObject {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Object,
    pub tagNo: i32,
    pub explicitly: bool,
    pub obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1";
    const CLASS_NAME: &'static str = "Asn1TaggedObject";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
impl crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    pub fn Asn1Equals(
        &mut self,
        asn1Object: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>),
                bool,
                1usize,
            >("Asn1Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Asn1Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (asn1Object)) };
        Ok(__cordl_ret.into())
    }
    pub fn Asn1GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("Asn1GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Asn1GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Asn1TaggedObject__cordl_bool0(
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1TaggedObject>,
        explicitly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1TaggedObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
                >,
                2usize,
            >("GetInstance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInstance", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
        > = unsafe { method.invoke_unchecked((), (obj, explicitly)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Il2CppObject1(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1TaggedObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
                >,
                1usize,
            >("GetInstance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInstance", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
                0usize,
            >("GetObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetObject", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectParser(
        &mut self,
        tag: i32,
        isExplicit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, bool),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
                >,
                2usize,
            >("GetObjectParser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetObjectParser", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
        > = unsafe { method.invoke_unchecked(self, (tag, isExplicit)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsConstructed(
        isExplicit: bool,
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    bool,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
                ),
                bool,
                2usize,
            >("IsConstructed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsConstructed", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (isExplicit, obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsEmpty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsEmpty", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IsExplicit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsExplicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsExplicit", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New__cordl_bool_i32_Asn1Encodable1(
        explicitly: bool,
        tagNo: i32,
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (explicitly, tagNo, obj))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_Asn1Encodable0(
        tagNo: i32,
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagNo, obj))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_i32_Asn1Encodable1(
        &mut self,
        explicitly: bool,
        tagNo: i32,
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Asn1Encodable,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (explicitly, tagNo, obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Asn1Encodable0(
        &mut self,
        tagNo: i32,
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Asn1Encodable,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tagNo, obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TagNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_TagNo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_TagNo", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
impl AsRef<crate::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser>
for crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
impl AsMut<crate::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser>
for crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObject")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::Asn1TaggedObject {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}
