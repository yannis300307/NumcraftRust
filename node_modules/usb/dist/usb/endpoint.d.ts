/// <reference types="node" />
import { EventEmitter } from 'events';
import { LibUSBException, Transfer, Device } from './bindings';
import { EndpointDescriptor } from './descriptors';
/** Common base for InEndpoint and OutEndpoint. */
export declare abstract class Endpoint extends EventEmitter {
    protected device: Device;
    address: number;
    /** Endpoint direction: `"in"` or `"out"`. */
    abstract direction: 'in' | 'out';
    /** Endpoint type: `usb.LIBUSB_TRANSFER_TYPE_BULK`, `usb.LIBUSB_TRANSFER_TYPE_INTERRUPT`, or `usb.LIBUSB_TRANSFER_TYPE_ISOCHRONOUS`. */
    transferType: number;
    /** Sets the timeout in milliseconds for transfers on this endpoint. The default, `0`, is infinite timeout. */
    timeout: number;
    /** Object with fields from the endpoint descriptor -- see libusb documentation or USB spec. */
    descriptor: EndpointDescriptor;
    constructor(device: Device, descriptor: EndpointDescriptor);
    /** Clear the halt/stall condition for this endpoint. */
    clearHalt(callback: (error: LibUSBException | undefined) => void): void;
    /**
     * Create a new `Transfer` object for this endpoint.
     *
     * The passed callback will be called when the transfer is submitted and finishes. Its arguments are the error (if any), the submitted buffer, and the amount of data actually written (for
     * OUT transfers) or read (for IN transfers).
     *
     * @param timeout Timeout for the transfer (0 means unlimited).
     * @param callback Transfer completion callback.
     */
    makeTransfer(timeout: number, callback: (error: LibUSBException | undefined, buffer: Buffer, actualLength: number) => void): Transfer;
}
/** Endpoints in the IN direction (device->PC) have this type. */
export declare class InEndpoint extends Endpoint {
    /** Endpoint direction. */
    direction: 'in' | 'out';
    protected pollTransfers: Transfer[];
    protected pollTransferSize: number;
    protected pollPending: number;
    pollActive: boolean;
    constructor(device: Device, descriptor: EndpointDescriptor);
    /**
     * Perform a transfer to read data from the endpoint.
     *
     * If length is greater than maxPacketSize, libusb will automatically split the transfer in multiple packets, and you will receive one callback with all data once all packets are complete.
     *
     * `this` in the callback is the InEndpoint object.
     *
     * The device must be open to use this method.
     * @param length
     * @param callback
     */
    transfer(length: number, callback: (error: LibUSBException | undefined, data?: Buffer) => void): InEndpoint;
    /**
     * Start polling the endpoint.
     *
     * The library will keep `nTransfers` transfers of size `transferSize` pending in the kernel at all times to ensure continuous data flow.
     * This is handled by the libusb event thread, so it continues even if the Node v8 thread is busy. The `data` and `error` events are emitted as transfers complete.
     *
     * The device must be open to use this method.
     * @param nTransfers
     * @param transferSize
     */
    startPoll(nTransfers?: number, transferSize?: number, _callback?: (error: LibUSBException | undefined, buffer: Buffer, actualLength: number) => void): Transfer[];
    protected startPollTransfers(nTransfers: number | undefined, transferSize: number | undefined, callback: (error: LibUSBException | undefined, buffer: Buffer, actualLength: number) => void): Transfer[];
    /**
     * Stop polling.
     *
     * Further data may still be received. The `end` event is emitted and the callback is called once all transfers have completed or canceled.
     *
     * The device must be open to use this method.
     * @param callback
     */
    stopPoll(callback?: () => void): void;
}
/** Endpoints in the OUT direction (PC->device) have this type. */
export declare class OutEndpoint extends Endpoint {
    /** Endpoint direction. */
    direction: 'in' | 'out';
    /**
     * Perform a transfer to write `data` to the endpoint.
     *
     * If length is greater than maxPacketSize, libusb will automatically split the transfer in multiple packets, and you will receive one callback once all packets are complete.
     *
     * `this` in the callback is the OutEndpoint object.
     *
     * The device must be open to use this method.
     * @param buffer
     * @param callback
     */
    transfer(buffer: Buffer, callback?: (error: LibUSBException | undefined, actual: number) => void): OutEndpoint;
    transferWithZLP(buffer: Buffer, callback: (error: LibUSBException | undefined) => void): void;
}
