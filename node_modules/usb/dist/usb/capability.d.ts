/// <reference types="node" />
import type { Device } from './bindings';
import { CapabilityDescriptor } from './descriptors';
export declare class Capability {
    protected device: Device;
    protected id: number;
    /** Object with fields from the capability descriptor -- see libusb documentation or USB spec. */
    descriptor: CapabilityDescriptor;
    /** Integer capability type. */
    type: number;
    /** Buffer capability data. */
    data: Buffer;
    constructor(device: Device, id: number);
}
