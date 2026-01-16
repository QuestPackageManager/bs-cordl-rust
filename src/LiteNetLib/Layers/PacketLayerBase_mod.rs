#[cfg(feature = "cordl_class_LiteNetLib+Layers+PacketLayerBase")]
#[repr(C)]
#[derive(Debug)]
pub struct PacketLayerBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ExtraPacketSizeForLayer: i32,
}
#[cfg(feature = "cordl_class_LiteNetLib+Layers+PacketLayerBase")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::Layers::PacketLayerBase {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Layers";
    const CLASS_NAME: &'static str = "PacketLayerBase";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
impl std::ops::Deref for crate::LiteNetLib::Layers::PacketLayerBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
impl std::ops::DerefMut for crate::LiteNetLib::Layers::PacketLayerBase {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
impl crate::LiteNetLib::Layers::PacketLayerBase {
    pub fn New(
        extraPacketSizeForLayer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (extraPacketSizeForLayer))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessInboundPacket(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ProcessInboundPacket"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessInboundPacket",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (remoteEndPoint, data, offset, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOutBoundPacket(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ProcessOutBoundPacket"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessOutBoundPacket",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (remoteEndPoint, data, offset, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        extraPacketSizeForLayer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (extraPacketSizeForLayer))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LiteNetLib+Layers+PacketLayerBase")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Layers::PacketLayerBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
