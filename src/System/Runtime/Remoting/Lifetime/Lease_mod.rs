#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease")]
#[repr(C)]
#[derive(Debug)]
pub struct Lease {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub _leaseExpireTime: crate::System::DateTime,
    pub _currentState: crate::System::Runtime::Remoting::Lifetime::LeaseState,
    pub _initialLeaseTime: crate::System::TimeSpan,
    pub _renewOnCallTime: crate::System::TimeSpan,
    pub _sponsorshipTimeout: crate::System::TimeSpan,
    pub _sponsors: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub _renewingSponsors: quest_hook::libil2cpp::Gc<crate::System::Collections::Queue>,
    pub _renewalDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Lifetime::Lease_RenewalDelegate,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Lifetime::Lease {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Lifetime";
    const CLASS_NAME: &'static str = "Lease";
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
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Lifetime::Lease {
    type Target = crate::System::MarshalByRefObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Lifetime::Lease {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease")]
impl crate::System::Runtime::Remoting::Lifetime::Lease {
    #[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease+RenewalDelegate")]
    pub type RenewalDelegate = crate::System::Runtime::Remoting::Lifetime::Lease_RenewalDelegate;
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Activate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Activate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNextSponsor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CheckNextSponsor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckNextSponsor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessSponsorResponse(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        timedOut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessSponsorResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessSponsorResponse", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, timedOut))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Renew(
        &mut self,
        renewalTime: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::TimeSpan),
                crate::System::TimeSpan,
                1usize,
            >("Renew")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Renew", 1usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked(self, (renewalTime))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unregister(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Lifetime::ISponsor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Lifetime::ISponsor,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Unregister")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Unregister", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentLeaseTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::TimeSpan, 0usize>("get_CurrentLeaseTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_CurrentLeaseTime", 0usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Remoting::Lifetime::LeaseState,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Runtime::Remoting::Lifetime::LeaseState,
                0usize,
            >("get_CurrentState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_CurrentState", 0usize
                )
            });
        let __cordl_ret: crate::System::Runtime::Remoting::Lifetime::LeaseState = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RenewOnCallTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::TimeSpan, 0usize>("get_RenewOnCallTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_RenewOnCallTime", 0usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Lifetime::Lease {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease")]
impl AsRef<crate::System::Runtime::Remoting::Lifetime::ILease>
for crate::System::Runtime::Remoting::Lifetime::Lease {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Lifetime::ILease {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease")]
impl AsMut<crate::System::Runtime::Remoting::Lifetime::ILease>
for crate::System::Runtime::Remoting::Lifetime::Lease {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Remoting::Lifetime::ILease {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease+RenewalDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct Lease_RenewalDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease+RenewalDelegate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Lifetime::Lease_RenewalDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Lifetime";
    const CLASS_NAME: &'static str = "Lease/RenewalDelegate";
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
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease+RenewalDelegate")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Lifetime::Lease_RenewalDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease+RenewalDelegate")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Lifetime::Lease_RenewalDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease+RenewalDelegate")]
impl crate::System::Runtime::Remoting::Lifetime::Lease_RenewalDelegate {
    pub fn BeginInvoke(
        &mut self,
        lease: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Lifetime::ILease,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Remoting::Lifetime::ILease,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                3usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginInvoke", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (lease, callback, object))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                crate::System::TimeSpan,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndInvoke", 1usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked(self, (result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        lease: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Lifetime::ILease,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Lifetime::ILease,
                >),
                crate::System::TimeSpan,
                1usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Invoke", 1usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked(self, (lease))
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
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
            method.invoke_unchecked(self, (object, method))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+Lease+RenewalDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Lifetime::Lease_RenewalDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
