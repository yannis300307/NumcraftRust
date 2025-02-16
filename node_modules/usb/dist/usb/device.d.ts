/// <reference types="node" />
import * as usb from './bindings';
import { Interface } from './interface';
import { Capability } from './capability';
import { BosDescriptor, ConfigDescriptor } from './descriptors';
export declare class ExtendedDevice {
    /**
     * List of Interface objects for the interfaces of the default configuration of the device.
     */
    interfaces: Interface[] | undefined;
    private _timeout;
    /**
     * Timeout in milliseconds to use for control transfers.
     */
    get timeout(): number;
    set timeout(value: number);
    /**
     * Object with properties for the fields of the active configuration descriptor.
     */
    get configDescriptor(): ConfigDescriptor | undefined;
    /**
     * Contains all config descriptors of the device (same structure as .configDescriptor above)
     */
    get allConfigDescriptors(): ConfigDescriptor[];
    /**
     * Contains the parent of the device, such as a hub. If there is no parent this property is set to `null`.
     */
    get parent(): usb.Device;
    /**
     * Open the device.
     * @param defaultConfig
     */
    open(this: usb.Device, defaultConfig?: boolean): void;
    /**
     * Close the device.
     *
     * The device must be open to use this method.
     */
    close(this: usb.Device): void;
    /**
     * Set the device configuration to something other than the default (0). To use this, first call `.open(false)` (which tells it not to auto configure),
     * then before claiming an interface, call this method.
     *
     * The device must be open to use this method.
     * @param desired
     * @param callback
     */
    setConfiguration(this: usb.Device, desired: number, callback?: (error: usb.LibUSBException | undefined) => void): void;
    /**
     * Perform a control transfer with `libusb_control_transfer`.
     *
     * Parameter `data_or_length` can be an integer length for an IN transfer, or a `Buffer` for an OUT transfer. The type must match the direction specified in the MSB of bmRequestType.
     *
     * The `data` parameter of the callback is actual transferred for OUT transfers, or will be passed a Buffer for IN transfers.
     *
     * The device must be open to use this method.
     * @param bmRequestType
     * @param bRequest
     * @param wValue
     * @param wIndex
     * @param data_or_length
     * @param callback
     */
    controlTransfer(this: usb.Device, bmRequestType: number, bRequest: number, wValue: number, wIndex: number, data_or_length: number | Buffer, callback?: (error: usb.LibUSBException | undefined, buffer: Buffer | number | undefined) => void): usb.Device;
    /**
     * Return the interface with the specified interface number.
     *
     * The device must be open to use this method.
     * @param addr
     */
    interface(this: usb.Device, addr: number): Interface;
    /**
     * Perform a control transfer to retrieve a string descriptor
     *
     * The device must be open to use this method.
     * @param desc_index
     * @param callback
     */
    getStringDescriptor(this: usb.Device, desc_index: number, callback: (error?: usb.LibUSBException, value?: string) => void): void;
    /**
     * Perform a control transfer to retrieve an object with properties for the fields of the Binary Object Store descriptor.
     *
     * The device must be open to use this method.
     * @param callback
     */
    getBosDescriptor(this: usb.Device, callback: (error?: usb.LibUSBException, descriptor?: BosDescriptor) => void): void;
    /**
     * Retrieve a list of Capability objects for the Binary Object Store capabilities of the device.
     *
     * The device must be open to use this method.
     * @param callback
     */
    getCapabilities(this: usb.Device, callback: (error: usb.LibUSBException | undefined, capabilities?: Capability[]) => void): void;
}
