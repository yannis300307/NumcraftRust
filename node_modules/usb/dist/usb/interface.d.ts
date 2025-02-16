import { LibUSBException, Device } from './bindings';
import { InterfaceDescriptor } from './descriptors';
import { Endpoint } from './endpoint';
export declare class Interface {
    protected device: Device;
    protected id: number;
    /** Integer interface number. */
    interfaceNumber: number;
    /** Integer alternate setting number. */
    altSetting: number;
    /** Object with fields from the interface descriptor -- see libusb documentation or USB spec. */
    descriptor: InterfaceDescriptor;
    /** List of endpoints on this interface: InEndpoint and OutEndpoint objects. */
    endpoints: Endpoint[];
    constructor(device: Device, id: number);
    protected refresh(): void;
    /**
     * Claims the interface. This method must be called before using any endpoints of this interface.
     *
     * The device must be open to use this method.
     */
    claim(): void;
    /**
     * Releases the interface and resets the alternate setting. Calls callback when complete.
     *
     * It is an error to release an interface with pending transfers.
     *
     * The device must be open to use this method.
     * @param callback
     */
    release(callback?: (error?: LibUSBException) => void): void;
    /**
     * Releases the interface and resets the alternate setting. Calls callback when complete.
     *
     * It is an error to release an interface with pending transfers. If the optional closeEndpoints
     * parameter is true, any active endpoint streams are stopped (see `Endpoint.stopStream`),
     * and the interface is released after the stream transfers are cancelled. Transfers submitted
     * individually with `Endpoint.transfer` are not affected by this parameter.
     *
     * The device must be open to use this method.
     * @param closeEndpoints
     * @param callback
     */
    release(closeEndpoints?: boolean, callback?: (error?: LibUSBException) => void): void;
    /**
     * Returns `false` if a kernel driver is not active; `true` if active.
     *
     * The device must be open to use this method.
     */
    isKernelDriverActive(): boolean;
    /**
     * Detaches the kernel driver from the interface.
     *
     * The device must be open to use this method.
     */
    detachKernelDriver(): void;
    /**
     * Re-attaches the kernel driver for the interface.
     *
     * The device must be open to use this method.
     */
    attachKernelDriver(): void;
    /**
     * Sets the alternate setting. It updates the `interface.endpoints` array to reflect the endpoints found in the alternate setting.
     *
     * The device must be open to use this method.
     * @param altSetting
     * @param callback
     */
    setAltSetting(altSetting: number, callback?: (error: LibUSBException | undefined) => void): void;
    /**
     * Return the InEndpoint or OutEndpoint with the specified address.
     *
     * The device must be open to use this method.
     * @param addr
     */
    endpoint(addr: number): Endpoint | undefined;
}
