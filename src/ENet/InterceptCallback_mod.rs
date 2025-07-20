#[cfg(feature = "ENet+InterceptCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct InterceptCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "ENet+InterceptCallback")]
unsafe impl quest_hook::libil2cpp::Type for crate::ENet::InterceptCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "ENet";
    const CLASS_NAME: &'static str = "InterceptCallback";
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
#[cfg(feature = "ENet+InterceptCallback")]
impl std::ops::Deref for crate::ENet::InterceptCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+InterceptCallback")]
impl std::ops::DerefMut for crate::ENet::InterceptCallback {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+InterceptCallback")]
impl crate::ENet::InterceptCallback {
    pub fn BeginInvoke(
        &mut self,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
        receivedData: crate::System::IntPtr,
        receivedDataLength: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
                            quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
                            crate::System::IntPtr,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        6usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (event, address, receivedData, receivedDataLength, callback, object),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
                            quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        i32,
                        3usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (event, address, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
        receivedData: crate::System::IntPtr,
        receivedDataLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
                            quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
                            crate::System::IntPtr,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (event, address, receivedData, receivedDataLength),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ENet+InterceptCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::InterceptCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
