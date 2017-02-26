#[allow(non_snake_case)]
#[repr(C)]
#[repr(packed)]
pub struct Descriptor {
    bLength: u8,
    bDescriptorType: u8,
    bcdCCID: u16,
    bMaxSlotIndex: u8,
    bVoltageSupport: u8,
    dwProtocols: u32,
    dwDefaultClock: u32,
    dwMaximumClock: u32,
    bNumClockSupported: u8,
    dwDataRate: u32,
    dwMaxDataRate: u32,
    bNumDataRatesSupported: u8,
    dwMaxIFSD: u32,
    dwSynchProtocols: u32,
    dwMechanical: u32,
    dwFeatures: u32,
    dwMaxCCIDMessageLength: u32,
    bClassGetResponse: u8,
    bClassEnvelope: u8,
    wLcdLayout: u16,
    bPINSupport: u8,
    bMaxCCIDBusySlots: u8,
}
